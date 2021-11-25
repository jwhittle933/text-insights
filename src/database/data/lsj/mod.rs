use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::Read;
use std::vec::Drain;
use serde_json;
use serde_json::map::Keys;
use serde_json::Result as JSONResult;

#[derive(Debug)]
pub struct LSJ {
    pub raw: HashMap<String, String>,
}

impl LSJ {
    pub fn from(f: &mut File) -> LSJ {
        let raw = LSJ::parse_raw(f).expect("failed to parse raw json");

        LSJ { raw }
    }

    pub fn parse_raw(f: &mut File) -> JSONResult<HashMap<String, String>> {
        let mut data: String = String::new();
        f.read_to_string(&mut data).expect("failed to read file");

        let map: JSONResult<HashMap<String, String>> = serde_json::from_str(&data[..]);
        map
    }

    pub fn has(self, key: &str) -> bool {
        self.raw.contains_key(key)
    }

    pub fn keys(&self) -> Vec<String> {
        let mut v = Vec::new();
        for k in self.raw.keys() {
            v.push(String::from(k))
        }

        v
    }

    pub fn values(&self) -> Vec<String> {
        let mut v = Vec::new();
        for val in self.raw.values() {
            v.push(String::from(val))
        }

        v
    }
}