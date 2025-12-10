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
    pub fn toggle(&mut self) {
        *self = match self {
            Light::Off => Light::On,
            Light::On => Light::Off,
        };
    }
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
impl MachineState {
    pub fn new(lights: Vec<Light>) -> Self {
        MachineState { lights }
    }
    pub fn from_len(len: usize) -> Self {
        MachineState {
            lights: vec![Light::Off; len],
        }
    }
}
impl MachineState {
    pub fn apply_action(&mut self, action: &ButtonPressAction) -> Result<&[Light]> {
        for toggle in action.toggles.iter() {
            self.lights
                .get_mut(*toggle)
                .ok_or_else(|| anyhow::anyhow!("Invalid toggle: {}", toggle))?
                .toggle();
        }
        Ok(&self.lights)
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_machine_state_apply_action() {
        // [.##.]
        let mut machine_state = MachineState::from_len(4);

        machine_state
            .apply_action(&ButtonPressAction::new(vec![0, 2]))
            .unwrap();
        machine_state
            .apply_action(&ButtonPressAction::new(vec![0, 1]))
            .unwrap();

        assert_eq!(
            machine_state.lights,
            vec![Light::Off, Light::On, Light::On, Light::Off]
        );
    }
}
