mod cli;
mod text;
mod book;

use postgres::{Client, NoTls};
use text::Texts;
use cli::env::ENV;
use crate::text::{GreekText, HebrewText};

fn main() -> std::io::Result<()> {
    let args = cli::parse_flags();
    let env = ENV::read();
    let mut client = Client::connect(
        format!("host={} user={} dbname={}", env.db_host, env.db_user, env.db_database).as_str(),
        NoTls
    ).expect("Failed to connect to psql");

    let lxx = GreekText::query(&mut client, &args.book, args.chapter, args.verse).expect("failed to query greek text");
    let mt = HebrewText::query(&mut client, &args.book, args.chapter, args.verse).expect("failed to query hebrew text");
    println!("{}", lxx);
    println!("{}", mt);

    return Ok(());
}
