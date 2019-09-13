use std::sync::mpsc::Sender;

use chrono::{Datelike, Utc};
use encoding::all::WINDOWS_1250;
use reqwest::Client;
use select::{
    document::Document,
    predicate::{Class, Name},
};

use crate::lunches::{
    menu::{Menu, MenuBody, MenuLine},
    store::StoreError,
};
use std::io::Read;

pub(crate) async fn fetch(tx: Sender<Result<Menu, StoreError>>, id: u16, name: String) {
    tx.send(fetch_data(id, name)).unwrap();
}

fn fetch_data(id: u16, name: String) -> Result<Menu, StoreError> {
    let c = Client::new();
    let mut res = c.get(&format!("https://menicka.cz/{}.html", id)).send()?;

    dbg!(&res);
    let mut body = String::new();
    res.read_to_string(&mut body)?;
    dbg!(2);

    let mut menu = Menu::new(&name);
    parser(&mut menu, body)?;

    Ok(menu)
}

fn parser(menu: &mut Menu, body: String) -> Result<(), StoreError> {
    let doc = Document::from_read(body.as_bytes())?;

    for node in doc.find(Class("menicka")) {
        dbg!(node);
//        let now = Utc::now();
//        let rr = format!("{}. {}.", now.day(), now.month());
//        if node.text().contains(rr.as_str()) {
//            for (i, node_dd) in node.find(Name("dd")).enumerate() {
//                match i {
//                    0 => {
//                        menu.body.push(MenuLine::Title(String::from("Polévka")));
//                        menu.body.push(MenuLine::Item(format_row(node_dd.text())?));
//                        menu.body.push(MenuLine::Title(String::from("Denní menu")));
//                    },
//                    _ => {
//                        menu.body.push(MenuLine::Item(format_row(node_dd.text())?));
//                    },
//                }
//            }
//        }
        break;
    }

    Ok(())
}

fn parse_line(row: String) -> Result<MenuBody, StoreError> {
    let mut body = MenuBody::new();
    body.label = row;
    Ok(body)
}
