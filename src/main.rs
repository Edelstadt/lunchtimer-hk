#![feature(async_await)]
mod lunches;

extern crate serde;
#[macro_use]
extern crate serde_derive;

use lunches::fetch;
use std::sync::mpsc::{channel};
use nickel::{Nickel, HttpRouter, Request, Response, MiddlewareResult};
use std::collections::HashMap;

#[derive(Serialize)]
struct Menu {
    id: u8,
    title: String,
    body: String,
}

#[runtime::main]
async fn main() {
    // Server
    let mut server = Nickel::new();
    server.get("**", lunch);
    server.listen("0.0.0.0:8000").expect("");



    // Example of async
    let (tx, rx) = channel();

    let c = 10;
    for i in 0..c { // Spawn jobs
        runtime::spawn(fetch(tx.clone()));
    }

    for i in 0..11 { // Await jobs
        dbg!(rx.recv());
    }
}

fn lunch<'mw>(_req: &mut Request, res: Response<'mw>) -> MiddlewareResult<'mw> {
    let menus = vec![
        Menu{id: 1, title: String::from("Restaurant"), body: String::from("Menu of A ...")},
        Menu{id: 2, title: String::from("RestaurantB"), body: String::from("Menu of B ...")},
        Menu{id: 3, title: String::from("RestaurantC"), body: String::from("Menu of C ...")},
    ];

    // Example data
    let mut data = HashMap::new();
    data.insert("menus", menus);

    return res.render("src/assets/lunches.tpl", &data)
}
