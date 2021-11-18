mod cli;
mod text;
mod book;

use postgres::{Client, NoTls};
use text::Texts;
use book::Book;
use cli::env;
use crate::text::{GreekText, HebrewText};

fn main() -> std::io::Result<()> {
    let args = cli::parse_flags();
    let mut client = Client::connect(
        format!("host={} user={} dbname={}", env::value("DB_HOST"), env::value("DB_USER"), env::value("DB_DATABASE")).as_str(),
        NoTls
    ).expect("Failed to connect to psql");

    let book = Book::query(&mut client, args.book).expect("Failed to query for book");
    let lxx = GreekText::query(&mut client, &book, args.chapter, args.verse).expect("failed to query greek text");
    let mt = HebrewText::query(&mut client, &book, args.chapter, args.verse).expect("failed to query hebrew text");
    println!("{}", lxx);
    println!("{}", mt);

    return Ok(());
}
