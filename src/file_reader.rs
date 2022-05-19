use std::io;
use std::fs::{self};
use std::path::{Path, PathBuf};

pub fn get_rs_file_paths<'a, P: AsRef<Path>>(dir: P) -> Vec<PathBuf> {
    let mut results = vec![];
    match visit_dirs(dir.as_ref(), &mut results) {
        Ok(_) => (),
        Err(e) => panic!("{:?}", e)
    }
    results.into_iter().filter(|pb| pb.extension().unwrap() == ("rs")).collect()
}

// one possible implementation of walking a directory only visiting files
fn visit_dirs(dir: &Path, results: &mut Vec<PathBuf>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, results)?;
            } else {
                results.push(entry.path());
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visit_dirs() {
        let results: Vec<PathBuf> = get_rs_file_paths("tests/sample_crate");
        assert_eq!(
            results,
            [
                Path::new("tests/sample_crate\\src\\main.rs").to_path_buf(),
            ]
        )
    }
}