use git2::{build::CheckoutBuilder, Error, Repository};

pub fn git_restore(path: &str) -> Result<(), Error> {
    let repo = Repository::open(".")?;
    let head = repo.head()?;
    let commit = head.peel_to_commit()?;
    let binding = commit.tree()?;
    let tree = binding.as_object();
    let mut checkout_opts = CheckoutBuilder::new();
    checkout_opts.path(path);
    checkout_opts.force(); // --force
    repo.checkout_tree(&tree, Some(&mut checkout_opts))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands;
    use crate::test_helpers::{get_repo, write_dummy_add_commit};
    use serial_test::serial;
    use std::fs;

    #[test]
    #[serial]
    fn test_git_restore() {
        let repo = get_repo();
        // 초기 HEAD 커밋이 없으면 더미 커밋을 수행합니다.
        if repo.head().is_err() {
            write_dummy_add_commit();
        }

        let file_name = "restore_test_file.txt";
        let original_content = "original content";
        // 파일 생성 후 add, commit 수행
        fs::write(file_name, original_content).expect("파일 작성 실패");
        commands::git_add(file_name).expect("파일 stage 실패");
        commands::git_commit("commit original content").expect("커밋 실패");

        // 파일 내용을 변경
        let modified_content = "modified content";
        fs::write(file_name, modified_content).expect("파일 수정 실패");

        // git_restore를 호출하여 파일 내용을 HEAD 상태로 복원
        git_restore(file_name).expect("restore 실패");

        // 파일 내용을 확인하여 복원이 제대로 되었는지 검증
        let restored_content = fs::read_to_string(file_name).expect("파일 읽기 실패");
        assert_eq!(
            restored_content, original_content,
            "복원된 파일 내용이 일치하지 않음"
        );
    }
}
