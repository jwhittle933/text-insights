use insights::cli::{self, env::ENV};
use insights::text;
use insights::book;
use insights::text::{GreekText, HebrewText};
use insights::database;

fn main() -> std::io::Result<()> {
    let args = cli::parse_flags();
    let env = ENV::read();
    let mut client = database::connect(env.db_host, env.db_user, env.db_database).expect("Failed to connect to database");

    let lxx = GreekText::query(&mut client, &args.book, args.chapter, args.verse).expect("failed to query greek text");
    let mt = HebrewText::query(&mut client, &args.book, args.chapter, args.verse).expect("failed to query hebrew text");
    println!("{}", lxx);
    println!("{}", mt);

    return Ok(());
}
