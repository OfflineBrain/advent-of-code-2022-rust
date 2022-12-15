pub fn part1(input: &str) -> String {
    let letters = ('a'..='z').chain('A'..='Z').collect::<Vec<_>>();
    let result = input
        .lines()
        .map(|line| {
            let half = line.len() / 2;

            let left = &line[0..half];
            let right = &line[half..(half * 2)];

            let common = left.chars().find(|c| right.contains(*c)).unwrap();
            let code = letters
                .iter()
                .enumerate()
                .find_map(|(idx, item)| if *item == common { Some(idx + 1) } else { None })
                .unwrap();

            code
        })
        .sum::<usize>();
    result.to_string()
}

pub fn part2(input: &str) -> String {
    let letters = ('a'..='z').chain('A'..='Z').collect::<Vec<_>>();

    let lines = input.lines().collect::<Vec<_>>();
    let lines_step = lines.iter().step_by(3).enumerate();
    let result = lines_step
        .map(|(idx, &line)| {
            let common = line
                .chars()
                .find(|c| lines[(idx * 3) + 1].contains(*c) && lines[(idx * 3) + 2].contains(*c))
                .unwrap();

            let code = letters
                .iter()
                .enumerate()
                .find_map(|(idx, item)| if *item == common { Some(idx + 1) } else { None })
                .unwrap();

            code
        })
        .sum::<usize>();

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, "157");
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, "70");
    }
}
