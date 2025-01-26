use std::fs::{self, Permissions};
use std::io::{BufReader, Read};
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;

/// It's in charge of manage every file related with this proyect such as root_file or their dependecies in case that it has them.
#[derive(Debug)]
pub struct Project {
    pub files: Vec<File>,
}

impl Project {
    pub fn new(files: Vec<PathBuf>) -> Result<Self, std::io::Error> {
        let mut file_vec: Vec<File> = vec![];
        for file in files {
            match File::new(file.clone()) {
                Ok(new_file) => file_vec.push(new_file),
                Err(e) => eprintln!("Error processing file {:?}: {}", file, e),
            }
        }
        Ok(Project { files: file_vec })
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct File {
    /// The path related with this file
    pub file_path: PathBuf,

    /// The name of the file
    pub filename: String,

    /// size in Bytes
    pub size: u64,

    /// Read, write, execution, etc...
    pub permissions: Permissions,

    /// data within the file
    pub content: String,
}

#[allow(dead_code)]
impl File {
    pub fn new(file_path: PathBuf) -> Result<Self, std::io::Error> {
        let content = fs::read_to_string(&file_path)?;
        let filename = file_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap_or("Unknown")
            .to_string();
        let metadata = file_path.metadata().unwrap();
        let size = metadata.size();
        let permissions = metadata.permissions();

        Ok(File {
            file_path,
            filename,
            size,
            permissions,
            content,
        })
    }

    pub fn new_buffer(file_path: PathBuf) -> Result<Self, std::io::Error> {
        let file = fs::File::open(&file_path)?;
        let mut reader = BufReader::new(file);

        let mut content = String::new();
        reader.read_to_string(&mut content)?;

        let filename = file_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap_or("Unknown")
            .to_string();
        let metadata = file_path.metadata()?;
        let size = metadata.size();
        let permissions = metadata.permissions();

        Ok(File {
            file_path,
            filename,
            size,
            permissions,
            content,
        })
    }
}
