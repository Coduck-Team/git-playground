use git2::{IndexAddOption, Repository};
use std::io::{self, BufRead, Write};
use std::path::Path;

pub fn git_init() -> Result<(), git2::Error> {
    let _repo = Repository::init(".")?;
    println!("repo init success.");
    Ok(())
}

pub fn git_add(path_str: &str) -> Result<(), git2::Error> {
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

fn git_commit(message: &str) -> Result<(), git2::Error> {
    let repo = Repository::open(".")?;
    let mut idx = repo.index()?;

    let tree_id = idx.write_tree()?;
    let tree = repo.find_tree(tree_id)?;
    let sig = repo.signature()?;

    let parent_commits = match repo.head() {
        Ok(head_ref) => {
            let head = head_ref
                .target()
                .ok_or_else(|| git2::Error::from_str("HEAD refers to non-HEAD"))?;
            vec![repo.find_commit(head)?]
        }
        Err(_) => Vec::new(),
    };

    let parents: Vec<&git2::Commit> = parent_commits.iter().collect();

    let commit_oid = repo.commit(Some("HEAD"), &sig, &sig, message, &tree, &parents)?;
    println!("commit created: {}", commit_oid);
    Ok(())
}

pub fn main() -> Result<(), git2::Error> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("명령어 입력 (init, add <path>, commit <msg>, q): ");
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
            "commit" => {
                if tokens.len() < 2 {
                    println!("input commit message");
                } else {
                    let commit_msg = tokens[1..].join(" ");
                    if let Err(e) = git_commit(&commit_msg) {
                        println!("commit error: {}", e);
                    }
                }
            }
            "q" => break,
            _ => println!("존재하지 않는 명령어임"),
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use once_cell::sync::Lazy;
    use serial_test::serial;
    use std::fs::File;
    use tempfile::TempDir;
    use uuid::Uuid;

    // 전역 공유 임시 디렉토리와 repo 초기화
    static SHARED_REPO: Lazy<TempDir> = Lazy::new(|| {
        let tmp_dir = TempDir::new().expect("failed to create temporary directory");
        std::env::set_current_dir(tmp_dir.path()).expect("failed to set temporary directory");
        git_init().expect("failed to git init");
        tmp_dir
    });

    fn get_repo() -> Repository {
        let _ = &*SHARED_REPO;
        Repository::open(".").expect("failed to open repo")
    }

    #[test]
    #[serial]
    fn test_git_init() {
        assert!(Path::new(".git").exists());
    }

    #[test]
    #[serial]
    fn test_git_add_specific_file() {
        let repo = get_repo();

        let file_name = generate_random_file_name(".txt");

        let file_path = Path::new(file_name.as_str());
        File::create(file_path).expect("failed to create temp file");

        git_add(file_name.as_str()).expect("failed to add file");

        let index = repo.index().expect("failed to get the index");
        let entries: Vec<_> = index
            .iter()
            .filter(|entry| std::str::from_utf8(&entry.path).unwrap() == file_name.as_str())
            .collect();
        assert_eq!(entries.len(), 1);
    }

    #[test]
    #[serial]
    fn test_git_add_all_files() {
        let repo = get_repo();
        let file_name = generate_random_file_name(".txt");

        let file_path = Path::new(file_name.as_str());
        File::create(file_path).expect("failed to create temp file");

        git_add(".").expect("failed to add file");

        let index = repo.index().unwrap();
        let entries: Vec<_> = index
            .iter()
            .filter(|entry| std::str::from_utf8(&entry.path).unwrap() == file_name.as_str())
            .collect();
        assert_eq!(entries.len(), 1);
    }

    #[test]
    #[serial]
    fn test_git_commit() {
        let repo = get_repo();

        let file_name = generate_random_file_name(".txt");
        let file_path = Path::new(file_name.as_str());
        File::create(file_path).expect("failed to create temp file");
        git_add(file_name.as_str()).expect("failed to add file");

        let commit_msg = "test commit msg";
        git_commit(commit_msg).expect("failed to commit message");

        let head_commit = {
            let head = repo.head().expect("failed to get HEAD");
            let commit_oid = head.target().expect("HEAD refers to non-HEAD");
            repo.find_commit(commit_oid).expect("failed to find commit")
        };

        assert_eq!(
            head_commit.message().unwrap(),
            commit_msg,
            "커밋 메시지가 다름"
        );
    }

    fn generate_random_file_name(suffix: &str) -> String {
        format!("{}{}", Uuid::new_v4(), suffix)
    }
}
