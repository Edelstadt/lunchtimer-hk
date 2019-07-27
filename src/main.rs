#![feature(async_await)]
#[macro_use]
extern crate serde_derive;

use crate::lunches::store::{get_menus, init_menus, update_menus, Menu};
mod lunches;
use crate::lunches::storage::{SQLite, Storage};

#[runtime::main]
async fn main() {
    init_menus();
    let st: SQLite = SQLite::new("dbName.db");
    st.create();
    runtime::spawn(update_menus());
    loop {
        let menus: &[Menu] = get_menus();
        for x in menus.iter() {
            st.insert(x.id, x.title.clone(), x.body.clone());
        }
    }
}
