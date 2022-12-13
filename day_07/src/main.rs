use std::collections::HashMap;
use std::{fs, usize};

use camino::Utf8PathBuf;
use nom::{
    branch::alt,
    bytes::complete::tag,
    bytes::complete::take_while1,
    combinator::{all_consuming, map},
    sequence::preceded,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug)]
enum FsEntryType {
    File,
    Directory,
}
#[derive(Debug)]
struct FSEntryInfo {
    name: String,
    size: usize,
    fs_type: FsEntryType,
}

impl FSEntryInfo {
    pub fn new(name: String, size: usize, fs_type: FsEntryType) -> Self {
        FSEntryInfo {
            name,
            size,
            fs_type,
        }
    }
}

#[derive(Debug)]
struct FsInfo {
    info: Vec<FSEntryInfo>,
}

impl FsInfo {
    pub fn new() -> Self {
        FsInfo { info: Vec::new() }
    }

    pub fn add_dir(self: &mut Self, name: &str) {
        self.info.push(FSEntryInfo {
            name: name.to_string(),
            size: 0,
            fs_type: FsEntryType::Directory,
        });
    }
}
fn process_commands_and_outputs(lines: &Vec<&str>) {
    println!("Processing {} lines.", lines.len());

    let mut fs_info = FsInfo::new();

    for line in lines {
        if line.starts_with("$") {
            // command
            let tokens = line.split_whitespace().collect::<Vec<&str>>();
            let cmd = tokens[1];

            match cmd {
                "cd" => {
                    let target_dir = tokens[2];
                    println!("cd into {}", target_dir);
                    if target_dir == ".." {
                    } else {
                        fs_info.add_dir(target_dir);
                    }
                }
                "ls" => {
                    println!("ls");
                }
                _ => {
                    panic!("Unsupported command: {} from:\n{}", cmd, line);
                }
            }
        } else {
            // output of the previous command
        }
    }

    println!("Fs Tree:\n{:#?}", fs_info);
}

fn parse_path(i: &str) -> IResult<&str, Utf8PathBuf> {
    map(
        take_while1(|c: char| "abcdefghijklmnopqrstuvwxyz./".contains(c)),
        Into::into,
    )(i)
}

#[derive(Debug)]
struct Ls;

fn parse_ls(i: &str) -> IResult<&str, Ls> {
    map(tag("ls"), |_| Ls)(i)
}

#[derive(Debug)]
struct Cd(Utf8PathBuf);

fn parse_cd(i: &str) -> IResult<&str, Cd> {
    map(preceded(tag("cd "), parse_path), Cd)(i)
}

#[derive(Debug)]
enum Command {
    Ls,
    Cd(Utf8PathBuf),
}

impl From<Ls> for Command {
    fn from(_ls: Ls) -> Self {
        Command::Ls
    }
}

impl From<Cd> for Command {
    fn from(cd: Cd) -> Self {
        Command::Cd(cd.0)
    }
}

fn parse_command(i: &str) -> IResult<&str, Command> {
    let (i, _) = tag("$ ")(i)?;

    alt((map(parse_ls, Into::into), map(parse_cd, Into::into)))(i)
}

#[derive(Debug)]
enum Entry {
    Dir(Utf8PathBuf),
    File(u64, Utf8PathBuf),
}

fn parse_entry(i: &str) -> IResult<&str, Entry> {
    let parse_file = map(
        separated_pair(nom::character::complete::u64, tag(" "), parse_path),
        |(size, path)| Entry::File(size, path),
    );

    let parse_dir = map(preceded(tag("dir "), parse_path), Entry::Dir);

    alt((parse_file, parse_dir))(i)
}

#[derive(Debug)]
enum InputLine {
    Command(Command),
    Entry(Entry),
}

fn parse_input_line(i: &str) -> IResult<&str, InputLine> {
    alt((
        map(parse_command, InputLine::Command),
        map(parse_entry, InputLine::Entry),
    ))(i)
}

#[derive(Debug, Default)]
struct TreeNode {
    size: usize,
    children: HashMap<Utf8PathBuf, TreeNode>,
}
fn main() -> color_eyre::Result<()> {
    let lines = include_str!("../input_small.txt")
        .lines()
        .map(|line| all_consuming(parse_input_line)(line).unwrap().1);

    let mut root = TreeNode::default();
    let mut node = &mut root;

    for line in lines {
        println!("{line:?}");

        match line {
            InputLine::Command(cmd) => match cmd {
                Command::Ls => {
                    println!("ls");
                }
                Command::Cd(path) => match path.as_str() {
                    "/" => {
                        // ignore this command, we are already at the FS root.
                        // works for this puzzle *only* as the "cd /"-command
                        // only appears once at the beginning of the input.
                    }
                    ".." => {
                        todo!("go to the parent.");
                    }
                    _ => {
                        node = node.children.entry(path).or_default();
                        todo!("handle these other paths.");
                    }
                },
            },
            InputLine::Entry(entry) => match entry {
                Entry::Dir(dir) => {
                    node.children.entry(dir).or_default();
                }
                Entry::File(size, path) => {
                    node.children.entry(path).or_default().size = size as usize;
                }
            },
        }
    }

    println!("{node:?}");

    Ok(())
}
