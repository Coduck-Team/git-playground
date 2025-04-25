use git2::Repository;

pub fn git_init() -> Result<(), git2::Error> {
    let _repo = Repository::init(".")?;
    println!("repo init success.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::test_helpers::get_repo;
    use serial_test::serial;
    use std::path::Path;

    #[test]
    #[serial]
    fn test_git_init() {
        let _repo = get_repo();
        assert!(Path::new(".git").exists());
    }
}
