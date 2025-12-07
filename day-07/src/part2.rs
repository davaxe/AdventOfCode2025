use crate::part1;
use std::collections::HashMap;

#[must_use]
pub fn task(input: &str) -> Option<String> {
    let map = part1::parse_map(input);
    let start = map.iter().find_map(|(&pos, &c)| {
        if c == part1::Cell::Start {
            Some(pos)
        } else {
            None
        }
    })?;

    let mut particles = HashMap::new();
    let mut count: u64 = 0;
    particles.insert(start, 1);
    while !particles.is_empty() {
        particles = step(particles, &map);
        let count_sum = particles.values().sum();
        count = count.max(count_sum);
    }
    Some(count.to_string())
}

fn step(
    particles: HashMap<(i32, i32), u64>,
    map: &HashMap<(i32, i32), part1::Cell>,
) -> HashMap<(i32, i32), u64> {
    let mut new = HashMap::new();

    for ((x, y), count) in particles {
        let (nx, ny) = (x, y + 1);
        match map.get(&(nx, ny)) {
            Some(part1::Cell::Empty | part1::Cell::Start) => {
                *new.entry((nx, ny)).or_insert(0) += count;
            }
            Some(part1::Cell::Splitter) => {
                *new.entry((nx + 1, ny)).or_insert(0) += count;
                *new.entry((nx - 1, ny)).or_insert(0) += count;
            }
            None => {}
        }
    }

    new
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "40");
    }
}
