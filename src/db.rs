//  -------------------------------------------------------------
//  Alkane :: Database
//  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
//  Project:        Nasqueron
//  License:        BSD-2-Clause
//  -------------------------------------------------------------

use std::fs;
use std::fs::OpenOptions;
use std::io::Error as IOError;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use log::warn;

use crate::config::AlkaneConfig;

pub struct Database {
    root: String,
}

impl Database {
    pub fn new<S>(root: S) -> Self
    where
        S: AsRef<str>,
    {
        Self {
            root: root.as_ref().to_string(),
        }
    }

    pub fn from_config(config: &AlkaneConfig) -> Option<Self> {
        config.get_root("db").map(Self::new)
    }

    pub fn is_initialized(&self, site_name: &str) -> bool {
        self.get_initialized_path(site_name).exists()
    }

    pub fn set_initialized(&self, site_name: &str) -> bool {
        let path = self.get_initialized_path(site_name);

        if !path.exists() {
            match ensure_parent_directory_exists(&path) {
                Ok(_) => match touch(&path) {
                    Ok(_) => true,
                    Err(error) => {
                        warn!("Can't mark site {} as initialized: {:?}", site_name, error);

                        false
                    }
                },
                Err(error) => {
                    warn!("Can't create parent directory for {:?}: {:?}", &path, error);

                    false
                }
            }
        } else {
            true
        }
    }

    fn get_initialized_path(&self, site_name: &str) -> PathBuf {
        Path::new(&self.root).join("initialized").join(site_name)
    }
}

/// Creates an empty file, similar to the touch command
/// Ignores existing files.
fn touch(path: &PathBuf) -> Result<(), IOError> {
    let mut options = OpenOptions::new();
    options.create(true).write(true);

    options.open(path).map(|_| ())
}

fn ensure_parent_directory_exists(path: &PathBuf) -> Result<(), IOError> {
    let parent = path
        .parent()
        .ok_or_else(|| IOError::new(ErrorKind::InvalidInput, "Invalid path"))?;

    if !parent.exists() {
        fs::create_dir_all(parent)?;
    }

    Ok(())
}

//  -------------------------------------------------------------
//  Tests
//  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    pub fn test_touch() {
        let path = Path::new("tmp-touch.empty");
        assert!(
            !path.exists(),
            "Temporary file tmp-touch.empty shouldn't exist when test starts"
        );

        touch(&path.to_path_buf()).expect("File can't be created");
        assert!(
            path.exists(),
            "Function touch returned Ok but temporary file does NOT exist."
        );

        fs::remove_file(path).expect("Can't remove file after having created it.")
    }
}
