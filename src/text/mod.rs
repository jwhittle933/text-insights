mod greek;
mod hebrew;

use postgres::{Client, Error};
use crate::book::Book;
pub use greek::GreekText;
pub use hebrew::HebrewText;

pub struct Texts {
    pub lxx: GreekText,
    pub mt: HebrewText,
}

impl Texts {
    pub fn query(c: &mut Client, book: &Book, chapter: i32, verse: i32) -> Result<Texts, Error> {
        let lxx: String = c.query_one("SELECT * FROM lxx WHERE book_id = $1 AND chapter = $2 AND verse = $3", &[&book.id, &chapter, &verse])?.get("text");
        let mt: String = c.query_one("SELECT * FROM mt WHERE book_id = $1 AND chapter = $2 AND verse = $3", &[&book.id, &chapter, &verse])?.get("text");

        Ok(Texts { lxx: GreekText::from(1, lxx, 12), mt: HebrewText::parse(1, mt) })
    }
}