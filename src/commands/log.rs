use git2::Repository;

pub fn git_log() -> Result<Vec<String>, git2::Error> {
    let repo = Repository::open(".")?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    let mut res = Vec::new();
    for commit_id in revwalk {
        let commit = repo.find_commit(commit_id?)?;
        let hash = commit.id().to_string();
        if let Some(msg) = commit.summary() {
            res.push(format!("{}: {}", hash, msg));
        }
    }
    Ok(res)
}

#[cfg(test)]
mod tests {
    use crate::commands;
    use crate::tests::get_repo;
    use serial_test::serial;
    use std::fs::File;
    use std::path::Path;

    #[test]
    #[serial]
    fn test_git_log() {
        let _repo = get_repo();

        let file_name = "world.txt";
        let file_path = Path::new(file_name);
        File::create(file_path).expect("failed to create temp file");
        commands::git_add(file_name).expect("failed to add file");

        let commit_msg = "log test commit";

        commands::git_commit(commit_msg).expect("failed to commit");

        let logs = commands::git_log().expect("failed to get log");
        assert!(
            logs.first().unwrap().contains(commit_msg),
            "커밋 로그가 다름"
        );
    }
}
