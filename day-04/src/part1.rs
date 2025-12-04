use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    NonEmpty,
}

#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
#[must_use]
pub fn task(input: &str) -> Option<String> {
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.char_indices().map(move |(x, c)| match c {
                '@' => ((x as i32, y as i32), Cell::NonEmpty),
                '.' => ((x as i32, y as i32), Cell::Empty),
                _ => unreachable!(),
            })
        })
        .collect::<HashMap<_, _>>();

    let res = map
        .iter()
        .filter(|(_, c)| matches!(c, Cell::NonEmpty))
        .map(|(k, _)| k)
        .filter(|(x, y)| count_adjacent_non_empty(&map, *x, *y) >= 5)
        .count();

    Some(res.to_string())
}

#[must_use]
pub fn count_adjacent_non_empty<S: ::std::hash::BuildHasher>(
    map: &HashMap<(i32, i32), Cell, S>,
    x: i32,
    y: i32,
) -> usize {
    [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ]
    .iter()
    .map(|(dx, dy)| (x + dx, y + dy))
    .filter(|pos| matches!(map.get(pos), Some(Cell::Empty) | None))
    .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "13");
    }
}
