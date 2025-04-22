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
