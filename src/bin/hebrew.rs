use insights::cli::{self, env::ENV};
use insights::database::{self, data, query};
use data::{lsj::LSJ};
use query::Insert;

use std::fs::File;
use dirs;

const HEBREW: &str = "lexica/heb/BDB/DictBDB.json";

fn main() -> std::io::Result<()> {
    let env = ENV::read();
    let mut client = database::connect(env.db_host, env.db_user, env.db_database)
        .expect("failed to connect to database");

    let home = dirs::home_dir()
        .expect("could not get $HOME dir")
        .join("Development");

    let mut hebrew = File::open(home.join(HEBREW))?;
    let mut greek = File::open(home.join(HEBREW))?;

    Ok(())
}
