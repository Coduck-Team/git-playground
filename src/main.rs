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

pub fn git_log() -> Result<Vec<String>, git2::Error> {
    let repo = Repository::open(".")?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    let mut res = Vec::new();
    for commit_id in revwalk {
        let commit = repo.find_commit(commit_id?)?;
        if let Some(msg) = commit.message() {
            res.push(msg.to_string());
        }
    }
    Ok(res)
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

pub fn git_push(remote_name: &str, refspec: &str) -> Result<(), git2::Error> {
    let repo = Repository::open(".")?;
    let mut remote = repo.find_remote(remote_name)?;

    remote.push(&[refspec], None)?;
    println!("push complete to remote: {}", remote_name);
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
            "push" => {
                if tokens.len() < 3 {
                    println!("입력 형식: push <remote> <refspec>");
                } else {
                    let remote = tokens[1];
                    let refspec = tokens[2];
                    if let Err(e) = git_push(remote, refspec) {
                        println!("push error: {}", e);
                    }
                }
            }
            "log" => match git_log() {
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

#[cfg(test)]
mod tests {
    use super::*;
    use once_cell::sync::Lazy;
    use serial_test::serial;
    use std::fs::File;
    use std::{env, fs};
    use tempfile::TempDir;
    use uuid::Uuid;

    // 전역 공유 임시 디렉토리와 repo 초기화
    static SHARED_REPO: Lazy<TempDir> = Lazy::new(|| {
        let tmp_dir = TempDir::new().expect("failed to create temporary directory");
        env::set_current_dir(tmp_dir.path()).expect("failed to set temporary directory");
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
        let _repo = get_repo();
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

    #[test]
    #[serial]
    fn test_git_log() {
        let _repo = get_repo();

        let file_name = generate_random_file_name(".txt");
        let file_path = Path::new(file_name.as_str());
        File::create(file_path).expect("failed to create temp file");
        git_add(file_name.as_str()).expect("failed to add file");

        let commit_msg = "log test commit";

        git_commit(commit_msg).expect("failed to commit");

        let logs = git_log().expect("failed to get log");
        assert_eq!(logs.first().unwrap().trim(), commit_msg, "커밋 로그가 다름");
    }

    #[test]
    #[serial]
    fn test_git_push() {
        let remote_dir = TempDir::new().expect("failed to create temporary directory");
        let remote_path = remote_dir.path();
        let remote_repo =
            Repository::init_bare(remote_path).expect("failed to initialize bare repository");

        let local_dir = TempDir::new().expect("failed to create temporary directory");
        env::set_current_dir(local_dir.path()).expect("failed to set current directory");
        let local_repo = Repository::init(".").expect("failed to initialize bare repository");

        // 로컬 repository에 원격(origin) 추가
        local_repo
            .remote("origin", remote_path.to_str().unwrap())
            .expect("failed to remote");

        let file_name = generate_random_file_name(".txt");
        fs::write(file_name.as_str(), "push test").expect("failed to write file");

        {
            let mut index = local_repo.index().expect("failed to get index");
            index
                .add_path(Path::new(file_name.as_str()))
                .expect("failed to add file");
            index.write().expect("failed to write index");
        }

        {
            let sig = local_repo.signature().expect("failed to get signature");
            let tree_id = local_repo
                .index()
                .unwrap()
                .write_tree()
                .expect("트리 쓰기 실패");
            let tree = local_repo.find_tree(tree_id).expect("failed to find tree");
            // 부모 커밋이 없으므로 빈 슬라이스 사용
            local_repo
                .commit(Some("HEAD"), &sig, &sig, "push test commit", &tree, &[])
                .expect("failed to commit");
        }

        // 로컬에서 원격의 refs/heads/main로 push 수행
        git_push("origin", "refs/heads/main").expect("failed to push origin");

        // 원격 repo에서 HEAD commit 검증
        let remote_head = remote_repo
            .revparse_single("HEAD")
            .expect("failed to get HEAD");
        let remote_commit = remote_repo
            .find_commit(remote_head.id())
            .expect("failed to find commit");
        assert_eq!(
            remote_commit.message().unwrap(),
            "push test commit",
            "remote 커밋 메시지가 다름"
        );
    }

    fn generate_random_file_name(suffix: &str) -> String {
        format!("{}{}", Uuid::new_v4(), suffix)
    }
}
