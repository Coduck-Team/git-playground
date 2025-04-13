use git2::{IndexAddOption, Repository};
use std::io::{self, BufRead, Write};
use std::path::Path;

fn git_init() -> Result<(), git2::Error> {
    let _repo = Repository::init(".");
    println!("repo init success.");
    Ok(())
}

fn git_add(path_str: &str) -> Result<(), git2::Error> {
    let repo = Repository::open(".")?;
    let mut idx = repo.index()?;

    if path_str == "." {
        idx.add_all(["."].iter(), IndexAddOption::DEFAULT, None)?;
        println!("all added.");
    } else {
        let path = Path::new(path_str);
        idx.add_path(&path)?;
        println!("{} added.", path_str);
    }
    idx.write()?;
    Ok(())
}

fn main() -> Result<(), git2::Error> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("명령어 입력 (init, add <path>, q): ");
        stdout.flush().unwrap();
        let mut input = String::new();
        stdin.lock().read_line(&mut input).unwrap();
        let input = input.trim();

        let tokens: Vec<&str> = input.split_whitespace().collect();
        if tokens.is_empty() {
            continue;
        }

        match tokens[0] {
            "init" => {
                if let Err(e) = git_init() {
                    println!("init error: {}", e);
                }
            }
            "add" => {
                if tokens.len() < 2 {
                    println!("input file path");
                } else {
                    if let Err(e) = git_add(tokens[1]) {
                        println!("add error: {}", e);
                    }
                }
            }
            "q" => break,
            _ => println!("존재하지 않는 명령어임"),
        }
    }

    Ok(())
}
