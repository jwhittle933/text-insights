use std::fs::File;
use std::io::Read;
use serde::de::DeserializeOwned;

pub fn deserialize_file<T>(f: &mut File) -> T where T: DeserializeOwned {
    let mut data = String::new();
    f.read_to_string(&mut data)
        .expect("failed to read file");

    serde_json::from_str::<T>(data.as_str())
        .expect("failed to deserialize data")
}
