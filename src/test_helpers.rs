use git2::Repository;
use once_cell::sync::Lazy;
use std::{env, fs};
use tempfile::TempDir;

// 전역으로 공유하는 임시 디렉토리 초기화
static SHARED_REPO: Lazy<TempDir> = Lazy::new(|| {
    let tmp_dir = TempDir::new().expect("failed to create temporary directory");
    env::set_current_dir(tmp_dir.path()).expect("failed to set temporary directory");
    // 테스트 초기화를 위한 예제 (예: git init 실행)
    Repository::init(".").expect("failed to init repository");
    tmp_dir
});

// Repository를 가져오는 함수
pub fn get_repo() -> Repository {
    env::set_current_dir(SHARED_REPO.path()).expect("failed to set directory");
    Repository::open(".").expect("failed to open repository")
}

// dummy 파일을 작성하고 commit 하는 함수
pub fn write_dummy_add_commit() {
    fs::write("dummy.txt", "initial commit").expect("failed to write dummy file");
    crate::commands::git_add("dummy.txt").expect("failed to add dummy.txt");
    crate::commands::git_commit("initial commit").expect("failed to commit");
}

pub fn checkout(repo: &Repository, branch_name: &str) -> Result<(), git2::Error> {
    let obj = repo.revparse_single(&format!("refs/heads/{}", branch_name))?;
    repo.checkout_tree(&obj, None)?;
    repo.set_head(&format!("refs/heads/{}", branch_name))?;
    Ok(())
}
