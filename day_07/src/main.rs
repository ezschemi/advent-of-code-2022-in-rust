use std::{fs, usize};

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

fn main() -> color_eyre::Result<()> {
    let input_filename = String::from("input_small.txt");
    let content = fs::read_to_string(&input_filename).unwrap();
    let lines = content.lines().collect();

    process_commands_and_outputs(&lines);

    Ok(())
}
