pub mod error;
pub mod project;

use clap::Parser;
use error::CLIError;
use project::Project;
use serde::Serialize;
use serde_json::Value;
use std::{path::Path, vec::IntoIter};

/// Atlas_lang Compiler
#[derive(Parser, Debug, Serialize)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Name of the file to compile
    #[arg(default_value_t = String::from("./main.atlas"))]
    pub file: String,

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
    type Item = (String, Value); // Cada ítem será (nombre del campo, valor serializado)
    type IntoIter = IntoIter<(String, Value)>; // Iterador de pares clave-valor

    fn into_iter(self) -> Self::IntoIter {
        // Serializamos el `Args` a un objeto JSON
        let json_value = serde_json::to_value(self).expect("Failed to serialize");

        // Si es un objeto JSON, convertimos sus claves y valores en pares
        if let Value::Object(map) = json_value {
            map.into_iter()
                .map(|(key, value)| (key, value))
                .collect::<Vec<_>>()
                .into_iter()
        } else {
            // Si no es un objeto JSON, devolvemos un iterador vacío
            vec![].into_iter()
        }
    }
}

impl Args {
    pub fn verify(&self) -> Result<Project, CLIError> {
        if !self.file.trim().is_empty() {
            let file = self.verify_file();
            if let Err(error) = file {
                return Err(error);
            }

            let file = file.unwrap();
            return Ok(file);
        }
        Ok(Project { files: vec![] })
    }

    fn verify_file(&self) -> Result<Project, CLIError> {
        let path = Path::new(&self.file);

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
