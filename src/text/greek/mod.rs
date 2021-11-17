#[derive(Debug)]
pub struct GreekText {
    t: String,
}

impl GreekText {
    pub fn parse(t: String) -> GreekText {
        GreekText { t }
    }
}

impl std::fmt::Display for GreekText {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.t)
    }
}