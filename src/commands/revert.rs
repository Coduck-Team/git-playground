use git2::build::CheckoutBuilder;
use git2::{ApplyLocation, Oid, Repository};

pub fn git_revert(commit_id: &str) -> Result<(), git2::Error> {
    let repo = Repository::open(".")?;

    // 타켓 커밋을 Oid로 변환후 찾기
    let target_oid = Oid::from_str(commit_id)?;
    let target_commit = repo.find_commit(target_oid)?;

    // 타켓 커밋의 첫번째 부모를 찾기
    let parent_commit = target_commit.parent(0)?;

    // 타켓과 부모의 트리 객체 가져오기
    let target_tree = target_commit.tree()?;
    let parent_tree = parent_commit.tree()?;

    // 부모와 타겟의 순서를 바꿔 역방향 diff를 만들면 타켓 커밋의 변경사항을 되돌리는 패치가 생성
    let diff = repo.diff_tree_to_tree(Some(&target_tree), Some(&parent_tree), None)?;

    repo.apply(&diff, ApplyLocation::Index, None)?;

    let mut idx = repo.index()?;
    let tree_id = idx.write_tree()?;
    let tree = repo.find_tree(tree_id)?;

    // 작업 디렉토리 업데이트
    let mut checkout = CheckoutBuilder::new();
    repo.checkout_index(Some(&mut idx), Some(&mut checkout))?;

    let head_oid = repo.refname_to_id("HEAD")?;
    let head_commit = repo.find_commit(head_oid)?;

    let sig = repo.signature()?;

    // 커밋 메시지에 Revert 추가
    let summary = target_commit.summary().unwrap_or("");
    let commit_msg = format!("Revert \"{}\"", summary);

    repo.commit(
        Some("HEAD"),
        &sig,
        &sig,
        &commit_msg,
        &tree,
        &[&head_commit],
    )?;

    println!("Revert commit created: {}", commit_msg);

    Ok(())
}
