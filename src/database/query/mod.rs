pub trait Query {
    fn new(table: &str, columns: &[&str]) -> String;
}

pub struct Insert {}

impl Insert {
    pub fn new(table: &str, columns: &[&str]) -> String {
        let cols = Vec::from(columns);

        format!(
            "INSERT INTO {} ({}) VALUES ({})",
            table,
            cols.join(", "),
            cols
                .iter()
                .enumerate()
                .map(|(i, _)| format!("${}", i + 1))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

pub struct Select {}

impl Select {
    pub fn new(table: &str, columns: &[&str]) -> String {
        format!(
            "SELECT from {} WHERE {}",
            table,
            Vec::from(columns)
                .iter()
                .enumerate()
                .map(|(i, col)| format!("{} = ${}", col, i + 1))
                .collect::<Vec<String>>()
                .join(" AND ")
        )
    }
}