use std::collections::HashSet;

pub fn part1(input: &str) -> String {
    unique_sequence_end_idx(input, 4)
}

pub fn part2(input: &str) -> String {
    unique_sequence_end_idx(input, 14)
}

fn unique_sequence_end_idx(input: &str, len: usize) -> String {
    let (idx, _) = input
        .chars()
        .collect::<Vec<_>>()
        .windows(len)
        .enumerate()
        .find(|(_, window)| window.iter().collect::<HashSet<_>>().len() == len)
        .unwrap();

    (idx + len).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, "7");
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, "19");
    }
}
