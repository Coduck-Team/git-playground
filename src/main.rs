use git_playground::commands;
use std::io::{self, BufRead, Write};

pub fn main() -> Result<(), git2::Error> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("git playground(도움말 help): ");
        stdout.flush().unwrap();
        let mut input = String::new();
        stdin.lock().read_line(&mut input).unwrap();
        let input = input.trim();

        let tokens: Vec<&str> = input.split_whitespace().collect();
        if tokens.is_empty() {
            continue;
        }

        match tokens[0] {
            "help" => {
                commands::git_help();
            }
            "init" => {
                if let Err(e) = commands::git_init() {
                    println!("init error: {}", e);
                }
            }
            "add" => {
                if tokens.len() < 2 {
                    println!("input file path");
                } else {
                    if let Err(e) = commands::git_add(tokens[1]) {
                        println!("add error: {}", e);
                    }
                }
            }
            "commit" => {
                if tokens.len() < 2 {
                    println!("input commit message");
                } else {
                    let commit_msg = tokens[1..].join(" ");
                    if let Err(e) = commands::git_commit(&commit_msg) {
                        println!("commit error: {}", e);
                    }
                }
            }
            "push" => {
                if tokens.len() < 3 {
                    println!("입력 형식: push <remote> <refspec>");
                } else {
                    let remote = tokens[1];
                    let refspec = tokens[2];
                    if let Err(e) = commands::git_push(remote, refspec) {
                        println!("push error: {}", e);
                    }
                }
            }
            "revert" => {
                if tokens.len() < 2 {
                    println!("입력 형식: revert <commit_id>");
                } else {
                    if let Err(e) = commands::git_revert(tokens[1]) {
                        println!("revert error: {}", e);
                    }
                }
            }
            // 왜 log는 vec 반환해서 여기서 출력하는데 이 친구는 그렇게 안함.
            // 뭐가 더 좋을까?
            "branch" => {
                if tokens.len() == 1 {
                    if let Err(e) = commands::git_show_branch() {
                        println!("branch show error: {}", e);
                    }
                } else if tokens.len() == 2 {
                    if let Err(e) = commands::git_create_branch(tokens[1]) {
                        println!("create branch error: {}", e);
                    }
                } else if tokens.len() == 3 && tokens[1] == "-d" {
                    if let Err(e) = commands::git_delete_branch(tokens[2]) {
                        println!("delete branch error: {}", e);
                    }
                }
            }
            "log" => match commands::git_log() {
                Ok(logs) => {
                    println!("커밋 로그:");
                    for msg in logs {
                        println!("{}", msg);
                    }
                }
                Err(e) => println!("log error: {}", e),
            },
            "q" => break,
            _ => println!("존재하지 않는 명령어임"),
        }
    }

    Ok(())
}
