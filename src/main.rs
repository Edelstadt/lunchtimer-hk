#![feature(async_await)]
#[macro_use]
extern crate serde_derive;

use crate::lunches::store::{get_menus, init_menus, update_menus, Menu};
mod lunches;
use crate::lunches::storage::{SQLite, SQLiteContext, Storage};

#[runtime::main]
async fn main() {
    init_menus();
    let st: SQLite = SQLite::new("dbName.db");
    st.create();
    let mut context = SQLiteContext::new(&st.connection);

    runtime::spawn(update_menus());
    loop {
        context
            .conn
            .execute_batch("BEGIN TRANSACTION;")
            .expect("Failed begin transaktion");
        let menus: &[Menu] = get_menus();
        for x in menus.iter() {
            context
                .create_menu(&x.title.clone(), &x.body.clone(), x.id)
                .expect("Failed create menus");
            //            st.insert(x.id, x.title.clone(), x.body.clone());
        }
        context
            .conn
            .execute_batch("COMMIT TRANSACTION;")
            .expect("Failed commit transaction");
    }
}
