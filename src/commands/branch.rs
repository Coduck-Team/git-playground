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
