#![deny(unreachable_patterns)]

use std::os::unix::prelude::OsStrExt;
use std::{collections::HashMap, path::PathBuf};

use std::iter::Peekable;

use anyhow::{anyhow, bail, Context, Result};

enum DirItem<'a> {
    Dir(&'a str),
    File { name: &'a str, size: usize },
}

pub struct Dirs<'a>(HashMap<Box<str>, Box<[DirItem<'a>]>>);

fn expect_next<'a, W>(words: &'_ mut W) -> Result<&'a str>
where
    W: Iterator<Item = &'a str>,
{
    match words.next() {
        Some(x) => Ok(x),
        None => Err(anyhow!("Unexpected end of input stream")),
    }
}

fn parse_ls<'a, W>(words: &'_ mut Peekable<W>) -> Result<Box<[DirItem<'a>]>>
where
    W: Iterator<Item = &'a str>,
{
    let mut items = Vec::new();

    loop {
        let Some(&next_up) = words.peek() else {
            return Ok(items.into_boxed_slice());
        };

        let item = match next_up {
            "dir" => {
                words.next();
                let dname =
                    expect_next(words).context("Expected a directory name after 'dir' keyword")?;
                DirItem::Dir(dname)
            }
            x if x.chars().all(|x| matches!(x, '0'..='9')) => {
                let size: usize = x.parse().context("Failed to parse file size")?;
                words.next();
                let name = expect_next(words).context("Expected a file name after a file size")?;
                DirItem::File { name, size }
            }

            // On a failed parse, we return to the parent
            // function to see if it can parse successfully
            _ => return Ok(items.into_boxed_slice()),
        };
        items.push(item);
    }
}

fn parse<'a, W>(words: W) -> Result<HashMap<Box<str>, Box<[DirItem<'a>]>>>
where
    W: Iterator<Item = &'a str>,
{
    let mut words = words.peekable();
    let mut path = PathBuf::new();
    let mut dirs: HashMap<Box<str>, Box<[DirItem]>> = HashMap::new();

    loop {
        match words.next() {
            Some("$") => {}
            Some(x) => bail!(
                "Expected a command to start with '$', found '{}' instead",
                x
            ),
            // This is an acceptable point in a parse
            // to reach an EOF
            None => return Ok(dirs),
        }

        let command =
            expect_next(&mut words).context("Expected to find a command following a '$'")?;
        match command {
            "cd" => {
                let dir = expect_next(&mut words).context("Expected a dir after cd command")?;
                match dir {
                    ".." => assert!(path.pop()),
                    x => path.push(x),
                };
            }

            "ls" => {
                let path: Box<str> = path.to_str().unwrap().into();
                if dirs.contains_key(&path) {
                    bail!("Directory '{}' has already been processed", path);
                }

                let items = parse_ls(&mut words).context("Failed to parse 'ls' command")?;
                dirs.insert(path, items);
            }
            x => bail!("Unexpected command {}, valid commands as 'cd', and 'ls'", x),
        };
    }
}

fn part1(input: &str) -> Result<()> {
    let words = input.split(&[' ', '\n']).filter(|x| !x.is_empty());
    let dirs = parse(words).context("Failed to complete part 1 of challenge")?;
    print_tree(&dirs, "/")?;

    Ok(())
}

fn print_tree(tree: &HashMap<Box<str>, Box<[DirItem]>>, item: &str) -> Result<()> {
    let children = tree
        .get(item)
        .ok_or(anyhow!("Key '{}', doesn't exist", item))?;

    println!("Directory {}", item);

    for child in children.iter() {
        if let DirItem::File { name, .. } = child {
            println!("- {}", name);
        }
    }

    for child in children.iter() {
        if let DirItem::Dir(name) = child {
            let mut new_path = PathBuf::from(item);
            new_path.push(name);
            // SAFETY: We know all parts of PathBuf are UTF-8
            let new_path =
                unsafe { std::str::from_utf8_unchecked(new_path.as_os_str().as_bytes()) };
            print_tree(tree, new_path)?;
        }
    }

    Ok(())
}

fn main() {
    let input = include_str!("input.txt");
    let Err(err) = part1(input) else { return; };
    println!("error: {}", err);

    if err.chain().skip(1).count() > 0 {
        println!("caused by:");
        for cause in err.chain().skip(1) {
            println!("- {cause}");
        }
    }
}
