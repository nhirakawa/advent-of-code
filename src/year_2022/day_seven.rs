use crate::common::parse::{finish, unsigned_number};
use anyhow::anyhow;
use log::info;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, not_line_ending},
    combinator::{map, value},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult, Parser,
};
use std::collections::HashMap;

pub fn part_one(input: &str) -> anyhow::Result<String> {
    let commands = parse(input);

    let mut filesystem = Filesystem::new();

    filesystem.execute_commands(&commands);

    let mut sum = 0;

    for (_, size) in filesystem.directory_sizes {
        if size <= 100000 {
            sum += size;
        }
    }

    Ok(sum.to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let commands = parse(input);

    let mut filesystem = Filesystem::new();

    filesystem.execute_commands(&commands);

    let total_space = 70000000;
    let necessary_free_space = 30000000;
    let current_free_space = total_space - filesystem.directory_sizes["root"];
    let space_to_free = necessary_free_space - current_free_space;

    let mut potentially_deleted_directory_sizes = vec![];

    for (name, size) in filesystem.directory_sizes {
        info!("{} has size {}", name, size);
        if size >= space_to_free {
            potentially_deleted_directory_sizes.push(size);
        }
    }

    potentially_deleted_directory_sizes
        .iter()
        .min()
        .map(|u| u.to_string())
        .ok_or(anyhow!("No directory to delete"))
}

#[derive(Debug)]
struct Filesystem {
    directory_sizes: HashMap<String, usize>,
    current_path: Vec<String>,
}

impl Filesystem {
    fn new() -> Filesystem {
        let mut file_sizes = HashMap::new();
        file_sizes.insert("root".into(), 0);
        let current_path = vec!["root".into()];

        Filesystem {
            directory_sizes: file_sizes,
            current_path,
        }
    }

    fn cd(&mut self, command: &Directory) {
        match command {
            Directory::Root => {
                self.current_path = vec!["root".into()];
            }
            Directory::Up => {
                self.current_path.pop();
            }
            Directory::Named(path) => {
                self.current_path.push(path.clone());
            }
        }
    }

    fn add_file_size(&mut self, size: usize) {
        for i in 1..=self.current_path.len() {
            let subpath: String = self.current_path[0..i].join("/");
            if let Some(sum) = self.directory_sizes.get_mut(&subpath) {
                *sum += size;
            } else {
                self.directory_sizes.insert(subpath, size);
            }
        }
    }

    fn execute_commands(&mut self, commands: &[Command]) {
        for command in commands {
            match command {
                Command::Cd(directory) => self.cd(directory),
                Command::Ls(directory_or_file_list) => {
                    for directory_or_file in directory_or_file_list {
                        if let DirectoryOrFile::File(_name, size) = directory_or_file {
                            self.add_file_size(*size);
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum DirectoryOrFile {
    Directory(String),
    File(String, usize),
}

/*
 * Command::Cd contains the input directory
 * Command::Ls contains the output list (files and directories)
 */
#[derive(Debug, PartialEq, Eq, Clone)]
enum Command {
    Cd(Directory),
    Ls(Vec<DirectoryOrFile>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Directory {
    Root,
    Up,
    Named(String),
}

fn parse(i: &str) -> Vec<Command> {
    finish(commands, i).unwrap()
}

fn commands(i: &str) -> IResult<&str, Vec<Command>> {
    separated_list1(tag("\n"), command).parse(i)
}

fn command(i: &str) -> IResult<&str, Command> {
    preceded(tag("$ "), alt((cd, ls))).parse(i)
}

fn cd(i: &str) -> IResult<&str, Command> {
    map(preceded(tag("cd "), directory), Command::Cd).parse(i)
}

fn directory(i: &str) -> IResult<&str, Directory> {
    alt((root_directory, named_directory, up_directory)).parse(i)
}

fn root_directory(i: &str) -> IResult<&str, Directory> {
    value(Directory::Root, tag("/")).parse(i)
}

fn named_directory(i: &str) -> IResult<&str, Directory> {
    map(alpha1, |s: &str| Directory::Named(s.into())).parse(i)
}

fn up_directory(i: &str) -> IResult<&str, Directory> {
    value(Directory::Up, tag("..")).parse(i)
}

fn ls(i: &str) -> IResult<&str, Command> {
    map(
        preceded(tag("ls\n"), separated_list1(tag("\n"), directory_or_file)),
        Command::Ls,
    )
    .parse(i)
}

fn directory_or_file(i: &str) -> IResult<&str, DirectoryOrFile> {
    alt((output_directory, output_file)).parse(i)
}

fn output_directory(i: &str) -> IResult<&str, DirectoryOrFile> {
    map(preceded(tag("dir "), alpha1), |s: &str| {
        DirectoryOrFile::Directory(s.into())
    })
    .parse(i)
}

fn output_file(i: &str) -> IResult<&str, DirectoryOrFile> {
    map(
        separated_pair(unsigned_number, tag(" "), filename),
        |(size, filename)| DirectoryOrFile::File(filename, size),
    )
    .parse(i)
}

fn filename(i: &str) -> IResult<&str, String> {
    map(not_line_ending, |s: &str| s.into()).parse(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filesystem_cd() {
        let mut filesystem = Filesystem::new();

        assert_eq!(filesystem.current_path, vec!["root"]);

        let directory = Directory::Named("a".into());

        filesystem.cd(&directory);

        assert_eq!(filesystem.current_path, vec!["root", "a"]);

        let directory = Directory::Named("b".into());

        filesystem.cd(&directory);

        assert_eq!(filesystem.current_path, vec!["root", "a", "b"]);

        let directory = Directory::Up;

        filesystem.cd(&directory);

        assert_eq!(filesystem.current_path, vec!["root", "a"]);

        let directory = Directory::Root;

        filesystem.cd(&directory);

        assert_eq!(filesystem.current_path, vec!["root"]);
    }

    #[test]
    fn test_cd() {
        assert_eq!(command("$ cd /"), Ok(("", Command::Cd(Directory::Root))));
        assert_eq!(
            command("$ cd a"),
            Ok(("", Command::Cd(Directory::Named("a".into()))))
        );
        assert_eq!(command("$ cd .."), Ok(("", Command::Cd(Directory::Up))));
    }

    #[test]
    fn test_ls() {
        let input = "$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d";
        let expected_output = vec![
            DirectoryOrFile::Directory("a".into()),
            DirectoryOrFile::File("b.txt".into(), 14848514),
            DirectoryOrFile::File("c.dat".into(), 8504156),
            DirectoryOrFile::Directory("d".into()),
        ];
        assert_eq!(command(input), Ok(("", Command::Ls(expected_output))));
    }
}
