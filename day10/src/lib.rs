use anyhow::Result;
pub mod parser;

pub fn parse_data(data: &str) -> Result<Vec<MachineDescription>> {
    data.lines()
        .map(|line| {
            parser::parse_machine_description(line)
                .map_err(|e| anyhow::anyhow!("Invalid input: {}", e))
                .and_then(|(remaining, description)| {
                    if remaining.is_empty() {
                        Ok(description)
                    } else {
                        Err(anyhow::anyhow!(
                            "Trailing data after machine description: {}",
                            remaining
                        ))
                    }
                })
        })
        .collect::<Result<Vec<_>>>()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Light {
    Off,
    On,
}

impl Light {
    fn from_char(c: char) -> Result<Self> {
        match c {
            '.' => Ok(Light::Off),
            '#' => Ok(Light::On),
            _ => Err(anyhow::anyhow!("Invalid button: {}", c)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MachineState {
    lights: Vec<Light>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ButtonPressAction {
    toggles: Vec<usize>,
}
impl ButtonPressAction {
    pub fn new(toggles: Vec<usize>) -> Self {
        ButtonPressAction { toggles }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MachineDescription {
    desired_state: MachineState,
    actions: Vec<ButtonPressAction>,
    joltage_requirements: Vec<u32>,
}
impl MachineDescription {
    pub fn new(
        lights: Vec<Light>,
        actions: Vec<ButtonPressAction>,
        joltage_requirements: Vec<u32>,
    ) -> Self {
        MachineDescription {
            desired_state: MachineState { lights },
            actions,
            joltage_requirements,
        }
    }
}
