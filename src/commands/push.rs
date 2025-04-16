use git2::Repository;

pub fn git_push(remote_name: &str, refspec: &str) -> Result<(), git2::Error> {
    let repo = Repository::open(".")?;
    let mut remote = repo.find_remote(remote_name)?;

    remote.push(&[refspec], None)?;
    println!("push complete to remote: {}", remote_name);
    Ok(())
}
