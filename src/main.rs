#![feature(async_await)]
#[macro_use]
extern crate nickel;
#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;

use nickel::{HttpRouter, Nickel};

use crate::lunches::store::{get_menus, update_menus};

mod lunches;

#[runtime::main]
async fn main() {
    //runtime::spawn(update_menus());
    asd().await;
}

async fn asd() {
    let mut server = Nickel::new();

    server.get(
        "**",
        middleware! { |_, response|
            update_menus()
            let mut data2 = HashMap::new();
            data2.insert("menus", get_menus());
            return response.render("assets/lunches.tpl", &data2)
        },
    );

    server.listen("0.0.0.0:8000").expect("Server fail");
}
