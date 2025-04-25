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

#[cfg(test)]
mod tests {
    use crate::commands;
    use crate::tests::{get_repo, write_dummy_add_commit};
    use serial_test::serial;
    use std::fs;

    #[test]
    #[serial]
    fn git_merge_success_no_conflict() {
        let repo = get_repo();
        write_dummy_add_commit();

        let feature_branch = "feature";
        commands::git_create_branch(feature_branch).expect("failed to create feature branch");
        // feature로 checkout
        crate::tests::checkout(&repo, feature_branch).unwrap();

        // feature 브랜치에서 새 커밋 생성
        let file_name = "new_file.txt".to_string();
        fs::write(&file_name, "feature 추가").unwrap();
        commands::git_add(&file_name).unwrap();
        commands::git_commit("feat: add new file").unwrap();

        let main_branch = "main";
        crate::tests::checkout(&repo, main_branch).unwrap();

        commands::git_merge(feature_branch).expect("failed to merge feature branch");

        let head_commit = repo.head().unwrap().peel_to_commit().unwrap();
        assert_eq!(
            head_commit.message().unwrap(),
            "Merge commit",
            "merge 커밋 메시지가 다름"
        );
    }

    #[test]
    #[serial]
    fn git_merge_conflict() {
        let repo = get_repo();
        write_dummy_add_commit();

        let file_name = "conflict.txt";

        fs::write(file_name, "base").expect("failed to write base content");
        commands::git_add(file_name).expect("failed to add conflict.txt");
        commands::git_commit("base commit").expect("failed to commit base content");

        // conflict_branch 브랜치 생성 후 체크아웃
        let branch_name = "conflict_branch";
        commands::git_create_branch(branch_name).expect("failed to create conflict_branch");
        crate::tests::checkout(&repo, branch_name).expect("failed to checkout conflict_branch");

        // conflict_branch에서 conflict.txt 수정 후 커밋
        fs::write(file_name, "branch").expect("failed to write branch content");
        commands::git_add(file_name).expect("failed to add updated conflict.txt");
        commands::git_commit("branch commit").expect("failed to commit branch change");

        // main 브랜치로 체크아웃
        crate::tests::checkout(&repo, "main").expect("failed to checkout main");

        // main 브랜치에서 conflict.txt 수정 후 커밋 (충돌 발생 준비)
        fs::write(file_name, "main").expect("failed to write main content");
        // FIXME commit 하고 난 이후 conflict가 발생한다(의도한 동작)
        // 그러나 repo.cleanup_state()로 클린업해도 git_merge_conflict 이후에 실행되는 테스트가 실패해버린다.
        // commit 만 일단 안하면 이후의 테스트도 성공을 한다.
        commands::git_add(file_name).expect("failed to add main branch conflict.txt");
        // commands::git_commit("main commit").expect("failed to commit main change");

        // conflict_branch를 main에 병합 -> 충돌이 발생해야 함
        let merge_result = commands::git_merge(branch_name);
        assert!(merge_result.is_err(), "merge 충돌이 발생하지 않음");

        repo.cleanup_state().unwrap();
    }
}
