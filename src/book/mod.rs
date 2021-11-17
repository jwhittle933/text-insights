use postgres::{Client, Error};

pub struct Book {
    pub id: i64,
    pub name: String,
}

impl Book {
    pub fn query<'a>(c: &mut Client, name: String) -> Result<Book, Error> {
        let row = c.query_one("SELECT * FROM books WHERE name = $1", &[&name])?;
        let id = row.get(0);

        Ok(Book { id, name })
    }
}