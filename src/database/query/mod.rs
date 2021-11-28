pub struct Insert {}

impl Insert {
    pub fn new(table: &str, columns: &[&str]) -> String {
        let cols = Vec::from(columns);

        let mut args = Vec::new();
        for i in 1..=cols.len() {
            args.push(format!("${}", i));
        }

        format!(
            "INSERT INTO {} ({}) VALUES ({})",
            table,
            cols.join(", "),
            args.join(", "),
        )
    }
}

