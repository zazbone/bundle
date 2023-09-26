#![feature(io_error_more)]

use std::error::Error;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::{io, io::Write};

mod index;

const DIR_NAME_LENGTH: usize = 8;

#[derive(Debug)]
pub struct Bundle<T: AsRef<Path>> {
    path: T,
    index: index::BundleIndex,
}

impl<T: AsRef<Path>> Bundle<T> {
    pub fn create(path: T) -> io::Result<Self> {
        let path_ref = path.as_ref();
        if path_ref.exists() {
            if !is_empty_folder(path_ref)? {
                return io::Result::Err(io::Error::new(
                    io::ErrorKind::AlreadyExists,
                    "Bundle folder is already existing and not empty.",
                ));
            }
        }
        std::fs::create_dir_all(path_ref)?;
        let index = index::BundleIndex::new(get_file_name(path_ref)?);
        Ok(Self { path, index })
    }

    pub fn open(path: T) -> io::Result<Self> {
        let path_ref = path.as_ref();
        if !path_ref.exists() {
            return io::Result::Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Not a bundle folder.",
            ));
        }
        let index: index::BundleIndex =
            serde_json::from_reader(File::open(path_ref.join("index.json"))?)?;
        Ok(Self { path, index })
    }

    pub fn save(&self) -> io::Result<()> {
        let mut index_file = File::create(self.path.as_ref().join("index.json"))?;
        writeln!(index_file, "{}", serde_json::to_string(&self.index)?)?;
        Ok(())
    }

    pub fn new_field(&mut self, field_meta: &serde_json::Value) -> Result<PathBuf, Box<dyn Error>> {
        if let Some(_) = self.index.find_entry_index(field_meta) {
            Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "Field already exist.",
            ))?
        } else {
            let entry_index = self.index.new_entry(field_meta)?;
            Ok(self.path.as_ref().join(format!(
                "{:0>width$}",
                entry_index,
                width = DIR_NAME_LENGTH
            )))
        }
    }

    pub fn get_field(&mut self, field_meta: &serde_json::Value) -> Option<PathBuf> {
        self.index.find_entry_index(field_meta).map(|i| {
            self.path
                .as_ref()
                .join(format!("{:0>width$}", i, width = DIR_NAME_LENGTH))
        })
    }

    pub fn rm_field(&mut self, field_meta: &serde_json::Value) -> Option<PathBuf> {
        if let Some(index) = self.index.find_entry_index(field_meta) {
            self.index.rm_entry(index);
            Some(
                self.path
                    .as_ref()
                    .join(format!("{:0>width$}", index, width = DIR_NAME_LENGTH)),
            )
        } else {
            None
        }
    }
}

fn is_empty_folder<T: AsRef<Path>>(path: T) -> io::Result<bool> {
    if !path.as_ref().is_dir() {
        return Ok(false);
    }
    Ok(path.as_ref().read_dir()?.count() == 0)
}

fn get_file_name<T: AsRef<Path>>(path: T) -> io::Result<String> {
    if let Some(name) = path.as_ref().file_name() {
        name.to_str()
            .ok_or(io::Error::new(
                io::ErrorKind::InvalidFilename,
                "The given path does not end with a valid path name",
            ))
            .map(|s| s.to_string())
    } else {
        Ok(path
            .as_ref()
            .canonicalize()?
            .file_name()
            .ok_or(io::ErrorKind::InvalidFilename)?
            .to_str()
            .ok_or(io::ErrorKind::InvalidFilename)?
            .into())
    }
}
