use git2::Repository;

pub fn git_push(remote_name: &str, refspec: &str) -> Result<(), git2::Error> {
    let repo = Repository::open(".")?;
    let mut remote = repo.find_remote(remote_name)?;

    remote.push(&[refspec], None)?;
    println!("push complete to remote: {}", remote_name);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::commands;
    use git2::Repository;
    use serial_test::serial;
    use std::path::Path;
    use std::{env, fs};
    use tempfile::TempDir;

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

        let file_name = "push.txt";
        fs::write(file_name, "push test").expect("failed to write file");

        {
            let mut index = local_repo.index().expect("failed to get index");
            index
                .add_path(Path::new(file_name))
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
}
