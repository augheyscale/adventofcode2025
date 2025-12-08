use anyhow::{Context, Result};
use itertools::Itertools;

use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct XYZ {
    x: u64,
    y: u64,
    z: u64,
}

impl FromStr for XYZ {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s_iter = s.split(',');
        let x = s_iter
            .next()
            .ok_or_else(|| anyhow::anyhow!("Missing x: {}", s))?
            .parse()
            .context("x")?;
        let y = s_iter
            .next()
            .ok_or_else(|| anyhow::anyhow!("Missing y: {}", s))?
            .parse()
            .context("y")?;
        let z = s_iter
            .next()
            .ok_or_else(|| anyhow::anyhow!("Missing z: {}", s))?
            .parse()
            .context("z")?;
        if s_iter.next().is_some() {
            anyhow::bail!("Extra values: {}", s);
        }
        Ok(XYZ { x, y, z })
    }
}

impl XYZ {
    pub fn sqr_distance(a: &XYZ, b: &XYZ) -> u64 {
        let dx = a.x.abs_diff(b.x).pow(2);
        let dy = a.y.abs_diff(b.y).pow(2);
        let dz = a.z.abs_diff(b.z).pow(2);
        dx + dy + dz
    }
}

pub fn parse_data(data: &str) -> Result<Vec<XYZ>> {
    data.lines().map(XYZ::from_str).collect::<Result<Vec<_>>>()
}

type Circuit<'a> = HashSet<&'a XYZ>;

pub fn initialize_circuits<'a>(
    xyzs: &'a [XYZ],
) -> (
    Vec<Circuit<'a>>,
    HashMap<&'a XYZ, usize>,
    Vec<(&'a XYZ, &'a XYZ)>,
) {
    let circuits = Vec::<Circuit>::new();
    let junction_to_circuit = HashMap::<&XYZ, usize>::new();
    let mut all_pairs = xyzs.iter().tuple_combinations().collect::<Vec<_>>();
    all_pairs.sort_by_key(|pair: &(&XYZ, &XYZ)| XYZ::sqr_distance(pair.0, pair.1));
    (circuits, junction_to_circuit, all_pairs)
}

pub fn part1(xyzs: &[XYZ]) -> Result<usize> {
    let (mut circuits, mut junction_to_circuit, all_pairs) = initialize_circuits(xyzs);

    for (junction0, junction1) in all_pairs.into_iter().take(1000) {
        combine_junctions(
            junction0,
            junction1,
            &mut circuits,
            &mut junction_to_circuit,
        );
    }

    let active_circuits = circuits.iter().filter(|circuit| !circuit.is_empty());
    let mut num_circuits_in_active_circuits = active_circuits
        .map(|circuit| circuit.len())
        .collect::<Vec<_>>();
    num_circuits_in_active_circuits.sort();

    let product = num_circuits_in_active_circuits
        .into_iter()
        .rev()
        .take(3)
        .product::<usize>();

    Ok(product)
}

enum Action {
    DoNothing,
    NewCircuit,
    Add1to0(usize),
    Add0to1(usize),
    CombineCircuits(usize, usize),
}

// Combines two junctions that might already be in circuits into a new circuit.
fn how_to_combine_junctions(circuit1: Option<&usize>, circuit2: Option<&usize>) -> Action {
    match (circuit1, circuit2) {
        (Some(circuit1), Some(circuit2)) => {
            // In the same circuit, do nothing.
            if circuit1 == circuit2 {
                return Action::DoNothing;
            }
            // In different circuits, combine them.
            return Action::CombineCircuits(*circuit1, *circuit2);
        }
        (Some(circuit), None) => {
            // The second doesn't have a circuit, so add it to the first.
            return Action::Add1to0(*circuit);
        }
        (None, Some(circuit)) => {
            // The first doesn't have a circuit, so add it to the second.
            return Action::Add0to1(*circuit);
        }
        (None, None) => {
            // Neither has a circuit, so create a new circuit.
            return Action::NewCircuit;
        }
    }
}

fn combine_junctions<'a>(
    junction0: &'a XYZ,
    junction1: &'a XYZ,
    circuits: &mut Vec<Circuit<'a>>,
    junction_to_circuit: &mut HashMap<&'a XYZ, usize>,
) -> Action {
    let action = how_to_combine_junctions(
        junction_to_circuit.get(junction0),
        junction_to_circuit.get(junction1),
    );
    match action {
        Action::DoNothing => {}
        Action::NewCircuit => {
            // Create a new circuit with the two junctions.
            let circuit = Circuit::from([junction0, junction1]);
            circuits.push(circuit);

            // Setup the index pointers in junction_to_circuit to point to the new circuit.
            let circuit_index = circuits.len() - 1;
            junction_to_circuit.insert(junction0, circuit_index);
            junction_to_circuit.insert(junction1, circuit_index);
        }
        Action::Add1to0(circuit) => {
            circuits
                .get_mut(circuit)
                .expect("circuit")
                .insert(junction1);
            junction_to_circuit.insert(junction1, circuit);
        }
        Action::Add0to1(circuit) => {
            circuits
                .get_mut(circuit)
                .expect("circuit")
                .insert(junction0);
            junction_to_circuit.insert(junction0, circuit);
        }
        Action::CombineCircuits(circuit1_index, circuit2_index) => {
            // We're going to clear circuit2 and add its junctions to circuit1.

            // Take circuit2 from circuits and replace it with an empty set.
            let circuit2 = std::mem::replace(
                circuits.get_mut(circuit2_index).expect("circuit2"),
                HashSet::new(),
            );

            let circuit1 = circuits.get_mut(circuit1_index).expect("circuit1");
            circuit1.extend(circuit2.iter());

            // Change all circuit2 references to circuit1
            for junction in circuit2.into_iter() {
                let prev = junction_to_circuit.insert(junction, circuit1_index);
                assert_eq!(prev, Some(circuit2_index));
            }
        }
    }
    action
}

pub fn part2(xyzs: &[XYZ]) -> Result<u64> {
    let (mut circuits, mut junction_to_circuit, all_pairs) = initialize_circuits(xyzs);

    let mut last_x_coordinates = None;
    for (junction0, junction1) in all_pairs {
        match combine_junctions(
            junction0,
            junction1,
            &mut circuits,
            &mut junction_to_circuit,
        ) {
            Action::DoNothing => {}
            _ => {
                last_x_coordinates = Some((junction0.x, junction1.x));
            }
        }
    }

    let Some(last_x_coordinates) = last_x_coordinates else {
        return Err(anyhow::anyhow!("No last x coordinates found"));
    };
    Ok(last_x_coordinates.1 * last_x_coordinates.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_data() {
        let data = "1,2,3\n4,5,6\n7,8,9";
        let xyzs = parse_data(data).unwrap();
        assert_eq!(xyzs.len(), 3);
    }

    #[test]
    fn test_part1() {
        let data = common::read_file("sample.txt").unwrap();
        let xyzs = parse_data(&data).unwrap();
        // Note: this is for 1000 pairs, not 10 in the sample data.
        assert_eq!(part1(&xyzs).unwrap(), 20);
    }
}
