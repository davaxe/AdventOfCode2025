#![warn(clippy::pedantic)]
#![allow(clippy::missing_const_for_fn)]

use std::fmt::Display;
pub mod parser;
pub mod part1;
pub mod part2;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Block {
    pub occupancy: [[bool; 3]; 3],
    area: u32,
}

impl Block {
    fn new(occupancy: [[bool; 3]; 3]) -> Self {
        Self {
            occupancy,
            area: occupancy
                .iter()
                .flatten()
                .fold(0, |acc, b| acc + u32::from(*b)),
        }
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.occupancy {
            for &cell in row {
                let symbol = if cell { '#' } else { '.' };
                write!(f, "{symbol}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Region {
    width: u32,
    height: u32,
    required: Vec<u32>,
}

impl Region {
    #[must_use]
    pub fn area(&self) -> u32 {
        self.width * self.height
    }
}
