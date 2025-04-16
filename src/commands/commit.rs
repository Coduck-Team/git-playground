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
