// The first section lists the standard present shapes. For convenience, each
// shape starts with its index and a colon; then, the shape is displayed
// visually, where # is part of the shape and . is not.

// The second section lists the regions under the trees. Each line starts with
// the width and length of the region; 12x5 means the region is 12 units wide
// and 5 units long. The rest of the line describes the presents that need to
// fit into that region by listing the quantity of each shape of present;

use anyhow::Result;
use common::grid::Grid;
use nom::{
    IResult, Parser,
    bytes::complete::is_a,
    character::complete::{char, digit1, line_ending, space1},
    combinator::map_res,
    multi::{many1, separated_list0, separated_list1},
    sequence::{separated_pair, terminated},
};

use crate::{Present, Problem, Region};

/// Parses the entire problem from input string
pub fn parse_problem(input: &str) -> Result<Problem> {
    let (_, problem) = parse_problem_internal(input)
        .map_err(|e| anyhow::anyhow!("Failed to parse problem: {}", e))?;
    Ok(problem)
}

fn parse_problem_internal(input: &str) -> IResult<&str, Problem> {
    map_res(
        (
            parse_presents_section,
            many1(line_ending),
            parse_regions_section,
        ),
        |(presents, _, regions)| Problem::try_new(presents, regions),
    )
    .parse(input)
}

fn parse_presents_section(input: &str) -> IResult<&str, Vec<Present>> {
    // Presents are separated by blank lines (two newlines)
    separated_list1((line_ending, line_ending), parse_present).parse(input)
}

fn parse_present(input: &str) -> IResult<&str, Present> {
    map_res(
        (
            terminated(digit1, char(':')),
            line_ending,
            separated_list1(line_ending, parse_grid_line),
        ),
        |(_index, _, grid_lines)| Grid::from_lines(grid_lines).map(|grid| Present::new(grid)),
    )
    .parse(input)
}

fn parse_grid_line(input: &str) -> IResult<&str, &str> {
    is_a(".#").parse(input)
}

fn parse_regions_section(input: &str) -> IResult<&str, Vec<Region>> {
    separated_list0(line_ending, parse_region).parse(input)
}

fn parse_region(input: &str) -> IResult<&str, Region> {
    let (input, (xsize, ysize)) = parse_dimensions.parse(input)?;
    let (input, _) = (char(':'), space1).parse(input)?;
    let (input, present_count) = separated_list0(space1, parse_usize).parse(input)?;

    Ok((
        input,
        Region {
            xsize,
            ysize,
            present_count,
        },
    ))
}

fn parse_dimensions(input: &str) -> IResult<&str, (usize, usize)> {
    separated_pair(parse_usize, char('x'), parse_usize).parse(input)
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>()).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_dimensions() {
        let input = "4x4";
        let (remaining, (width, height)) = parse_dimensions(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(width, 4);
        assert_eq!(height, 4);
    }

    #[test]
    fn test_parse_grid_line() {
        let input = ".##.";
        let (remaining, line) = parse_grid_line(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(line, ".##.");
    }

    #[test]
    fn test_parse_region() {
        let input = "4x4: 0 0 0 0 2 0";
        let (remaining, region) = parse_region(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(region.xsize, 4);
        assert_eq!(region.ysize, 4);
        assert_eq!(region.present_count, vec![0, 0, 0, 0, 2, 0]);
    }

    #[test]
    fn test_parse_present() {
        let input = "0:\n###\n##.\n##.";
        let (remaining, present) = parse_present(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(present.grid.width(), 3);
        assert_eq!(present.grid.height(), 3);
    }

    #[test]
    fn test_parse_sample() {
        let sample = include_str!("../sample.txt");
        let result = parse_problem(sample);
        assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
        let problem = result.unwrap();
        assert_eq!(problem.presents.len(), 6);
        assert_eq!(problem.regions.len(), 3);
    }
}
