use std::{io::Read, sync::mpsc::Sender};

use reqwest::get;
use select::{
    document::Document,
    predicate::{Class, Name},
};

use crate::lunches::{
    menu::{Menu, MenuBody, MenuLine},
    store::StoreError,
};

pub(crate) fn fetch(tx: Sender<Result<Menu, StoreError>>, id: &str, name: &str) {
    tx.send(fetch_data(id, name)).unwrap();
}

fn fetch_data(id: &str, name: &str) -> Result<Menu, StoreError> {
    let body =
        reqwest::blocking::get(format!("https://www.menicka.cz/{}.html", id).as_str())?.text()?;

    let mut menu = Menu::new(name);
    denni_parser(&mut menu, body)?;
    Ok(menu)
}

fn denni_parser(menu: &mut Menu, body: String) -> Result<(), StoreError> {
    let doc = Document::from_read(body.as_bytes())?;

    for node in doc.find(Class("menicka")) {
        menu.body.push(MenuLine::Title(String::from("Polévky")));

        for it in node.find(Class("polevka")) {
            let mut price = String::new();
            if let Some(cl) = it.find(Class("cena")).next() {
                if let Some(cl1) = cl.first_child() {
                    if let Some(t) = cl1.as_text() {
                        price = t.to_string();
                    }
                }
            }

            menu.body.push(MenuLine::Item(MenuBody {
                label:  it
                    .find(Class("polozka"))
                    .next()?
                    .first_child()?
                    .as_text()?
                    .to_string(),
                price,
            }));
        }

        menu.body.push(MenuLine::Title(String::from("Jídla")));
        for it in node.find(Class("jidlo")) {
            let mut price = String::new();
            if let Some(cl) = it.find(Class("cena")).next() {
                if let Some(cl1) = cl.first_child() {
                    if let Some(t) = cl1.as_text() {
                        price = t.to_string();
                    }
                }
            }
            let label = it
                .find(Class("polozka"))
                .next()?
                .first_child()?
                .next()?
                .as_text()?
                .to_string();

            menu.body.push(MenuLine::Item(MenuBody {
                label,
                price,
            }));
        }

        break;
    }

    Ok(())
}
