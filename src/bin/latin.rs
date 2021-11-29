use std::fs::File;
use std::path::PathBuf;
use std::thread::{self, JoinHandle};
use dirs;
use insights::database::data::ls::LS;
use insights::coreio;

const LATIN: &str = "Development/lexica/lat/ls/*.json";

fn main() -> std::io::Result<()> {
    let globs = dirs::home_dir()
        .expect("could not get $HOME dir")
        .join(LATIN)
        .to_str()
        .expect("failed to covert PathBuf to &str")
        .to_owned();

    let mut out: Vec<LS> = Vec::new();
    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    for file in glob::glob(globs.as_str()).unwrap() {
        match file {
            Ok(p) => {
                handles.push(
                    thread::spawn(|| {
                        insert(p)
                            .expect("failed in insert collection");
                    })
                );
            }
            Err(e) => println!("{}", e)
        }
    }

    for h in handles {
        h.join().unwrap();
    }

    Ok(())
}

pub fn insert(p: PathBuf) -> std::io::Result<()> {
    let ls = coreio::json::deserialize_file::<Vec<LS>>(&mut File::open(p)?);
    println!("Size: {}", ls.len());
    Ok(())
}