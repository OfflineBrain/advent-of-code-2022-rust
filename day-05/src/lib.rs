use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{alpha1, digit1, multispace1, newline, space1};
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, preceded};
use nom::IResult;

pub fn part1(input: &str) -> String {
    let (_, (mut stacks, moves)) = crates(input).unwrap();

    for (amount, from, to) in moves {
        let len = stacks[from as usize].len();
        let drained = stacks[from as usize]
            .drain((len - (amount as usize))..)
            .rev()
            .collect::<Vec<_>>();
        for crt in drained {
            stacks[to as usize].push(crt);
        }
    }

    let top = stacks
        .iter()
        .map(|stack| *stack.last().unwrap_or(&""))
        .collect::<String>();

    top
}

pub fn part2(input: &str) -> String {
    let (_, (mut stacks, moves)) = crates(input).unwrap();

    for (amount, from, to) in moves {
        let len = stacks[from as usize].len();
        let drained = stacks[from as usize]
            .drain((len - (amount as usize))..)
            .collect::<Vec<_>>();
        for crt in drained {
            stacks[to as usize].push(crt);
        }
    }

    let top = stacks
        .iter()
        .map(|stack| *stack.last().unwrap_or(&""))
        .collect::<String>();

    top
}

fn parse_crate(input: &str) -> IResult<&str, Option<&str>> {
    let (input, c) = alt((
        tag("   "),
        delimited(complete::char('['), alpha1, complete::char(']')),
    ))(input)?;

    let result = match c {
        "   " => None,
        value => Some(value),
    };

    Ok((input, result))
}

fn line(input: &str) -> IResult<&str, Vec<Option<&str>>> {
    let (input, result) = separated_list1(tag(" "), parse_crate)(input)?;

    Ok((input, result))
}

fn move_crate(input: &str) -> IResult<&str, (u32, u32, u32)> {
    let (input, _) = tag("move ")(input)?;
    let (input, amount) = complete::u32(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, from) = complete::u32(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, to) = complete::u32(input)?;

    Ok((input, (amount, from - 1, to - 1)))
}

fn crates(input: &str) -> IResult<&str, (Vec<Vec<&str>>, Vec<(u32, u32, u32)>)> {
    let (input, horizontal_crates) = separated_list1(newline, line)(input)?;
    let (input, _) = newline(input)?;
    let (input, _numbers) = many1(preceded(space1, digit1))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, moves) = separated_list1(newline, move_crate)(input)?;

    let mut vertical_crates: Vec<Vec<&str>> = vec![];
    for _ in 0..=horizontal_crates.len() {
        vertical_crates.push(vec![]);
    }

    for row in horizontal_crates.iter().rev() {
        for (id, value) in row.iter().enumerate() {
            if let Some(value) = *value {
                vertical_crates[id].push(value)
            }
        }
    }

    Ok((input, (vertical_crates, moves)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "    [D]
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, "MCD");
    }
}
