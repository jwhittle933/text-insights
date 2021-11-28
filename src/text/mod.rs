mod greek;
mod hebrew;

pub use greek::GreekText;
pub use hebrew::HebrewText;

pub struct Texts {
    pub lxx: GreekText,
    pub mt: HebrewText,
}

impl Texts {
}