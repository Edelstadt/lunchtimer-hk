#![feature(async_await)]
mod lunches;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate nickel;

use crate::lunches::store::{get_menus, update_menus};
use nickel::{HttpRouter, Nickel};
use std::collections::HashMap;

#[runtime::main]
async fn main() {
    let mut server = Nickel::new();
    runtime::spawn(update_menus());

    server.get(
        "**",
        middleware! { |_, response|
            let mut data2 = HashMap::new();
            data2.insert("menus", get_menus());
            return response.render("assets/lunches.tpl", &data2)
        },
    );

    server.listen("127.0.0.7:8000").expect("");
}
