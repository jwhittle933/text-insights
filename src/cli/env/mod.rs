use std::env;

pub fn value(v: &str) -> String {
    env::var(v).expect(format!("No value set for ${}", v).as_str())
}

pub fn value_or(v: &str, default: String) -> String {
    match env::var(v) {
        Ok(val) => val,
        Err(_) => default
    }
}