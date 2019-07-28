extern crate rusqlite;

use rusqlite::{Connection, Error, OpenFlags, Statement, NO_PARAMS};

pub struct SQLite {
    databases:      String,
    pub connection: Connection,
}

pub struct SQLiteContext<'a> {
    pub conn:                  &'a Connection,
    pub create_menu_statement: Option<Statement<'a>>,
}
impl<'a> SQLiteContext<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        return SQLiteContext {
            conn,
            create_menu_statement: None,
        };
    }

    pub fn create_menu(&mut self, title: &String, body: &String, id: u8) -> Result<i64, Error> {
        if let None = &self.create_menu_statement {
            let stmt = self.conn.prepare(
                "INSERT OR REPLACE INTO cat_colors (id, title, body) VALUES (:id, :title, :body)",
            ).expect("Failed to prepare context");
            self.create_menu_statement = Some(stmt);
        };
        self.create_menu_statement
            .as_mut()
            .unwrap()
            .execute_named(&[
                (":id", &id.to_string()),
                (":title", &title),
                (":body", &body),
            ])
            .expect("Failed create menu statement");
        return Ok(self.conn.last_insert_rowid());
    }
}

impl SQLite {
    pub fn new(db_name: &str) -> SQLite {
        SQLite {
            databases: db_name.to_string(),
            //            connection: Connection::open(db_name).expect("Not open"),
            connection: Connection::open_with_flags(
                db_name,
                OpenFlags::SQLITE_OPEN_READ_WRITE
                    | OpenFlags::SQLITE_OPEN_CREATE
                    | OpenFlags::SQLITE_OPEN_SHARED_CACHE,
            )
            .expect("Not open"),
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
