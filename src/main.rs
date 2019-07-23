#![feature(async_await)]
mod lunches;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate nickel;

use lunches::{r#impl as menus};
use std::sync::mpsc::{channel};
use nickel::{Nickel, HttpRouter, Request, Response, MiddlewareResult};
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use crate::lunches::store::{Menu, get_menus, set_menus, init_menus};

#[runtime::main]
async fn main() {
    init_menus();
    let mut server = Nickel::new();
    runtime::spawn(update_menus());

    server.get("**", middleware! { |_, response|
        let mut data2 = HashMap::new();
        data2.insert("menus", get_menus());
        return response.render("assets/lunches.tpl", &data2)
    });

    server.listen("0.0.0.0:8000").expect("");
}

async fn update_menus() {
    loop {
        let (tx, rx) = channel();

        let c = 2;
        runtime::spawn(menus::fascila(tx.clone()));
        runtime::spawn(menus::u_kocoura(tx.clone()));

        let mut data: Vec<Menu> = vec![];
        for _ in 0..c {
            data.push(rx.recv().unwrap());
        }
        set_menus(data);

        thread::sleep(Duration::from_secs(3600)); // TODO proper timer
    }
}
