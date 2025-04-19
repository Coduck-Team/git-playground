use git2::Repository;

pub fn git_init() -> Result<(), git2::Error> {
    let _repo = Repository::init(".")?;
    println!("repo init success.");
    Ok(())
}
