#![feature(async_await)]
mod lunches;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate nickel;

use crate::lunches::store::{get_menus, init_menus, set_menus, Menu};
use chrono::{Local, Timelike};
use lunches::r#impl as menus;
use nickel::{HttpRouter, Nickel};
use std::{collections::HashMap, sync::mpsc::channel, thread, time::Duration};

#[runtime::main]
async fn main() {
    init_menus();
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

    server.listen("0.0.0.0:8000").expect("");
}

async fn update_menus() {
    let mut last_hour: u32 = 99;
    loop {
        // Updates once an hour
        let curr = Local::now();
        if curr.hour() == last_hour {
            thread::sleep(Duration::from_secs(60 * 15));
            continue;
        }
        last_hour = curr.hour();

        // Fetch new data
        let (tx, rx) = channel();

        let c = 2;
        runtime::spawn(menus::fascila(tx.clone()));
        runtime::spawn(menus::u_kocoura(tx.clone()));

        let mut data: Vec<Menu> = vec![];
        for _ in 0..c {
            data.push(rx.recv().unwrap());
        }
        set_menus(data);
    }
}
