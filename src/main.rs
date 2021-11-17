mod cli;
mod text;
mod book;

use postgres::{Client, NoTls};
use text::Texts;
use book::Book;

fn main() -> std::io::Result<()> {
    let args = cli::parse_flags();
    let mut client = Client::connect("host=localhost user=jonathanwhittle dbname=texts", NoTls).expect("Failed to connect to psql");
    let book = Book::query(&mut client, args.book).expect("Failed to query for book");
    let text = Texts::query(&mut client, book, args.chapter, args.verse).expect("Failed to query for texts");

    println!("{}", text.lxx);
    println!("{}", text.mt);

    return Ok(())
}
