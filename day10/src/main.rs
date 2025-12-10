use anyhow::Result;
use day10::{ButtonPressAction, MachineDescription, MachineState};
use rayon::prelude::*;

fn main() -> Result<()> {
    let arg1 = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("No input file provided"))?;
    let data = common::read_file(&arg1)?;
    let data = day10::parse_data(&data)?;

    println!("Part 1: {:?}", part1(&data)?);
    println!("Part 2: {:?}", part2(&data)?);

    Ok(())
}

fn part1(data: &[MachineDescription]) -> Result<usize> {
    Ok(data
        .iter()
        .map(|desc| {
            find_shortest_path_lights(&desc.desired_state, &desc.actions).expect("Invalid path")
        })
        .sum())
}

fn part2(data: &[MachineDescription]) -> Result<u32> {
    Ok(data
        .iter()
        .map(|desc| {
            let res = find_shortest_path_joltage(&desc.joltage_requirements, &desc.actions)
                .expect("Invalid path");
            println!("Joltage: {:?}, Path: {:?}", desc.joltage_requirements, res);
            res
        })
        .sum::<u32>())
}

fn find_shortest_path_lights(
    desired_state: &MachineState,
    actions: &[ButtonPressAction],
) -> Result<usize> {
    let start_state = MachineState::from_len(desired_state.len());
    let res = pathfinding::directed::dijkstra::dijkstra(
        &start_state,
        |state| {
            let state = state.clone();
            actions.iter().map(move |action| {
                let mut state = state.clone();
                state.apply_action(action).expect("Invalid action");
                (state, 1)
            })
        },
        |state| state == desired_state,
    )
    .ok_or_else(|| anyhow::anyhow!("No path found"))?;
    //    println!("Path: {:?}", res);
    Ok(res.1)
}

fn find_shortest_path_joltage(
    desired_joltage: &[u32],
    actions: &[ButtonPressAction],
) -> Result<u32> {
    let start_joltage = vec![0; desired_joltage.len()];
    let res = pathfinding::directed::dijkstra::dijkstra(
        &start_joltage,
        |joltage| {
            println!("Joltage: {joltage:?}, Desired Joltage: {desired_joltage:?}");
            let take = if joltage
                .iter()
                .enumerate()
                .any(|(i, j)| *j > desired_joltage[i])
            {
                0
            } else {
                actions.len()
            };
            let joltage = joltage.clone();

            actions
                .iter()
                .map(move |action| {
                    let mut new_joltage = joltage.clone();
                    apply_joltage_action(&mut new_joltage, action).expect("Invalid action");
                    (new_joltage, 1)
                })
                .take(take)
        },
        // |joltage| {
        //     return 1;
        //     let distance = joltage
        //         .iter()
        //         .enumerate()
        //         .map(|(i, j)| (*j).abs_diff(desired_joltage[i]))
        //         .sum::<u32>();
        //     distance
        // },
        |joltage| *joltage == desired_joltage,
    )
    .ok_or_else(|| anyhow::anyhow!("No path found"))?;
    Ok(res.1)
}

fn apply_joltage_action<'a>(
    joltage: &'a mut [u32],
    action: &ButtonPressAction,
) -> Result<&'a [u32]> {
    for increment in action.toggles.iter() {
        *joltage
            .get_mut(*increment)
            .ok_or_else(|| anyhow::anyhow!("Invalid increment: {}", increment))? += 1;
    }
    Ok(joltage)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_joltage_action() {
        let mut joltage = vec![0, 0, 0];
        let action = ButtonPressAction::new(vec![0, 1]);
        apply_joltage_action(&mut joltage, &action).unwrap();
        assert_eq!(joltage, vec![1, 1, 0]);
    }
}
