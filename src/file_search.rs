use std::{
    fs,
    io::{self, Error},
    path::PathBuf,
};

pub struct SrrFileSet {
    pub file_set: Vec<PathBuf>,
}

impl SrrFileSet {
    pub fn new() -> SrrFileSet {
        SrrFileSet { file_set: vec![] }
    }
    pub fn from_filepath(file_path: PathBuf) -> Result<SrrFileSet, Error> {
        if file_path.exists() {
            println!("file_path : {:?}", file_path);
            Ok(SrrFileSet {
                file_set: vec![file_path],
            })
        } else {
            Err(Error::new(
                io::ErrorKind::InvalidInput,
                "File doe not exist",
            ))
        }
    }
    pub fn from_directory(dir_path: PathBuf) -> Result<SrrFileSet, Error> {
        let mut file_set: Vec<PathBuf> = vec![];
        if dir_path.is_dir() {
            for entry in fs::read_dir(dir_path)? {
                let entry = match entry {
                    Ok(it) => it,
                    Err(err) => return Err(err),
                };
                let path = entry.path();
                if !path.is_dir() {
                    file_set.push(path);
                }
            }

            Ok(SrrFileSet { file_set })
        } else {
            Err(Error::last_os_error())
        }
    }
}
