use std::{
    fs,
    io::{Error, Read},
    path::Path,
};

pub struct File {
    name: String,
    path: String,
    size: u64,
    extension: String,
    file: Option<fs::File>,
}

impl File {
    pub fn open(file_name: &str) -> Result<Self, Error> {
        let path = Path::new(file_name);
        let file = fs::File::open(&path)?;

        Ok(File {
            name: path.file_name().unwrap().to_str().unwrap().to_string(),
            path: path.to_str().unwrap().to_string(),
            size: path.metadata().unwrap().len(),
            extension: path.extension().unwrap().to_str().unwrap().to_string(),
            file: Some(file),
        })
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_content(&self) -> String {
        let mut file = self.file.as_ref().unwrap();
        let mut buf = String::new();
        file.read_to_string(&mut buf).unwrap();
        buf
    }

    pub fn get_size(&self) -> u64 {
        self.size
    }

    pub fn get_extension(&self) -> &str {
        &self.extension
    }
}
