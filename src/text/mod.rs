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
}