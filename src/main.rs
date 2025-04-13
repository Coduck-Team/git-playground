use git2::Repository;
use std::io::{self, BufRead, Write};

fn git_init() -> Result<(), git2::Error> {
    let _repo = Repository::init(".");
    println!("repo init success.");
    Ok(())
}

fn main() -> Result<(), git2::Error> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("명령어 입력 (init, q): ");
        stdout.flush().unwrap();
        let mut input = String::new();
        stdin.lock().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "init" => {
                if let Err(e) = git_init() {
                    println!("init error: {}", e);
                }
            }
            "q" => break,
            _ => println!("존재하지 않는 명령어임"),
        }
    }

    Ok(())
}
