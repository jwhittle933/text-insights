use std::fs::File;
use std::path::PathBuf;
use std::thread::{self, JoinHandle};

use dirs;
use postgres::Client;

use insights::coreio;
use insights::cli::env::ENV;
use insights::database::{self, data, query::Insert};
use data::ls::LS;

const LATIN: &str = "Development/lexica/lat/ls/*.json";

fn main() -> std::io::Result<()> {
    let globs = dirs::home_dir()
        .expect("could not get $HOME dir")
        .join(LATIN)
        .to_str()
        .expect("failed to covert PathBuf to &str")
        .to_owned();

    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    for file in glob::glob(globs.as_str()).unwrap() {
        match file {
            Ok(p) => {
                handles.push(
                    thread::spawn(|| {
                        let env = ENV::read();
                        let mut client = database::connect(env.db_host, env.db_user, env.db_database)
                            .expect("failed to connect to database");

                        insert(p, &mut client)
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

pub fn insert(p: PathBuf, c: &mut Client) -> std::io::Result<()> {
    let entries = coreio::json::deserialize_file::<Vec<LS>>(&mut File::open(p)?);
    for entry in entries {
        c.execute(
            Insert::new(
                "ls_temp",
                &[
                    "entry_type",
                    "key",
                    "main_notes",
                    "pos",
                    "title_orthography",
                    "alternative_genitive",
                    "declension",
                    "gender",
                    "title_genitive",
                    "alternative_orthography",
                    "greek_word"
                ],
            ).as_str(),
            &[
                &entry.entry_type.unwrap_or("".to_owned()),
                &entry.key.unwrap_or("".to_owned()),
                &entry.main_notes.unwrap_or("".to_owned()),
                &entry.part_of_speech.unwrap_or("".to_owned()),
                &entry.title_orthography.unwrap_or("".to_owned()),
                &entry.alternative_genative.unwrap_or(Vec::new()),
                &entry.declension.unwrap_or(0),
                &entry.gender.unwrap_or("".to_owned()),
                &entry.title_genitive.unwrap_or("".to_owned()),
                &entry.alternative_orthography.unwrap_or(Vec::new()),
                &entry.greek_word.unwrap_or("".to_owned()),
            ],
        ).expect("failed to insert");
    }

    Ok(())
}