use std::fs::File;
use std::io::Read;
use std::clone::Clone;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct LS {
    pub entry_type: Option<String>,
    pub key: Option<String>,
    pub main_notes: Option<String>,
    pub part_of_speech: Option<String>,
    pub title_orthography: Option<String>,
    // misspelling in the data
    pub alternative_genative: Option<Vec<String>>,
    pub declension: Option<i8>,
    pub gender: Option<String>,
    pub title_genitive: Option<String>,
    pub alternative_orthography: Option<Vec<String>>,
    pub greek_word: Option<String>,
}

impl LS {
    pub fn from(f: &mut File) -> LS {
        let mut data: String = String::new();
        f.read_to_string(&mut data)
            .expect("");

        serde_json::from_str::<LS>(&data[..])
            .expect("")
    }
}