use std::fs::File;
use dirs;

const LATIN: &str = "lexica/lat/ls/lat_a.json";

fn main() -> std::io::Result<()> {
    let home = dirs::home_dir()
        .expect("could not get $HOME dir")
        .join("Development");

    let mut latin = File::open(home.join(LATIN));

    Ok(())
}