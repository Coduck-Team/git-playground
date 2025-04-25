use git2::{MergeOptions, Repository};

pub fn git_merge(branch: &str) -> Result<(), git2::Error> {
    let repo = Repository::open(".")?;

    // 대상 브랜치의 annotatedCommit 가져오기
    let branch_ref = repo.find_branch(branch, git2::BranchType::Local)?;
    let branch_commit = branch_ref.get().peel_to_commit()?;
    let annotated_commit = repo.find_annotated_commit(branch_commit.id())?;

    let mut merge_opts = MergeOptions::new();
    // merge 수행 (워킹 디렉토리와 index에 결과가 반영됨)
    repo.merge(&[&annotated_commit], Some(&mut merge_opts), None)?;

    // 충돌 여부 확인
    let mut index = repo.index()?;
    if index.has_conflicts() {
        repo.cleanup_state()?;
        return Err(git2::Error::from_str("머지 충돌 발생"));
    }

    let tree_oid = index.write_tree()?;
    let tree = repo.find_tree(tree_oid)?;

    let head_commit = repo.head()?.peel_to_commit()?;

    let sig = repo.signature()?;
    repo.commit(
        Some("HEAD"),
        &sig,
        &sig,
        "Merge commit",
        &tree,
        &[&head_commit, &branch_commit],
    )?;

    repo.checkout_head(None)?;
    Ok(())
}
