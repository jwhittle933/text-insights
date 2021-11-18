use postgres::{Client, Error};
use crate::book::Book;

#[derive(Debug)]
pub struct GreekText {
    id: i64,
    text: String,
    book_id: i64
}

const QUERY: &str = "SELECT * FROM lxx WHERE book_id = $1 AND chapter = $2 AND verse = $3";

impl GreekText {
    pub fn from(id: i64, text: String, book_id: i64) -> GreekText {
        GreekText { id, text, book_id }
    }

    pub fn query(c: &mut Client, book: &Book, chapter: i32, verse: i32) -> Result<GreekText, Error> {
        let q = c.query_one(&QUERY.to_owned(), &[&book.id, &chapter, &verse])?;

        let id: i64 = q.get("id");
        let text = q.get("text");
        let book_id = q.get("book_id");
        Ok(GreekText{ id, text, book_id })
    }
}

impl std::fmt::Display for GreekText {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.text)
    }
}