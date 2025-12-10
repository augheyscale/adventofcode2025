// A nom parser for the machine description.

use nom::{IResult, Parser as _};

use crate::{ButtonPressAction, Light, MachineDescription};

pub fn parse_machine_description(input: &str) -> IResult<&str, MachineDescription> {
    // parse list_description parse_many_button_presses* parse_joltage_requirements
    let (input, (lights, _, actions, _, joltage_requirements)) = (
        parser_lights_description,
        nom::character::complete::space1,
        parse_many_button_presses,
        nom::character::complete::space1,
        parser_joltage_requirements,
    )
        .parse(input)?;

    Ok((
        input,
        MachineDescription::new(lights, actions, joltage_requirements),
    ))
}

fn parse_many_button_presses(input: &str) -> IResult<&str, Vec<ButtonPressAction>> {
    // where each button_press_parser is separated by a space
    // where it needs to continue to parse the button presses while it can until it hits something else
    nom::multi::separated_list0(nom::character::complete::space1, button_press_parser).parse(input)
}

fn button_press_parser(input: &str) -> IResult<&str, ButtonPressAction> {
    // Looks like (1,2,3)
    let (input, toggles) = nom::sequence::delimited(
        nom::character::complete::char('('),
        nom::multi::separated_list0(
            nom::character::complete::char(','),
            nom::character::complete::usize,
        ),
        nom::character::complete::char(')'),
    )
    .parse(input)?;

    Ok((input, ButtonPressAction::new(toggles)))
}

fn parser_lights_description(input: &str) -> IResult<&str, Vec<Light>> {
    // Looks like [.##..]
    nom::sequence::delimited(
        nom::character::complete::char('['),
        // any number of parser_button with no seperator
        nom::multi::many0(button_parser),
        nom::character::complete::char(']'),
    )
    .parse(input)
}

fn parser_joltage_requirements(input: &str) -> IResult<&str, Vec<u32>> {
    // Looks like (1,2,3)
    nom::sequence::delimited(
        nom::character::complete::char('{'),
        nom::multi::separated_list0(
            nom::character::complete::char(','),
            nom::character::complete::u32,
        ),
        nom::character::complete::char('}'),
    )
    .parse(input)
}

fn button_parser(input: &str) -> IResult<&str, Light> {
    // Consume a single character and parse it as a button, and use map_res to convert the error
    nom::combinator::map_res(nom::character::complete::one_of(".#"), Light::from_char).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_light_description_parser() {
        let input = "[.##..]";
        let (remaining, buttons) = parser_lights_description(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(
            buttons,
            vec![Light::Off, Light::On, Light::On, Light::Off, Light::Off]
        );
    }

    #[test]
    fn test_button_press_parser() {
        let input = "(1,2,3)";
        let (remaining, action) = button_press_parser(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(action, ButtonPressAction::new(vec![1, 2, 3]));
    }

    #[test]
    fn test_parse_many_button_presses() {
        let input = "(1,2,3) (4,5,6) (7,8,9) SOMETHINGELSE";
        let (remaining, actions) = parse_many_button_presses(input).unwrap();
        assert_eq!(remaining, " SOMETHINGELSE");
        assert_eq!(
            actions,
            vec![
                ButtonPressAction::new(vec![1, 2, 3]),
                ButtonPressAction::new(vec![4, 5, 6]),
                ButtonPressAction::new(vec![7, 8, 9])
            ]
        );
    }

    #[test]
    fn test_parse_machine_description() {
        let input = "[.##..] (1,2,3) (4,5,6) (7,8,9) {1,2,3}";
        let (remaining, description) = parse_machine_description(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(
            description,
            MachineDescription::new(
                vec![Light::Off, Light::On, Light::On, Light::Off, Light::Off],
                vec![
                    ButtonPressAction::new(vec![1, 2, 3]),
                    ButtonPressAction::new(vec![4, 5, 6]),
                    ButtonPressAction::new(vec![7, 8, 9])
                ],
                vec![1, 2, 3]
            )
        );
    }
}
