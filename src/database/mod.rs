pub mod data;
pub mod query;
use postgres::{Client, NoTls, Error};

pub fn connect(host: String, user: String, db: String) -> Result<Client, Error> {
    Client::connect(
        format!("host={} user={} dbname={}", host, user, db).as_str(),
        NoTls,
    )
}
