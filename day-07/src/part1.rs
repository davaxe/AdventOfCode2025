use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Start,
    Empty,
    Splitter,
}

impl TryFrom<char> for Cell {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'S' => Ok(Self::Start),
            '.' => Ok(Self::Empty),
            '^' => Ok(Self::Splitter),
            _ => Err(()),
        }
    }
}

#[must_use]
pub fn parse_map(input: &str) -> HashMap<(i32, i32), Cell> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.char_indices().filter_map(move |(x, c)| {
                Some((
                    (i32::try_from(x).ok()?, i32::try_from(y).ok()?),
                    Cell::try_from(c).ok()?,
                ))
            })
        })
        .collect()
}

#[must_use]
pub fn task(input: &str) -> Option<String> {
    let map = parse_map(input);
    let start = map
        .iter()
        .find_map(|(&pos, &c)| if c == Cell::Start { Some(pos) } else { None })?;

    let mut beams = vec![start];
    let mut total_splits = 0;
    while !beams.is_empty() {
        let (splits, new_beams) = step(&beams, &map);
        total_splits += splits;
        beams = new_beams;
    }

    Some(total_splits.to_string())
}

fn step(beams: &Vec<(i32, i32)>, map: &HashMap<(i32, i32), Cell>) -> (u32, Vec<(i32, i32)>) {
    let mut new_beams = HashSet::with_capacity(beams.len() * 2);
    let mut splits = 0;
    for &(x, y) in beams {
        let (nx, ny) = (x, y + 1);
        match map.get(&(nx, ny)) {
            Some(Cell::Empty | Cell::Start) => {
                new_beams.insert((nx, ny));
            }
            Some(Cell::Splitter) => {
                splits += 1;
                new_beams.insert((nx + 1, ny));
                new_beams.insert((nx - 1, ny));
            }
            None => {}
        }
    }

    (splits, new_beams.into_iter().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "21");
    }
}
