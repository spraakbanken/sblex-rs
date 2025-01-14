use std::{
    fs,
    io::{self, BufRead},
    path::{Path, PathBuf},
};

use tracing::instrument;

pub trait Morphology: Clone + Send + Sync + 'static {
    fn lookup(&self, fragment: &str) -> Result<Option<Vec<u8>>, LookupError>;
    fn lookup_with_cont(&self, fragment: &str) -> Result<Vec<u8>, LookupError>;
}

pub trait MorphologyBuilder {
    fn insert(&mut self, word: &str, value: String) -> Result<(), MorphologyBuilderError>;
    fn finish(&mut self) -> Result<(), MorphologyBuilderError>;
}
pub type BoxDynError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum MorphologyBuilderError {
    #[error("word '{word}' already exists")]
    Duplicate { word: String },
    #[error("Failed to open file '{path}'")]
    CouldNotOpenFile {
        path: PathBuf,
        #[source]
        error: io::Error,
    },
    #[error("Failed to read line {line_number} from '{path}'")]
    CouldNotReadLine {
        line_number: usize,
        path: PathBuf,
        #[source]
        error: io::Error,
    },
    #[error("Failed to deserialize data")]
    FailedToDeserialize(#[from] serde_json::Error),
    #[error("unknown error")]
    Unknown(BoxDynError),
}
#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum LookupError {
    #[error("unknown error")]
    Unknown(BoxDynError),
}

#[instrument(skip(path, builder))]
pub fn build_from_path<P: AsRef<Path>>(
    builder: &mut dyn MorphologyBuilder,
    path: P,
) -> Result<(), MorphologyBuilderError> {
    let file = fs::File::open(path.as_ref()).map_err(|error| {
        MorphologyBuilderError::CouldNotOpenFile {
            path: path.as_ref().to_path_buf(),
            error,
        }
    })?;
    let reader = io::BufReader::new(file);
    for (line_number, line) in reader.lines().enumerate() {
        let line = line.map_err(|error| MorphologyBuilderError::CouldNotReadLine {
            line_number,
            path: path.as_ref().to_path_buf(),
            error,
        })?;
        let j: serde_json::Value = serde_json::from_str(&line)?;
        let w = &j["word"];
        let a = serde_json::json!({
            "gf": j["head"],
            "id": j["id"],
            "pos": j["pos"],
            "is": j["inhs"],
            "msd": j["param"],
            "p": j["p"]
        });
        builder.insert(w.as_str().expect("fm.morphology word"), a.to_string())?;
    }
    Ok(())
}
