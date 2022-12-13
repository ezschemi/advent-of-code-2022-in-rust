use id_tree::Node;
use id_tree::Tree;

use camino::Utf8PathBuf;
use id_tree::InsertBehavior;

use nom::{
    branch::alt,
    bytes::complete::tag,
    bytes::complete::take_while1,
    combinator::{all_consuming, map},
    sequence::preceded,
    sequence::separated_pair,
    IResult,
};

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

#[derive(Debug)]
struct FsEntry {
    path: Utf8PathBuf,
    size: u64,
}

fn total_size(tree: &Tree<FsEntry>, node: &Node<FsEntry>) -> color_eyre::Result<u64> {
    let mut total = node.data().size;
    for child in node.children() {
        total += total_size(tree, tree.get(child)?)?;
    }
    Ok(total)
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install().unwrap();

    let lines = include_str!("../input_small.txt")
        .lines()
        .map(|line| all_consuming(parse_input_line)(line).unwrap().1);

    let mut tree = Tree::<FsEntry>::new();

    let root = tree.insert(
        Node::new(FsEntry {
            path: "/".into(),
            size: 0,
        }),
        InsertBehavior::AsRoot,
    )?;

    let mut current_node = root;

    for line in lines {
        println!("line: {:?}", line);

        match line {
            InputLine::Command(cmd) => match cmd {
                Command::Ls => {
                    println!("ls");
                }
                Command::Cd(path) => match path.as_str() {
                    "/" => {
                        // ignore, we are already at the root and
                        // for this exercise there is only one of these
                        // commands at the beginning of the input so
                        // ignoring it works fine.
                    }
                    ".." => {
                        current_node = tree.get(&current_node)?.parent().unwrap().clone();
                    }
                    _ => {
                        let node = Node::new(FsEntry {
                            path: path.clone(),
                            size: 0,
                        });
                        current_node =
                            tree.insert(node, InsertBehavior::UnderNode(&current_node))?;
                    }
                },
            },
            InputLine::Entry(entry) => match entry {
                Entry::Dir(_) => {
                    // ignore, this is being handled when processing the "cd" command
                }
                Entry::File(size, path) => {
                    let node = Node::new(FsEntry { size, path });
                    tree.insert(node, InsertBehavior::UnderNode(&current_node))?;
                }
            },
        }
    }

    let mut s = String::new();
    tree.write_formatted(&mut s)?;
    println!("Tree:\n{s}");

    let sum = tree
        .traverse_pre_order(tree.root_node_id().unwrap())?
        .filter(|node| !node.children().is_empty())
        .map(|node| total_size(&tree, node).unwrap())
        .filter(|&size| size <= 100_000)
        .inspect(|s| {
            dbg!(s);
        })
        .sum::<u64>();
    dbg!(sum);

    let total_space = 70000000_u64;
    let needed_free_space = 30000000_u64;

    let used_space = total_size(&tree, tree.get(tree.root_node_id().unwrap())?)?;
    let free_space = total_space.checked_sub(dbg!(used_space)).unwrap();

    let minimum_space_to_free = needed_free_space.checked_sub(free_space).unwrap();

    let size_to_remove = tree
        .traverse_pre_order(tree.root_node_id().unwrap())?
        .filter(|node| !node.children().is_empty())
        .map(|node| total_size(&tree, node).unwrap())
        .filter(|&size| size >= minimum_space_to_free)
        .inspect(|s| {
            dbg!(s);
        })
        .min();

    dbg!(size_to_remove);

    Ok(())
}
