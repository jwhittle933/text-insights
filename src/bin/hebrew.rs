use std::fs::File;
use dirs;

const HEBREW: &str = "lexica/heb/BDB/DictBDB.json";

fn main() -> std::io::Result<()> {
    let home = dirs::home_dir()
        .expect("could not get $HOME dir")
        .join("Development");

    let mut hebrew = File::open(home.join(HEBREW));

    Ok(())
}
