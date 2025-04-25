use git2::{IndexAddOption, Repository};
use std::path::Path;

pub fn git_add(path_str: &str) -> Result<(), git2::Error> {
    let repo = Repository::open(".")?;
    let mut idx = repo.index()?;

    if path_str == "." {
        idx.add_all(["."].iter(), IndexAddOption::DEFAULT, None)?;
        println!("all added.");
    } else {
        let path = Path::new(path_str);
        idx.add_path(&path)?;
        println!("{} added.", path_str);
    }
    idx.write()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::commands;
    use crate::test_helpers::get_repo;
    use serial_test::serial;
    use std::fs::File;
    use std::path::Path;

    #[test]
    #[serial]
    fn test_git_add_specific_file() {
        let repo = get_repo();

        let file_name = "hello.txt";

        let file_path = Path::new(file_name);
        File::create(file_path).expect("failed to create temp file");

        commands::git_add(file_name).expect("failed to add file");

        let index = repo.index().expect("failed to get the index");
        let entries: Vec<_> = index
            .iter()
            .filter(|entry| std::str::from_utf8(&entry.path).unwrap() == file_name)
            .collect();
        assert_eq!(entries.len(), 1);
    }

    #[test]
    #[serial]
    fn test_git_add_all_files() {
        let repo = get_repo();
        let file_name = "bye.txt";

        let file_path = Path::new(file_name);
        File::create(file_path).expect("failed to create temp file");

        commands::git_add(".").expect("failed to add file");

        let index = repo.index().unwrap();
        let entries: Vec<_> = index
            .iter()
            .filter(|entry| std::str::from_utf8(&entry.path).unwrap() == file_name)
            .collect();
        assert_eq!(entries.len(), 1);
    }
}
