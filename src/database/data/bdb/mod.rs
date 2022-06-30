use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Hebrew {
    pub top: String,
    pub def: String,
}