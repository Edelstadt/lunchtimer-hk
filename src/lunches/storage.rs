extern crate rusqlite;

use rusqlite::{Connection, NO_PARAMS};

pub struct SQLite {
    databases:  String,
    connection: Connection,
}

impl SQLite {
    pub fn new(db_name: &str) -> SQLite {
        SQLite {
            databases:  db_name.to_string(),
            connection: Connection::open(db_name).expect("Not open"),
        }
    }
}
pub trait Storage {
    fn insert(&self, id: u8, title: String, body: String);
    fn create(&self);
}

impl Storage for SQLite {
    fn insert(&self, id: u8, title: String, body: String) {
        self.connection
            .execute(
                "INSERT OR REPLACE INTO cat_colors (id, title, body) values (?1, ?2, ?3)",
                &[&id.to_string(), &title, &body],
            )
            .expect("Not insert");
    }

    fn create(&self) {
        self.connection
            .execute(
                "create table if not exists cat_colors (
             id integer primary key,
             title text not null unique,
             body text
         )",
                NO_PARAMS,
            )
            .expect("Not creater");
    }
}
