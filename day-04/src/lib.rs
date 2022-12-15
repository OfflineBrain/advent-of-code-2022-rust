use std::collections::HashSet;

pub fn part1(input: &str) -> String {
    count_overlaps(input, full_overlap)
}

pub fn part2(input: &str) -> String {
    count_overlaps(input, partial_overlap)
}

fn count_overlaps(input: &str, p: impl Fn(HashSet<u32>, HashSet<u32>) -> bool) -> String {
    let result = input
        .lines()
        .filter(|line| {
            let numbers = line
                .split(&['-', ','])
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<_>>();

            let first = (numbers[0]..=numbers[1]).collect::<HashSet<_>>();
            let second = (numbers[2]..=numbers[3]).collect::<HashSet<_>>();

            let full_overlap = p(first, second);
            full_overlap
        })
        .count();

    result.to_string()
}

fn full_overlap(first: HashSet<u32>, second: HashSet<u32>) -> bool {
    first.iter().all(|i| second.contains(i)) || second.iter().all(|i| first.contains(i))
}

fn partial_overlap(first: HashSet<u32>, second: HashSet<u32>) -> bool {
    first.iter().any(|i| second.contains(i)) || second.iter().any(|i| first.contains(i))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, "2");
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, "4");
    }
}
