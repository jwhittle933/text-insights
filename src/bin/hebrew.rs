use std::fs::File;

use dirs;

use insights::cli::env::ENV;
use insights::database::{
    self,
    data::bdb::Hebrew,
    query::{Insert, Select},
};
use insights::coreio;


const HEBREW: &str = "lexica/heb/BDB/DictBDB.json";

fn main() -> std::io::Result<()> {
    let env = ENV::read();
    let mut client = database::connect(env.db_host, env.db_user, env.db_database)
        .expect("failed to connect to database");

    let home = dirs::home_dir()
        .expect("could not get $HOME dir")
        .join("Development");

    let entries = coreio::json::deserialize_file::<Vec<Hebrew>>(&mut File::open(HEBREW)?);
    for entry in entries {
        // TODO: select from strongs by the `top`
        ///
    }

    println!("{}", Insert::new("ls_temp", &["test", "another"]));
    println!("{}", Select::new("ls_temp", &["test", "another"]));
    Ok(())
}
