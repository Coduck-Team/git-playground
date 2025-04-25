use git2::{BranchType, Repository};

pub fn git_show_branch() -> Result<(), git2::Error> {
    let repo = Repository::open(".")?;
    let branches = repo.branches(Some(BranchType::Local))?;

    let head = repo.head()?;
    let head_name = head.shorthand().unwrap_or("HEAD");

    println!("Branch 목록:");
    for branch in branches {
        let (branch, _) = branch?;
        let name = branch.name()?.unwrap_or("unknown");
        if name == head_name {
            println!("* {name}");
        } else {
            println!("  {name}");
        }
    }
    Ok(())
}

pub fn git_create_branch(branch_name: &str) -> Result<(), git2::Error> {
    let repo = Repository::open(".")?;

    let head_ref = repo.head()?.resolve()?;
    let commit = head_ref.peel_to_commit()?;

    // 브랜치 생성. force:false 인데, 이건 같은 이름으로 브랜치가 존재하는 경우 에러 발생
    repo.branch(branch_name, &commit, false)?;
    println!("branch '{}' created", branch_name);
    Ok(())
}

pub fn git_delete_branch(branch_name: &str) -> Result<(), git2::Error> {
    let repo = Repository::open(".")?;

    let head = repo.head()?;
    let head_name = head.shorthand().unwrap_or("HEAD");

    if head_name == branch_name {
        return Err(git2::Error::from_str("현재 체크아웃 된 브랜치는 삭제 불가"));
    }

    let mut branch = repo.find_branch(branch_name, BranchType::Local)?;
    branch.delete()?;

    println!("branch '{}' deleted", branch_name);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::commands;
    use crate::tests::get_repo;
    use git2::BranchType;
    use serial_test::serial;
    use uuid::Uuid;

    #[test]
    #[serial]
    fn test_git_show_branch() {
        let repo = get_repo();
        // 이게 젤 처음이라서 에러가 발생한다면,
        if repo.head().is_err() {
            crate::tests::write_dummy_add_commit();
        }
        // FIXME 과연 정말 이게 테스트 한다고 볼수 있을까?
        // show branch 를 유닛 리턴하고 해당 메서드에서 출력했던 이유는 이 방법이 현재
        // 체크아웃 된 브랜치를 마크하며 출력하기 가장 편했음.
        // 근데 테스트 하기에는 살짝쿵 애매하다.
        assert!(commands::git_show_branch().is_ok());
    }

    // FIXME branch 관련 테스트는 제일 먼저 실행되면 실패한다. 그래서 임시로 파일 생성하고 commit 하는 구간이 생겼다.
    // 더 나은 방법이 있을까? 현재는 코드도 너져분 하고 뭘 테스트하고자 하는지 한눈에 안들어온다.
    // TODO 리팩터링 하자.
    #[test]
    #[serial]
    fn test_git_create_branch() {
        let repo = get_repo();
        let branch_name = format!("create_test_{}", Uuid::new_v4());

        // 이게 젤 처음이라서 에러가 발생한다면,
        if repo.head().is_err() {
            crate::tests::write_dummy_add_commit();
        }

        // 이미 동일 이름의 브랜치 있다면 삭제 후 진행.
        if let Ok(mut branch) = repo.find_branch(&branch_name, BranchType::Local) {
            branch.delete().expect("기존에 있던 test 브랜치 삭제 실패");
        }

        assert!(commands::git_create_branch(&branch_name).is_ok());

        // 브랜치 생성 확인
        assert!(repo.find_branch(&branch_name, BranchType::Local).is_ok());

        // clean up
        if let Ok(mut branch) = repo.find_branch(&branch_name, BranchType::Local) {
            branch.delete().expect("test 브랜치 삭제 실패");
        }
    }

    #[test]
    #[serial]
    fn test_git_delete_branch() {
        let repo = get_repo();
        let branch_name = format!("delete_test_{}", Uuid::new_v4());

        // 이게 젤 처음이라서 에러가 발생한다면,
        if repo.head().is_err() {
            crate::tests::write_dummy_add_commit();
        }

        if let Ok(mut branch) = repo.find_branch(&branch_name, BranchType::Local) {
            branch.delete().expect("기존에 있던 test 브랜치 삭제 실패");
        }

        // 삭제할 브랜치 먼저 생성
        assert!(commands::git_create_branch(&branch_name).is_ok());
        // 이럴바에 그냥 git2가 제공하는 branch create 사용하는게 나으려나
        assert!(repo.find_branch(&branch_name, BranchType::Local).is_ok());

        // 브랜치 삭제 테스트
        assert!(commands::git_delete_branch(&branch_name).is_ok());
        // 삭제 확인
        assert!(repo.find_branch(&branch_name, BranchType::Local).is_err());
    }
}
