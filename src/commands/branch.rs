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
