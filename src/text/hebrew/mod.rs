pub mod vowels;

extern crate unicode_segmentation;

use postgres::{Client, Error};
use unicode_segmentation::UnicodeSegmentation;
use crate::book::Book;
pub use vowels::is_vowel;

const QUERY: &str = "SELECT * FROM mt WHERE book_id = (SELECT id FROM books WHERE name = $1) AND chapter = $2 AND verse = $3";

#[derive(Debug)]
pub struct HebrewText {
    id: i64,
    text: String,
}

impl HebrewText {
    pub fn query(c: &mut Client, book: &String, chapter: i32, verse: i32) -> Result<HebrewText, Error> {
        let q = c.query_one(&QUERY.to_owned(), &[book, &chapter, &verse])?;

        let id: i64 = q.get("id");
        let text = q.get("text");
        Ok(HebrewText { id, text })
    }
}

impl std::fmt::Display for HebrewText {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let s: String = self.text.graphemes(true).rev().collect();
        write!(f, "{}", s)
    }
}
