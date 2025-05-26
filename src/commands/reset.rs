use git2::{build::CheckoutBuilder, Error, Repository, ResetType};

pub fn git_reset(path: &str, reset_type: &str) -> Result<(), Error> {
    let repo = Repository::open(".")?;
    let head = repo.head()?;
    let head_commit = head.peel_to_commit()?;
    let rt = match reset_type.to_lowercase().as_str() {
        "soft" => ResetType::Soft,
        "hard" => ResetType::Hard,
        "mixed" | _ => ResetType::Mixed,
    };

    match rt {
        ResetType::Soft => {
            repo.reset(head_commit.as_object(), rt, None)?;
        }
        ResetType::Mixed => {
            let mut checkout_opts = CheckoutBuilder::new();
            checkout_opts.path(path);
            repo.reset(head_commit.as_object(), rt, Some(&mut checkout_opts))?;
        }
        ResetType::Hard => {
            let mut checkout_opts = CheckoutBuilder::new();
            checkout_opts.path(path);
            checkout_opts.force();
            repo.reset(head_commit.as_object(), rt, Some(&mut checkout_opts))?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands;
    use crate::test_helpers::{get_repo, write_dummy_add_commit};
    use serial_test::serial;
    use std::fs;
    use std::path::Path;

    #[test]
    #[serial]
    fn test_git_reset_soft() {
        let repo = get_repo();
        if repo.head().is_err() {
            write_dummy_add_commit();
        }
        let file_name = "reset_soft_test.txt";
        let original_content = "original content";
        fs::write(file_name, original_content).expect("파일 작성 실패");
        commands::git_add(file_name).expect("파일 stage 실패");
        commands::git_commit("commit original content").expect("커밋 실패");

        // 파일 내용을 수정하고 add
        let modified_content = "modified content";
        fs::write(file_name, modified_content).expect("파일 수정 실패");
        commands::git_add(file_name).expect("파일 stage 실패");

        // soft reset: 인덱스와 워킹 트리 모두 그대로 유지해야 함
        git_reset(file_name, "soft").expect("soft reset 실패");

        // 인덱스의 엔트리와 비교하지 않습니다. soft reset은 인덱스를 건드리지 않음
        let working_content = fs::read_to_string(file_name).expect("파일 읽기 실패");
        assert_eq!(
            working_content, modified_content,
            "워킹 디렉토리 내용이 변경됨"
        );
    }

    #[test]
    #[serial]
    fn test_git_reset_mixed() {
        let repo = get_repo();
        if repo.head().is_err() {
            write_dummy_add_commit();
        }
        let file_name = "reset_mixed_test.txt";
        let original_content = "original content";
        fs::write(file_name, original_content).expect("파일 작성 실패");
        commands::git_add(file_name).expect("파일 stage 실패");
        commands::git_commit("commit original content").expect("커밋 실패");

        // 파일 내용 수정 후 add
        let modified_content = "modified content";
        fs::write(file_name, modified_content).expect("파일 수정 실패");
        commands::git_add(file_name).expect("파일 stage 실패");

        // mixed reset: 인덱스는 초기 상태로, 워킹 디렉토리는 수정 내용 유지
        git_reset(file_name, "mixed").expect("mixed reset 실패");

        let head_commit = repo.head().unwrap().peel_to_commit().unwrap();
        let head_tree = head_commit.tree().unwrap();
        let tree_entry = head_tree
            .get_path(Path::new(file_name))
            .expect("HEAD tree에 파일이 없음");

        let idx = repo.index().expect("인덱스 로드 실패");
        let index_entry = idx
            .get_path(Path::new(file_name), 0)
            .expect("인덱스에 파일이 없음");

        assert_eq!(index_entry.id, tree_entry.id(), "인덱스가 reset되지 않음");
        let working_content = fs::read_to_string(file_name).expect("파일 읽기 실패");
        assert_eq!(
            working_content, modified_content,
            "워킹 디렉토리 내용이 변경됨"
        );
    }

    #[test]
    #[serial]
    fn test_git_reset_hard() {
        let repo = get_repo();
        if repo.head().is_err() {
            write_dummy_add_commit();
        }
        let file_name = "reset_hard_test.txt";
        let original_content = "original content";
        fs::write(file_name, original_content).expect("파일 작성 실패");
        commands::git_add(file_name).expect("파일 stage 실패");
        commands::git_commit("commit original content").expect("커밋 실패");

        // 파일 내용을 수정하고 add
        let modified_content = "modified content";
        fs::write(file_name, modified_content).expect("파일 수정 실패");
        commands::git_add(file_name).expect("파일 stage 실패");

        // hard reset: 인덱스와 워킹 디렉토리 모두 HEAD 상태로 복원
        git_reset(file_name, "hard").expect("hard reset 실패");

        let head_commit = repo.head().unwrap().peel_to_commit().unwrap();
        let head_tree = head_commit.tree().unwrap();
        let tree_entry = head_tree
            .get_path(Path::new(file_name))
            .expect("HEAD tree에 파일이 없음");

        let idx = repo.index().expect("인덱스 로드 실패");
        let index_entry = idx
            .get_path(Path::new(file_name), 0)
            .expect("인덱스에 파일이 없음");

        assert_eq!(index_entry.id, tree_entry.id(), "인덱스가 reset되지 않음");

        let working_content = fs::read_to_string(file_name).expect("파일 읽기 실패");
        assert_eq!(
            working_content, original_content,
            "워킹 디렉토리 내용이 변경됨"
        );
    }
}
