use crate::part1;
use std::collections::HashMap;

#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
#[must_use]
pub fn task(input: &str) -> Option<String> {
    let mut map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.char_indices().map(move |(x, c)| match c {
                '@' => ((x as i32, y as i32), part1::Cell::NonEmpty),
                '.' => ((x as i32, y as i32), part1::Cell::Empty),
                _ => unreachable!(),
            })
        })
        .collect::<HashMap<_, _>>();

    let mut res = 0;
    loop {
        let changed = step(&mut map);
        if changed == 0 {
            break;
        }
        res += changed;
    }

    Some(res.to_string())
}

fn step(map: &mut HashMap<(i32, i32), part1::Cell>) -> usize {
    let updates: Vec<(i32, i32)> = map
        .iter()
        .filter_map(|(pos, c)| (c == &part1::Cell::NonEmpty).then_some(*pos))
        .filter(|(x, y)| part1::count_adjacent_non_empty(map, *x, *y) >= 5)
        .collect();

    for pos in &updates {
        map.insert(*pos, part1::Cell::Empty);
    }

    updates.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "43");
    }
}
