mod commands;

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
                }
            }
            // TODO 로그 출력시 메시지만 보여줄게 아니라 해시도 보여줘야 함
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

#[cfg(test)]
mod tests {
    use super::*;
    use git2::Repository;
    use once_cell::sync::Lazy;
    use serial_test::serial;
    use std::fs::File;
    use std::path::Path;
    use std::{env, fs};
    use tempfile::TempDir;
    use uuid::Uuid;

    // 전역 공유 임시 디렉토리와 repo 초기화
    static SHARED_REPO: Lazy<TempDir> = Lazy::new(|| {
        let tmp_dir = TempDir::new().expect("failed to create temporary directory");
        env::set_current_dir(tmp_dir.path()).expect("failed to set temporary directory");
        commands::git_init().expect("failed to git init");
        tmp_dir
    });

    fn get_repo() -> Repository {
        env::set_current_dir(SHARED_REPO.path()).expect("failed to set temporary directory");
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

        commands::git_add(file_name.as_str()).expect("failed to add file");

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

        commands::git_add(".").expect("failed to add file");

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
        commands::git_add(file_name.as_str()).expect("failed to add file");

        let commit_msg = "test commit msg";
        commands::git_commit(commit_msg).expect("failed to commit message");

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
        commands::git_add(file_name.as_str()).expect("failed to add file");

        let commit_msg = "log test commit";

        commands::git_commit(commit_msg).expect("failed to commit");

        let logs = commands::git_log().expect("failed to get log");
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
        commands::git_push("origin", "refs/heads/main").expect("failed to push origin");

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

    #[test]
    #[serial]
    fn test_git_revert() {
        let repo = get_repo();

        let file_name = generate_random_file_name(".txt");

        // 파일 작성
        fs::write(&file_name, "비빔밥").expect("failed to write file");
        commands::git_add(file_name.as_str()).expect("failed to add file");
        let commit_msg = "비빔밥 먹고싶다.";
        commands::git_commit(commit_msg).expect("failed to commit message");

        let content = fs::read_to_string(file_name.as_str()).expect("failed to read file");
        assert_eq!(content, "비빔밥", "파일 생성 및 변경 안됨");

        // 파일 수정
        fs::write(file_name.as_str(), "국밥").expect("failed to write file");
        commands::git_add(file_name.as_str()).expect("failed to add file");
        let commit_msg = "비빔밥 질렸다.";
        commands::git_commit(commit_msg).expect("failed to commit message");

        let content = fs::read_to_string(file_name.as_str()).expect("failed to read file");
        assert_eq!(content, "국밥", "파일 변경 안됨");

        // HEAD 커밋 id 가져오기
        let head_commit = repo.head().expect("failed to get HEAD");
        let commit_oid = head_commit.target().expect("HEAD refers to non-HEAD");

        // git revert
        commands::git_revert(&commit_oid.to_string()).expect("failed to revert");

        let content = fs::read_to_string(file_name.as_str()).expect("failed to read file");
        assert_eq!(content, "비빔밥", "파일 롤백 안됨");
    }

    fn generate_random_file_name(suffix: &str) -> String {
        format!("{}{}", Uuid::new_v4(), suffix)
    }
}
