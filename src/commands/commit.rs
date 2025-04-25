use git2::Repository;

pub fn git_commit(message: &str) -> Result<(), git2::Error> {
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

#[cfg(test)]
mod tests {
    use crate::commands;
    use crate::test_helpers::get_repo;
    use serial_test::serial;
    use std::fs::File;
    use std::path::Path;

    #[test]
    #[serial]
    fn test_git_commit() {
        let repo = get_repo();

        let file_name = "hello.txt";
        let file_path = Path::new(file_name);
        File::create(file_path).expect("failed to create temp file");
        commands::git_add(file_name).expect("failed to add file");

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
}
