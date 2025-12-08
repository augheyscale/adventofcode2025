use anyhow::{Context, Result};
use itertools::Itertools;

use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

/// Represents a 3D coordinate point with x, y, and z components.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct XYZ {
    x: u64,
    y: u64,
    z: u64,
}

/// Parses an XYZ coordinate from a string in the format "x,y,z".
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
    /// Calculates the squared Euclidean distance between two points.
    pub fn sqr_distance(a: &XYZ, b: &XYZ) -> u64 {
        let dx = a.x.abs_diff(b.x).pow(2);
        let dy = a.y.abs_diff(b.y).pow(2);
        let dz = a.z.abs_diff(b.z).pow(2);
        dx + dy + dz
    }
}

/// A vector that can only be appended to, not modified.
///
/// This append-only behavior is critical for compile-time correctness of algorithms
/// that use position indicies of the vector.  Position indicies can not
/// be invalidated by modifications to the vector.
#[derive(Default)]
struct AppendOnlyVec<T> {
    inner: Vec<T>,
}
impl<T> AppendOnlyVec<T> {
    pub fn push(&mut self, value: T) {
        self.inner.push(value);
    }
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.inner.iter()
    }
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.inner.get_mut(index)
    }
}

/// Parses input data into a vector of XYZ coordinates, one per line.
pub fn parse_data(data: &str) -> Result<Vec<XYZ>> {
    data.lines().map(XYZ::from_str).collect::<Result<Vec<_>>>()
}

#[derive(Default)]
struct CircuitManager<'a> {
    circuits: AppendOnlyVec<Circuit<'a>>,
    junction_to_circuit: HashMap<Junction<'a>, usize>,
}
impl<'a> CircuitManager<'a> {
    pub fn circuits(&self) -> impl Iterator<Item = &Circuit<'a>> {
        self.circuits.iter()
    }
    pub fn active_circuits(&self) -> impl Iterator<Item = &Circuit<'a>> {
        self.circuits().filter(|circuit| !circuit.is_empty())
    }
}

type Junction<'a> = &'a XYZ;

/// A circuit is a set of connected junctions (XYZ points).
type Circuit<'a> = HashSet<&'a XYZ>;

/// Initializes the data structures needed for circuit processing: an empty circuits vector,
/// a mapping from junctions to circuit indices, and all pairs of possible junctions sorted by distance.
fn initialize_circuits<'a>(xyzs: &'a [XYZ]) -> (CircuitManager<'a>, Vec<(&'a XYZ, &'a XYZ)>) {
    // Get all pairs of junctions and sort them by distance.
    let mut all_pairs = xyzs.iter().tuple_combinations().collect::<Vec<_>>();
    all_pairs.sort_by_key(|pair: &(&XYZ, &XYZ)| XYZ::sqr_distance(pair.0, pair.1));

    (Default::default(), all_pairs)
}

/// Processes the first 1000 closest junction pairs to form circuits, then returns the product
/// of the sizes of the three largest circuits.
pub fn part1(xyzs: &[XYZ]) -> Result<usize> {
    let (mut circuits_manager, all_pairs) = initialize_circuits(xyzs);

    for (junction0, junction1) in all_pairs.into_iter().take(1000) {
        circuits_manager.combine_junctions(junction0, junction1);
    }

    // Map the circuits to how many junctions are in each circuit.
    let mut num_circuits_in_active_circuits = circuits_manager
        .active_circuits()
        .map(|circuit| circuit.len())
        .collect::<Vec<_>>();
    // Sort the circuits by size.
    num_circuits_in_active_circuits.sort();

    // Take the three largest circuits and return the product of their sizes.
    let product = num_circuits_in_active_circuits
        .into_iter()
        .rev()
        .take(3)
        .product::<usize>();

    Ok(product)
}

/// Represents the action to take when combining two junctions into circuits.
enum Action {
    // Circuits are the same, so do nothing.
    DoNothing,
    // New circuit, so create a new circuit with the two junctions.
    NewCircuit,
    // Add the second junction to the first circuit.
    Add1to0(usize),
    // Add the first junction to the second circuit.
    Add0to1(usize),
    // Combine the two circuits.
    CombineCircuits(usize, usize),
}

/// Determines what action should be taken when combining two junctions based on whether
/// they already belong to circuits.
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

/// Combines two junctions into circuits according to the determined action, updating
/// the circuits vector and junction-to-circuit mapping accordingly.
impl<'a> CircuitManager<'a> {
    fn combine_junctions(&mut self, junction0: &'a XYZ, junction1: &'a XYZ) -> Action {
        let circuits = &mut self.circuits;
        let junction_to_circuit = &mut self.junction_to_circuit;

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
}

/// Processes all junction pairs in order of distance, forming circuits. Returns the product
/// of the x coordinates of the last pair that resulted in a circuit combination.
pub fn part2(xyzs: &[XYZ]) -> Result<u64> {
    let (mut circuits_manager, all_pairs) = initialize_circuits(xyzs);

    let mut last_x_coordinates = None;
    for (junction0, junction1) in all_pairs {
        match circuits_manager.combine_junctions(junction0, junction1) {
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
