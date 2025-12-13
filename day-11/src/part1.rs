use std::{collections::HashMap, hash::BuildHasher};

#[derive(Debug)]
pub struct Device<'a> {
    pub name: &'a str,
    pub connections: Vec<&'a str>,
}

impl Device<'_> {
    #[must_use]
    pub fn new<'a>(name: &'a str, connections: Vec<&'a str>) -> Device<'a> {
        Device { name, connections }
    }
}

#[must_use]
pub fn task(input: &str) -> Option<String> {
    let mut devices: HashMap<&str, Device<'_>> = input
        .lines()
        .filter_map(|line| {
            let (name, connections) = line.split_once(':')?;

            Some((
                name.trim(),
                Device::new(
                    name.trim(),
                    connections.split_whitespace().map(str::trim).collect(),
                ),
            ))
        })
        .collect();

    devices.insert("out", Device::new("out", Vec::new()));
    Some(paths_between_devices(&devices, "you", "out").to_string())
}

pub fn paths_between_devices<S: BuildHasher>(
    devices: &HashMap<&str, Device<'_>, S>,
    start: &str,
    end: &str,
) -> usize {
    let mut memo = HashMap::new();
    count_paths(start, end, devices, &mut memo)
}

fn count_paths<'a, S: BuildHasher>(
    current: &'a str,
    target: &'a str,
    devices: &HashMap<&str, Device<'a>, S>,
    memo: &mut HashMap<&'a str, usize>,
) -> usize {
    // Base case: If we reached the target, we found 1 valid path
    if current == target {
        return 1;
    }

    // Check cache: If we already computed paths from this node, return it
    if let Some(&count) = memo.get(current) {
        return count;
    }

    let mut total_paths = 0;

    // Traverse neighbors
    if let Some(device) = devices.get(current) {
        for connection in &device.connections {
            total_paths += count_paths(connection, target, devices, memo);
        }
    }

    memo.insert(current, total_paths);
    total_paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../example_part1.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "5");
    }
}
