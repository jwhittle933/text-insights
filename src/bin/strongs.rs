use insights::cli::env::ENV;
use insights::database::{self, data, query};
use data::strongs::{Hebrew, Greek};
use query::Insert;

use std::fs::File;
use dirs;

const HEBREW: &str = "lexica/heb/strongs-hebrew-dictionary.json";
const GREEK: &str = "lexica/grc/strongs-greek-dictionary.json";

fn main() -> std::io::Result<()> {
    let env = ENV::read();
    let mut conn = database::connect(env.db_host, env.db_user, env.db_database)
        .expect("failed to connect to database");

    let home = dirs::home_dir()
        .expect("could not get $HOME dir")
        .join("Development");

    let hb = Hebrew::from(
        &mut File::open(home.join(HEBREW)).expect("failed to open Strongs Hebrew")
    );
    let gk = Greek::from(
        &mut File::open(home.join(GREEK)).expect("failed to open Strongs Greek")
    );

    println!("Inserting Hebrew...");
    for (k, v) in hb {
        conn.execute(
            Insert::new(
                "strongs",
                &["id", "pre", "lemma", "transliteration", "pronunciation", "derivation", "definition"],
            ).as_str(),
            &[
                &k,
                &"H",
                &v.lemma,
                &v.xlit,
                &v.pron,
                &v.derivation.unwrap_or("".to_owned()),
                &v.strongs_def,
            ],
        ).expect("failed on Hebrew insert");
    }

    println!("Inserting Greek...");
    for (k, v) in gk {
        conn.execute(
            Insert::new(
                "strongs",
                &["id", "pre", "lemma", "transliteration", "derivation", "definition"],
            ).as_str(),
            &[
                &k,
                &"G",
                &v.lemma,
                &v.translit,
                &v.derivation.unwrap_or("".to_owned()),
                &v.strongs_def.unwrap_or("".to_owned()),
            ],
        ).expect("failed on Greek insert");
    }

    Ok(())
}