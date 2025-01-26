pub mod error;
pub mod project;

use clap::Parser;
use error::CLIError;
use project::Project;
use serde::Serialize;
use serde_json::Value;
use std::{path::Path, vec::IntoIter};

/// # Atlas_lang Compiler
///
/// This scruct contain all the arguments accepted by the CLI app
#[derive(Parser, Debug, Serialize)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Name of the file to compile
    #[arg(default_value_t = String::from("./main.atlas"))]
    pub root_file: String,

    /// Output directory and Filename
    #[arg(short, long, default_value_t = String::from("./"))]
    pub output: String,

    /// Select between compiling or directly running the file
    #[arg(short, long, default_value_t = false)]
    pub compile: bool,

    /// Create the config file
    #[arg(short, long, default_value_t = false)]
    pub init: bool,

    /// Additional flag that depends on --init
    #[arg(short, default_value_t = false)]
    pub y: bool,
}

impl IntoIterator for Args {
    type Item = (String, Value); // Each item will be (field's name, Serialize value)
    type IntoIter = IntoIter<(String, Value)>; // Key-Value pair iterator

    fn into_iter(self) -> Self::IntoIter {
        // Let's serialize the 'Args' to a JSON Object
        let json_value = serde_json::to_value(self).expect("Failed to serialize");

        // If it's a JSON Object We transform their key-value in a tuple
        if let Value::Object(map) = json_value {
            map.into_iter()
                .map(|(key, value)| (key, value))
                .collect::<Vec<_>>()
                .into_iter()
        } else {
            // If it's not a JSON Object. Let's return a Empty iterator
            vec![].into_iter()
        }
    }
}

impl Args {
    /// Check if the introduced file doesn't is empty because in this case there isn't a file to read.
    pub fn verify(&self) -> Result<Project, CLIError> {
        if !self.root_file.trim().is_empty() {
            let file = self.verify_file();
            if let Err(error) = file {
                return Err(error);
            }

            let file = file.unwrap();
            return Ok(file);
        }
        Ok(Project { files: vec![] })
    }

    /// Check if the alleged file exist in this path, it's a directory or it has a correct extension.
    ///
    /// Otherwise, If it has a correct extension and it's a file so let's continue with the parse.
    fn verify_file(&self) -> Result<Project, CLIError> {
        let path = Path::new(&self.root_file);

        if !path.is_file() {
            return Err(CLIError::InvalidPath);
        }
        if let Some(ext) = path.extension() {
            if ext != "atlas" && ext != ".atl" {
                return Err(CLIError::InvalidExtension);
            }
        } else {
            return Err(CLIError::InvalidExtension);
        }
        let project = Project::new(vec![path.to_path_buf()]);
        return match project {
            Ok(pr) => Ok(pr),
            Err(error) => panic!("{error}"),
        };
    }
}
