use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use serde::Deserialize;

/*
 H8674: {
    lemma: 'תַּתְּנַי',
    xlit: 'Tattᵉnay',
    pron: "tat-ten-ah'-ee",
    derivation: 'of foreign derivation;',
    strongs_def: 'Tattenai, a Persian',
    kjv_def: 'Tatnai.'
  }
*/

#[derive(Debug, Deserialize)]
pub struct Hebrew {
    pub lemma: String,
    pub xlit: String,
    pub pron: String,
    pub derivation: Option<String>,
    pub strongs_def: String,
}

impl Hebrew {
    pub fn from(f: &mut File) -> HashMap<String, Hebrew> {
        let mut data: String = String::new();
        f.read_to_string(&mut data)
            .expect("failed to read strongs hebrew file");

        serde_json::from_str::<HashMap<String, Hebrew>>(&data[..])
            .expect("failed to parse json into Hebrew")
    }
}

/*
G489: {
    derivation: 'from a compound of G473 (ἀντί) and G3408 (μισθός);',
    strongs_def: ' requital, correspondence',
    kjv_def: 'recompense',
    translit: 'antimisthía',
    lemma: 'ἀντιμισθία'
  }
*/
#[derive(Debug, Deserialize)]
pub struct Greek {
    pub lemma: String,
    pub derivation: Option<String>,
    pub strongs_def: Option<String>,
    pub translit: String,
}

impl Greek {
    pub fn from(f: &mut File) -> HashMap<String, Greek> {
        let mut data: String = String::new();
        f.read_to_string(&mut data)
            .expect("failed to read strongs greek file");

        serde_json::from_str::<HashMap<String, Greek>>(&data[..])
            .expect("failed to parse json into Greek")
    }
}
