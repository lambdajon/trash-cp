use std::env;
use std::fs::File;
use std::io::Result;
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

fn get_current_exe() -> Result<String> {
    return match env::current_exe() {
        Ok(path) => Ok(path.as_path().display().to_string()),
        Err(err) => panic!("Error while get current exe: {:?}", err),
    };
}

fn get_current_dir() -> Result<String> {
    return match env::current_dir() {
        Ok(path) => Ok(path.as_path().display().to_string()),
        Err(err) => panic!("Error while get current exe: {:?}", err),
    };
}

#[cfg(test)]
mod filesystem_tests {
    #![feature(file_create_new)]

    use crate::filesystem::get_current_dir;

    use super::get_current_exe;
    use super::valid_destination;
    use super::valid_source;
    use std::env;
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::Error as IOError;
    use std::io::Write;
    use std::path::{Path, PathBuf};

    fn get_tmp_directory() -> String {
        let current_dir = get_current_dir().unwrap();

        let res = [&current_dir, "/tmp/"].join("");

        return res;
    }

    #[test]
    fn check_valid_source() {
        let exe_path = get_current_exe();
        assert_eq!(valid_source(exe_path.unwrap()), true);
    }

    #[test]
    fn check_valid_destination() {
        let dest_path = get_current_dir();
        assert_eq!(valid_destination(dest_path.unwrap()), true);
    }

    #[test]
    fn check_valid_source_via_create_file_tmp() {
        let mut f_path = PathBuf::new();
        f_path.push(get_tmp_directory());
        f_path.push("dummy.txt");

        let mut dummy_file = File::create(f_path.clone()).unwrap();
        dummy_file.write_all(b"Some text");
        dummy_file.sync_all();

        let dummy_file_path = String::from(f_path.into_os_string().into_string().unwrap());

        assert_eq!(valid_source(dummy_file_path), true);
    }
}
