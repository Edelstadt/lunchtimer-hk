use std::{sync::mpsc::Sender};

use chrono::{Datelike, Utc};
use encoding::all::WINDOWS_1250;
use regex::Regex;
use reqwest::Client;
use select::{
    document::Document,
    predicate::Name,
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
    let mut res = c
        .get("https://www.menicka.cz/api/iframe/?id=5754")
        .send()?;

    let mut menu = Menu::new("Náplavka");
    naplavka_parser(&mut menu, res.text_with_charset(WINDOWS_1250.name)?).expect("Náplavka - parse error");

    Ok(menu)
}

fn naplavka_parser(menu: &mut Menu, body: String) -> Result<(), StoreError> {
    let doc = Document::from_read(body.as_bytes())?;
    for node in doc.find(Name("div")) {
        let now = Utc::now();
        let rr = format!("{}.{}.{}", now.day(), now.month(), now.year());
        if node.text().contains(rr.as_str()) {
            menu.body.push(MenuLine::Title(String::from("Denní menu")));
            let t = node.text();

            menu.body.push(MenuLine::Item(format_row(String::from(t.trim_matches('\n')))?));
        }
    }

    Ok(())
}

fn format_row(row: String) -> Result<MenuBody, StoreError> {
    let mut body = MenuBody::new();

    let re = Regex::new(r"(?ms)(?P<amount>\d+ g)(?P<label>\D+)(?P<price>[-+]?\d+) Kč").unwrap();

    for cap in re.captures_iter(row.as_str()) {
        match cap.name("amount") {
            Some(ok) => {
                body.amount = String::from(ok.as_str());
            }
            _ => {}
        }
        match cap.name("label") {
            Some(ok) => {
                body.label = String::from(ok.as_str());
            }
            _ => {}
        }
        match cap.name("price") {
            Some(ok) => {
                body.price = ok.as_str().parse().clone()?;
            }
            _ => {}
        }
    }
    Ok(body)
}
