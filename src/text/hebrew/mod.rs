pub mod vowels;

extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;
pub use vowels::is_vowel;

#[derive(Debug)]
pub struct HebrewText {
    t: String,
}

impl HebrewText {
    pub fn parse(t: String) -> HebrewText {
        HebrewText { t }
    }
}

impl std::fmt::Display for HebrewText {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let s: String = self.t.graphemes(true).rev().collect();
        write!(f, "{}", s)
    }
}
