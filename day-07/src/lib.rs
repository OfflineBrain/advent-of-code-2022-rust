use std::collections::BTreeMap;
use std::ops::Add;

use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{alpha1, digit1, newline};
use nom::multi::separated_list1;
use nom::IResult;

use crate::Files::{Dir, File};
use crate::Move::{Down, Root, Up};

#[derive(Debug)]
enum Action<'a> {
    Ls(Vec<Files<'a>>),
    Cd(Move<'a>),
}

#[derive(Debug)]
enum Move<'a> {
    Root,
    Up,
    Down(&'a str),
}

#[derive(Debug)]
enum Files<'a> {
    File { name: &'a str, size: u32 },
    Dir(&'a str),
}

fn action(input: &str) -> IResult<&str, Vec<Action>> {
    let (input, actions) = separated_list1(newline, alt((ls, cd)))(input)?;

    Ok((input, actions))
}

fn ls(input: &str) -> IResult<&str, Action> {
    let (input, _) = tag("$ ls")(input)?;
    let (input, _) = newline(input)?;
    let (input, files) = files(input)?;

    Ok((input, Action::Ls(files)))
}

fn cd(input: &str) -> IResult<&str, Action> {
    let (input, _) = tag("$ cd ")(input)?;
    let (input, directory) = alt((tag(".."), alpha1, tag("/")))(input)?;

    let op = match directory {
        "/" => Action::Cd(Root),
        ".." => Action::Cd(Up),
        name => Action::Cd(Down(name)),
    };

    Ok((input, op))
}

fn files(input: &str) -> IResult<&str, Vec<Files>> {
    let (input, x) = separated_list1(newline, alt((dir, file)))(input)?;
    Ok((input, x))
}

fn dir(input: &str) -> IResult<&str, Files> {
    let (input, _) = tag("dir ")(input)?;
    let (input, name) = alpha1(input)?;

    Ok((input, Dir(name)))
}

fn file(input: &str) -> IResult<&str, Files> {
    let (input, size) = digit1(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, name) = is_not("\n")(input)?;

    Ok((
        input,
        File {
            size: size.parse::<u32>().unwrap(),
            name,
        },
    ))
}

#[derive(Debug)]
struct FileStruct<'a> {
    size: u32,
    name: &'a str,
}
pub fn part1(input: &str) -> String {
    let cmds = action(input).unwrap().1;

    let mut dirs: BTreeMap<String, Vec<FileStruct>> = BTreeMap::new();
    let mut context: Vec<&str> = Vec::new();

    for command in cmds.iter() {
        match command {
            Action::Ls(files) => {
                let mut key = context
                    .iter()
                    .map(|s| s.to_string().add("/"))
                    .collect::<String>();
                key.pop();
                dirs.entry(key.clone()).or_insert(Vec::new());

                for file in files.iter() {
                    match file {
                        File { size, name } => {
                            dirs.entry(key.clone())
                                .and_modify(|fs| fs.push(FileStruct { size: *size, name }));
                        }
                        Dir(_) => (),
                    }
                }
            }
            Action::Cd(Root) => context.push(""),
            Action::Cd(Up) => {
                context.pop();
            }
            Action::Cd(Down(name)) => {
                context.push(name);
            }
        }
    }

    let mut sizes: BTreeMap<String, u32> = BTreeMap::new();

    for (path, files) in dirs.iter() {
        let directories = path.split("/").collect::<Vec<_>>();
        let size = files
            .iter()
            .map(|FileStruct { size, .. }| size)
            .sum::<u32>();

        for x in 0..directories.len() {
            let mut key = directories[..=x]
                .iter()
                .map(|s| s.to_string().add("/"))
                .collect::<String>();
            key.pop();

            sizes.entry(key).and_modify(|v| *v += size).or_insert(size);
        }
    }

    sizes
        .iter()
        .filter(|(_, &size)| size < 100000)
        .map(|(_, size)| size)
        .sum::<u32>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let cmds = action(input).unwrap().1;

    let mut dirs: BTreeMap<String, Vec<FileStruct>> = BTreeMap::new();
    let mut context: Vec<&str> = Vec::new();

    for command in cmds.iter() {
        match command {
            Action::Ls(files) => {
                let mut key = context
                    .iter()
                    .map(|s| s.to_string().add("/"))
                    .collect::<String>();
                key.pop();
                dirs.entry(key.clone()).or_insert(Vec::new());

                for file in files.iter() {
                    match file {
                        File { size, name } => {
                            dirs.entry(key.clone())
                                .and_modify(|fs| fs.push(FileStruct { size: *size, name }));
                        }
                        Dir(_) => (),
                    }
                }
            }
            Action::Cd(Root) => context.push(""),
            Action::Cd(Up) => {
                context.pop();
            }
            Action::Cd(Down(name)) => {
                context.push(name);
            }
        }
    }

    let mut sizes: BTreeMap<String, u32> = BTreeMap::new();

    for (path, files) in dirs.iter() {
        let directories = path.split("/").collect::<Vec<_>>();
        let size = files
            .iter()
            .map(|FileStruct { size, .. }| size)
            .sum::<u32>();

        for x in 0..directories.len() {
            let mut key = directories[..=x]
                .iter()
                .map(|s| s.to_string().add("/"))
                .collect::<String>();
            key.pop();

            sizes.entry(key).and_modify(|v| *v += size).or_insert(size);
        }
    }

    let total_size = *sizes.get("").unwrap();

    let required_space = 30_000_000 as u32;
    let total_space = 70_000_000 as u32;
    let free_space = total_space - total_size;
    let space_to_free = required_space - free_space;

    sizes
        .iter()
        .filter(|(_, &size)| size >= space_to_free)
        .map(|(_, size)| size)
        .min()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, "95437");
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, "24933642");
    }
}
