use std::path::Path;
use git2::{IndexAddOption, Repository};

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
