pub fn part1(input: &str) -> String {
    let result = input
        .split("\n\n")
        .map(|load| {
            println!("{:}", load);
            load.lines()
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap();

    result.to_string()
}

pub fn part2(input: &str) -> String {
    let mut ranking = input
        .split("\n\n")
        .map(|load| {
            load.lines()
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<_>>();

    ranking.sort_by(|left, right| right.cmp(left));

    let result = ranking.iter().take(3).sum::<u32>();

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, "24000");
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, "45000");
    }
}
