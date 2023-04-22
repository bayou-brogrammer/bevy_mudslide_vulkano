use std::{
    fs,
    path::{Path, PathBuf},
};
use thiserror::Error;

pub type FileResult<T> = std::result::Result<T, FileUtilsError>;

#[allow(unused)]
#[derive(Error, Debug)]
pub enum FileUtilsError {
    #[error("{} is not a directory,", .0)]
    NotADir(String),
    #[error("{} is not a file", .0)]
    NotAFile(String),
    #[error("Not a string.")]
    NotAString,

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Std(Box<dyn std::error::Error>),

    #[error(transparent)]
    RonError(#[from] ron::Error),
}

pub struct FileUtils;
impl FileUtils {
    // Reading files

    #[allow(unused)]
    pub fn read_str<Path: Into<PathBuf>>(path: Path) -> FileResult<String> {
        let path: PathBuf = path.into();
        match std::fs::read_to_string(path) {
            Ok(s) => Ok(s),
            Err(e) => Err(FileUtilsError::Io(e)),
        }
    }

    pub fn read_ron<T: for<'a> serde::Deserialize<'a>>(path: impl Into<PathBuf>) -> FileResult<T> {
        let path: PathBuf = path.into();
        std::fs::canonicalize::<PathBuf>(path).map_or_else(
            |err| Err(FileUtilsError::NotAFile(err.to_string())),
            |path| {
                std::fs::File::open(path).map_or_else(
                    |err| Err(FileUtilsError::Io(err)),
                    |file| {
                        ron::de::from_reader::<_, T>(file).map_or_else(
                            |err| Err(FileUtilsError::RonError(err.code)),
                            |ron| Ok(ron),
                        )
                    },
                )
            },
        )
    }

    #[allow(unused)]
    pub fn write_str<P: AsRef<Path>>(path: P, value: &str) -> FileResult<()> {
        let path = path.as_ref();
        let path_string = match path.to_str() {
            Some(s) => s.to_string(),
            None => return Err(FileUtilsError::NotAString),
        };

        match path.try_exists() {
            Ok(b) => {
                if !b {
                    match path.parent() {
                        Some(dir) => {
                            #[cfg(feature = "debug")]
                            debug!("Creating directory: {:?}", dir);
                            if let Err(e) = fs::create_dir_all(dir) {
                                return Err(FileUtilsError::Io(e));
                            }
                        }
                        None => return Err(FileUtilsError::NotADir(path_string)),
                    };
                }

                #[cfg(feature = "debug")]
                debug!("Writing to file: {:?}", path);
                match fs::write(path, value) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(FileUtilsError::Io(e)),
                }
            }
            Err(e) => Err(FileUtilsError::Io(e)),
        }
    }
}
