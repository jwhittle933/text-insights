use itertools::Itertools;
use std::fs::File;

use anyhow::Context;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct DataFiles(pub Vec<DataFile>);

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct DataFile {
    pub book: String,
    #[serde(rename = "content")]
    pub chapters: Vec<Chapter>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Chapter {
    pub chapter: usize,
    #[serde(rename = "content")]
    pub verses: Vec<Verse>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Verse {
    pub verse: usize,
    pub text: String,
}

impl DataFile {
    pub fn new_from_file(path: &str) -> anyhow::Result<Self> {
        open_json::<Self>(path)
    }
}

impl DataFiles {
    pub fn new_from_dir(data_dir: &str) -> anyhow::Result<Self> {
        let mut entries = crate::fs::read_entries(data_dir)?;
        entries.sort();

        let entries: Vec<DataFile> = entries
            .iter()
            .map(|path| DataFile::new_from_file(path.as_str()))
            .try_collect()?;

        Ok(Self(entries))
    }
}

fn open_json<T: serde::de::DeserializeOwned>(path: &str) -> anyhow::Result<DataFile> {
    let file = File::open(path).context("failed to open file")?;
    Ok(
        simd_json::from_reader(&file)
            .with_context(|| format!("failed to parse json for {path}"))?,
    )
}
