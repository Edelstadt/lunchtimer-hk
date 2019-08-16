use std::{io::Read, sync::mpsc::Sender};

use chrono::{Datelike, Utc};
use regex::Regex;
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
    let mut res = c
        .get("http://www.restauracefascila.cz/denni-menu/")
        .send()?;

    let mut body = String::new();
    res.read_to_string(&mut body)?;

    let mut menu = Menu::new("Fascila");
    parser(&mut menu, body)?;

    Ok(menu)
}

fn parser(menu: &mut Menu, body: String) -> Result<(), StoreError> {
    let doc = Document::from_read(body.as_bytes()).expect("Fascila - read body failed");
    let mut gg: usize = 0;
    for (index, node) in doc.find(Class("wpb_wrapper")).enumerate() {
        node.find(Name("h2")).enumerate().for_each(|(_, _)| {
            let now = Utc::now();
            let rr = format!("{}.{}", now.day(), now.month());
            if node.text().contains(rr.as_str()) {
                gg = index + 1;
            }
        });

        if gg != 0 && index == gg {
            for (i, node_i) in node.find(Class("vc_row")).enumerate() {
                let line = node_i.text().trim().to_string();
                let mut body = MenuBody::new();
                body.label = line.clone();
                match i {
                    0 => {
                        menu.body.push(MenuLine::Title(String::from("Polévka")));
                        menu.body.push(MenuLine::Item(format_row(line.clone())?));
                        menu.body.push(MenuLine::Title(String::from("Denní menu")));
                    },

                    _ => {
                        menu.body.push(MenuLine::Item(format_row(line.clone())?));
                    },
                }
            }
        }
    }
    Ok(())
}

fn format_row(row: String) -> Result<MenuBody, StoreError> {
    let mut body = MenuBody::new();

    let re = Regex::new(r"(?ms)(?P<label>\D+)(?P<price>[-+]?\d+),-").unwrap();

    for cap in re.captures_iter(row.as_str()) {
        if let Some(ok) = cap.name("label") {
            body.label = String::from(ok.as_str());
        }
        if let Some(ok) = cap.name("price") {
            body.price = ok.as_str().parse().clone()?;
        }
    }
    Ok(body)
}
