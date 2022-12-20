use std::collections::HashSet;

use ::lending_iterator::prelude::*;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1, newline};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;

fn far(from: (i32, i32), to: (i32, i32)) -> bool {
    let x = from.0 - to.0;
    let y = from.1 - to.1;

    x.abs() > 1 || y.abs() > 1
}

fn diagonal(from: (i32, i32), to: (i32, i32)) -> bool {
    from.0 != to.0 && from.1 != to.1
}

fn moves(input: &str) -> IResult<&str, (&str, u32)> {
    let (input, (mov, count)) = separated_pair(alpha1, tag(" "), digit1)(input)?;

    Ok((input, (mov, count.parse::<u32>().unwrap())))
}

pub fn part1(input: &str) -> String {
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut result: HashSet<(i32, i32)> = HashSet::new();
    result.insert(tail.clone());

    let (_, moves): (&str, Vec<(&str, u32)>) = separated_list1(newline, moves)(input).unwrap();

    for (dir, count) in moves {
        let step = direction(dir);

        for _ in 0..count {
            head.0 += step.0;
            head.1 += step.1;

            if far(tail, head) {
                if diagonal(tail, head) {
                    if step.0 == 0 {
                        tail.0 = head.0;
                    } else {
                        tail.1 = head.1;
                    }
                }
                tail.0 += step.0;
                tail.1 += step.1;

                result.insert(tail.clone());
            }
        }
    }
    result.len().to_string()
}

fn direction(dir: &str) -> (i32, i32) {
    let step = match dir {
        "U" => (1, 0),
        "D" => (-1, 0),
        "L" => (0, -1),
        "R" => (0, 1),
        _ => {
            panic!()
        }
    };
    step
}

pub fn part2(input: &str) -> String {
    let mut rope = [(0, 0); 10];
    let mut result: HashSet<(i32, i32)> = HashSet::new();
    result.insert(*rope.last().unwrap());

    let (_, moves): (&str, Vec<(&str, u32)>) = separated_list1(newline, moves)(input).unwrap();

    for (dir, count) in moves {
        let step = direction(dir);
        for _ in 0..count {
            rope[0].0 += step.0;
            rope[0].1 += step.1;

            let mut rope_windows = rope.windows_mut::<2>();

            while let Some([ref mut head, ref mut tail]) = rope_windows.next() {
                {
                    // println!("{:?}{:?}", head, tail);
                    let x_range = (head.0 - 1)..=(head.0 + 1);
                    let y_range = (head.1 - 1)..=(head.1 + 1);

                    let tail_is_connected = x_range
                        .cartesian_product(y_range)
                        .any(|tuple| tuple == *tail);

                    if !tail_is_connected {
                        // println!("{last_head_move:?}");
                        // move_tail
                        // let mut new_tail = head.clone();
                        if head.0 == tail.0 {
                            if head.1 > tail.1 {
                                tail.1 += 1;
                            } else {
                                tail.1 -= 1;
                            }
                        } else if head.1 == tail.1 {
                            if head.0 > tail.0 {
                                tail.0 += 1;
                            } else {
                                tail.0 -= 1;
                            }
                        } else {
                            // diagonal
                            // let head_cross_positions = [
                            //     (head.0 - 1, head.1),
                            //     (head.0 + 1, head.1),
                            //     (head.0, head.1 - 1),
                            //     (head.0, head.1 + 1),
                            // ];
                            let x_range = (head.0 - 1)..=(head.0 + 1);
                            let y_range = (head.1 - 1)..=(head.1 + 1);

                            let head_3x3 = x_range.cartesian_product(y_range).collect::<Vec<_>>();

                            let x_range = (tail.0 - 1)..=(tail.0 + 1);
                            let y_range = (tail.1 - 1)..=(tail.1 + 1);

                            let maybe_new_tail: Vec<(i32, i32)> = x_range
                                .cartesian_product(y_range)
                                .filter(|tuple| head_3x3.contains(tuple))
                                .collect();
                            match maybe_new_tail.len() {
                                2 => {
                                    let new_head_cross_positions = [
                                        (head.0 - 1, head.1),
                                        (head.0 + 1, head.1),
                                        (head.0, head.1 - 1),
                                        (head.0, head.1 + 1),
                                    ];
                                    let next = maybe_new_tail
                                        .iter()
                                        .find(|tuple| new_head_cross_positions.contains(tuple))
                                        .unwrap();
                                    *tail = *next;
                                }
                                1 => {
                                    *tail = maybe_new_tail[0];
                                }
                                _ => {
                                    panic!("unknown tail length");
                                }
                            };
                            // *tail = new_tail;
                        }
                    }
                }
            }

            result.insert(*rope.last().unwrap());
        }
    }
    result.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const INPUT_2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn part1_works() {
        let result = part1(INPUT_1);
        assert_eq!(result, "13");
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT_2);
        assert_eq!(result, "36");
    }
}
