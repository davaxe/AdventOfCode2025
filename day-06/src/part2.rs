use std::collections::HashMap;

#[must_use]
pub fn task(input: &str) -> Option<String> {
    let mut values: HashMap<usize, Vec<char>> = HashMap::new();
    let mut start_indices: Vec<(usize, char)> = Vec::new();
    for line in input.lines() {
        if line.starts_with('*') || line.starts_with('+') {
            start_indices = line
                .char_indices()
                .filter_map(|(i, c)| if c == ' ' { None } else { Some((i, c)) })
                .collect();
            // To handle the last when using `windows(2)` later
            start_indices.push((line.len(), ' '));
            break;
        }

        line.char_indices().for_each(|(char_i, c)| {
            values.entry(char_i).or_default().push(c);
        });
    }

    let mut window_iter = start_indices.windows(2);
    let mut sum = 0;
    while let Some(&[(start, c), (end, _)]) = window_iter.next() {
        let range = start..end;
        let values: Vec<u64> = range
            .filter_map(|j| {
                let v: String = values
                    .get(&j)
                    .map(|col| col.iter().filter(|c| *c != &' ').collect())?;
                v.parse().ok()
            })
            .collect();
        sum += match c {
            '+' => values.iter().sum::<u64>(),
            '*' => values.iter().product::<u64>(),
            _ => unreachable!("unknown operator"),
        }
    }

    Some(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "3263827");
    }
}
