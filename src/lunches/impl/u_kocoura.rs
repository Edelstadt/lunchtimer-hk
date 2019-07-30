use std::{error::Error, io::Read, sync::mpsc::Sender};

use reqwest::Client;
use select::{
    document::Document,
    predicate::{Attr, Class, Name},
};

use crate::lunches::{menu::Menu, store::StoreError};

pub async fn fetch(tx: Sender<Result<Menu, StoreError>>) {
    tx.send(fetch_data()).unwrap();
}

pub fn fetch_data() -> Result<Menu, StoreError> {
    let c = Client::new();
    let mut res = c
        .get("https://www.ukocourahk.cz/denni-menu/")
        .send()?;
        //.or(Err(StoreError::Fetch("Kocour: failed to fetch menu")))?;

    let mut body = String::new();
    res.read_to_string(&mut body)
        .or(Err(StoreError::Fetch("Kocour: failed to read fetched menu")))?;

    let mut menu = Menu::new("U kocoura");
    kocour_denni_parser(&mut menu, body)?;

    Ok(menu)
}

fn kocour_denni_parser(menu: &mut Menu, body: String) -> Result<(), StoreError> {
    let mut doc = Document::from_read(body.as_bytes())
        .or(Err(StoreError::Fetch("Kocour: failed to parse menu")))?;

    let mut r = String::new();
    for node in doc.find(Class("cms-content")) {
        for tr in node.find(Name("tr")) {
            let line = tr.text().trim().to_string();
            if !line.ends_with("Kč") {
                r += format!("<h3><span>{}</span></h3>", line).as_str();
            } else {
                let mut c = line
                    .chars()
                    .rev()
                    .skip(3)
                    .collect::<String>()
                    .find(" ")
                    .ok_or(StoreError::Parse("Kocour: parse price"))?;
                c = line.len() - c;

                r += format!(
                    "<p>{}&nbsp&nbsp&nbsp...<strong>{}</strong></p>",
                    line.chars().take(c).collect::<String>(),
                    line.chars().skip(c).collect::<String>()
                )
                .as_str();
            }
        }
    }

    Ok()
}
