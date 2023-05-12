use std::path::{Path, PathBuf};

fn valid_source(source: String) -> bool {
    let is_file = Path::new(&source).is_file();
    let is_exist = match Path::new(&source).try_exists() {
        Ok(right) => right,
        Err(lef) => false, // need handle error
    };
    return is_file && is_exist;
}

fn valid_destination(destination: String) -> bool {
    let is_file = Path::new(&destination).is_dir();
    let is_exist = match Path::new(&destination).try_exists() {
        Ok(right) => right,
        Err(lef) => false, // need handle error
    };
    let d: std::path::PathBuf = Path::new("/etc").join("passwd");
    return is_file && is_exist;
}

#[cfg(test)]
mod filesystem_tests {
    use super::valid_destination;
    use super::valid_source;
    use std::path::{Path, PathBuf};
    use std::env;

    #[test]
    fn check_valid_source() {
      
    }
}
