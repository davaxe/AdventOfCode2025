#![warn(clippy::pedantic)]
#![allow(clippy::missing_const_for_fn)]
pub mod parse;
pub mod part1;
pub mod part2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum Light {
    On,
    #[default]
    Off,
}

impl Light {
    #[must_use]
    pub fn toggle(&self) -> Self {
        match self {
            Self::On => Self::Off,
            Self::Off => Self::On,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Button {
    pub connections: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Machine {
    pub lights: Vec<Light>,
    pub target: Vec<Light>,
    pub buttons: Vec<Button>,
    pub requirements: Vec<u32>,
}

impl Machine {
    #[must_use]
    pub fn is_satisfied(&self) -> bool {
        self.lights == self.target
    }

    pub fn press_button(&mut self, button_index: usize) -> bool {
        if let Some(button) = self.buttons.get(button_index) {
            for &light_index in &button.connections {
                if let Some(light) = self.lights.get_mut(light_index) {
                    *light = light.toggle();
                }
            }
            return true;
        }
        false
    }

    #[must_use]
    pub fn to_no_requirement_machine(self) -> NoRequirementMachine {
        NoRequirementMachine {
            lights: self.lights,
            target: self.target,
            buttons: self.buttons,
        }
    }

    #[must_use]
    pub fn to_no_lights_machine(self) -> NoLightsMachine {
        NoLightsMachine {
            count: vec![0; self.requirements.len()],
            buttons: self.buttons,
            requirements: self.requirements,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NoRequirementMachine {
    pub lights: Vec<Light>,
    pub target: Vec<Light>,
    pub buttons: Vec<Button>,
}

impl NoRequirementMachine {
    #[must_use]
    pub fn is_satisfied(&self) -> bool {
        self.lights == self.target
    }

    pub fn press_button(&mut self, button_index: usize) -> bool {
        if let Some(button) = self.buttons.get(button_index) {
            for &light_index in &button.connections {
                if let Some(light) = self.lights.get_mut(light_index) {
                    *light = light.toggle();
                }
            }
            return true;
        }
        false
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NoLightsMachine {
    pub buttons: Vec<Button>,
    pub requirements: Vec<u32>,
    pub count: Vec<u32>,
}

impl NoLightsMachine {
    #[must_use]
    pub fn is_satisfied(&self) -> bool {
        self.requirements == self.count
    }

    pub fn press_button(&mut self, button_index: usize) -> bool {
        let mut any_out_of_bounds = false;
        if let Some(button) = self.buttons.get(button_index) {
            for &light_index in &button.connections {
                if let Some(cnt) = self.count.get_mut(light_index) {
                    *cnt += 1;
                    if *cnt > self.requirements[light_index] {
                        any_out_of_bounds = true;
                    }
                }
            }
        }
        !any_out_of_bounds
    }
}
