use git2::build::CheckoutBuilder;
use git2::Repository;

pub fn git_checkout(branch: &str) -> Result<(), git2::Error> {
    let repo = Repository::open(".")?;

    let (object, reference) = repo.revparse_ext(branch)?;

    let mut checkout_builder = CheckoutBuilder::new();
    checkout_builder.force();

    repo.checkout_tree(&object, Some(&mut checkout_builder))?;

    match reference {
        None => {
            repo.set_head_detached(object.id())?;
        }
        Some(ref_ref) => {
            if let Some(name) = ref_ref.name() {
                repo.set_head(name)?;
            } else {
                return Err(git2::Error::from_str("유효하지 않은 레퍼런스 이름"));
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::commands::git_checkout;
    use crate::test_helpers::{get_repo, write_dummy_add_commit};
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_git_checkout() {
        let repo = get_repo();

        write_dummy_add_commit();

        let branch_name = "test_branch";
        repo.branch(
            branch_name,
            &repo
                .find_commit(repo.refname_to_id("HEAD").unwrap())
                .unwrap(),
            false,
        )
        .expect("failed to create branch");

        git_checkout(branch_name).expect("체크아웃 실패");

        // HEAD의 현재 브랜치가 test-branch인지 확인
        let head_ref = repo.head().expect("HEAD 참조 읽기 실패");
        let head_name = head_ref.shorthand().unwrap_or("");
        assert_eq!(head_name, branch_name, "체크아웃된 브랜치가 다릅니다");
    }
}
