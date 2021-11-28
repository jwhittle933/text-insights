use insights::cli::{self, env::ENV};
use insights::database::{self, data, query};
use data::{lsj::LSJ};
use query::Insert;

use std::fs::{self, File};
use dirs;

const GREEK: &str = "lexica/grc/lsj/dictionary.json";

fn main() -> std::io::Result<()> {
    let env = ENV::read();
    let mut client = database::connect(env.db_host, env.db_user, env.db_database)
        .expect("failed to connect to database");

    let home = dirs::home_dir()
        .expect("failed to lookup $HOME")
        .join("Development");

    let mut greek = File::open(home.join(GREEK))?;
    let mut lsj = LSJ::parse_raw(&mut greek)?;

    for (k, v) in lsj {
        client.execute(
            Insert::new("lsj_temp", &["lex", "description", "html"]).as_str(),
            &[
                &k,
                &voca_rs::strip::strip_tags(&v),
                &v,
            ],
        );
    }

    Ok(())
}
