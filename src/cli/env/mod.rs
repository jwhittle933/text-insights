use std::env;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ENV {
    pub db_host: String,
    pub db_user: String,
    pub db_database: String,
}

impl ENV {
    pub fn read() -> ENV {
        envy::from_env::<ENV>().expect("Missing ENV")
    }
}

pub fn value(v: &str) -> String {
    env::var(v).expect(format!("No value set for ${}", v).as_str())
}

pub fn value_or(v: &str, default: String) -> String {
    match env::var(v) {
        Ok(val) => val,
        Err(_) => default
    }
}