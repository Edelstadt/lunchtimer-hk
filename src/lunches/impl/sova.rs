use std::{io::Read, sync::mpsc::Sender};

use chrono::{Datelike, Utc};
use reqwest::Client;
use select::{
    document::Document,
    predicate::{Attr, Class, Name, Predicate},
};

use crate::lunches::store::{StoreError};
use select::node::Node;
use crate::lunches::menu::{Menu, MenuLine, MenuBody};

pub async fn fetch(tx: Sender<Result<Menu, StoreError>>) {
    tx.send(fetch_data()).unwrap();
}

pub fn fetch_data() -> Result<Menu, StoreError> {
    let c = Client::new();
    let mut res = c
        .get("https://www.sovahk.cz/jidelni_listek")
        .send()?;

    let mut body = String::new();
    res.read_to_string(&mut body)?;

    let mut menu = Menu::new("Sova");
    parser(&mut menu, body)?;

    Ok(menu)
}

fn parser(menu: &mut Menu,body: String) -> Result<(), StoreError> {
    let mut doc = Document::from_read(body.as_bytes())?;
    let mut gg: usize = 0;

    let today = format!("{}.", Utc::now().day());

    doc.find(Class("den")).try_for_each(|day| -> Result<(), StoreError> {
        let date = day.attr("data_datum").ok_or(StoreError::Parse("Failed to parse date"))?;
        if date.starts_with(today.as_str()) {
            let mut offer = day;
            menu.body.push(MenuLine::Title(String::from("Polévka")));
            menu.body.push(MenuLine::Item(format_row(&offer.next().ok_or(StoreError::Parse("parse"))?)?));
            menu.body.push(MenuLine::Title(String::from("Denní menu")));

            for i in 0..5 {
                offer = offer.next().ok_or(StoreError::Parse("parse"))?;
                menu.body.push(MenuLine::Item(format_row(&offer.next().ok_or(StoreError::Parse("parse"))?)?));
            }
        }
        Ok(())
    })?;

    Ok(())
}

fn format_row(row: &Node) -> Result<MenuBody, StoreError> {
    let mut body = MenuBody::empty();
    for (i, ch) in row.children().enumerate() {
        match i {
            0 => {
                body.amount =
                    ch.first_child()?
                        .first_child()?
                        .as_text()?
                        .to_string();
            },
            1 => {
                body.label =
                    ch.first_child()?
                        .first_child()?
                        .first_child()?
                        .as_text()?
                        .to_string();
            },
            2 => {
                body.price =
                    ch.first_child()?
                        .first_child()?
                        .as_text()?
                        .chars()
                        .filter(|ch| ch.is_numeric())
                        .collect::<String>()
                        .parse::<usize>()?;
            },
            _ => break,
        }
    }

    Ok(body)
}
