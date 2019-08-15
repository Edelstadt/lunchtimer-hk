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

pub(crate) async fn fetch(tx: Sender<Result<Menu, StoreError>>) {
    tx.send(fetch_data()).unwrap();
}

fn fetch_data() -> Result<Menu, StoreError> {
    let c = Client::new();
    let mut res = c.get("https://naplavkabistro.cz/").send()?;

    let mut menu = Menu::new("Náplavka");
    naplavka_parser(&mut menu, res.text_with_charset(WINDOWS_1250.name)?)
        .expect("Náplavka - parse error");

    Ok(menu)
}

fn naplavka_parser(menu: &mut Menu, body: String) -> Result<(), StoreError> {
    let doc = Document::from_read(body.as_bytes())?;
    for node in doc.find(Class("daily_offer")) {
        let now = Utc::now();
        let rr = format!("{}. {}.", now.day(), now.month());
        if node.text().contains(rr.as_str()) {
            for (i, node_dd) in node.find(Name("dd")).enumerate() {
                match i {
                    0 => {
                        menu.body.push(MenuLine::Title(String::from("Polévka")));
                        menu.body.push(MenuLine::Item(format_row(node_dd.text())?));
                        menu.body.push(MenuLine::Title(String::from("Denní menu")));
                    },
                    _ => {
                        menu.body.push(MenuLine::Item(format_row(node_dd.text())?));
                    },
                }
            }
        }
    }

    Ok(())
}

fn format_row(row: String) -> Result<MenuBody, StoreError> {
    let mut body = MenuBody::new();
    body.label = row;
    Ok(body)
}
