#![feature(async_await)]
mod lunches;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use] extern crate nickel;
use lunches::{fetch, r#impl as menus};
use std::sync::mpsc::{channel};
use nickel::{Nickel, HttpRouter, Request, Response, MiddlewareResult};
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

#[derive(Serialize)]
pub struct Menu {
    id: u8,
    title: String,
    body: String,
}

//static mut p_data: Option<&Vec<Menu>> = None;
#[runtime::main]
async fn main() {
    let mut server = Nickel::new();

    let mut data: Vec<Menu> = Vec::new();
//    let p_data: &'static Vec<Menu> = &data;
    runtime::spawn(update_menus(&mut data));

//    server.get("**", lunch);
    server.get("**", middleware! { |_, response|
        let mut data2 = HashMap::new();
        data2.insert("menus", &data);
        return response.render("assets/lunches.tpl", &data2)
    });

    server.listen("0.0.0.0:8000").expect("");
}

//fn lunch<'mw>(_req: &mut Request, res: Response<'mw>) -> MiddlewareResult<'mw> {
//    let menus = vec![
//        Menu{id: 1, title: String::from("Restaurant"), body: String::from("Menu of A ...")},
//        Menu{id: 2, title: String::from("RestaurantB"), body: String::from("Menu of B ...")},
//        Menu{id: 3, title: String::from("RestaurantC"), body: String::from("Menu of C ...")},
//    ];
//
//    // Example data
//    let mut data = HashMap::new();
//    data.insert("menus", menus);
//
//    return res.render("assets/lunches.tpl", &data)
//}

async fn update_menus(data: &'static mut Vec<Menu>) {
    loop {
        // TODO
        let (tx, rx) = channel();
        let mut list: Vec<Menu> = Vec::new();

        let c = 2; // Spawn jobs
        runtime::spawn(menus::fascila(tx.clone()));
        runtime::spawn(menus::u_kocoura(tx.clone()));

        let mut tmp_data: Vec<Menu> = vec![];
        for _ in 0..c { // Await jobs
            tmp_data.push(rx.recv().unwrap());
        }
        data.clear();
        data.append(&mut tmp_data);

        thread::sleep(Duration::from_secs(3600)); // TODO proper timer
    }
}
