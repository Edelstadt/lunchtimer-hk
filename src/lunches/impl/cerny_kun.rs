use std::{io::Read, sync::mpsc::Sender};

use reqwest::get;
use select::{
    document::Document,
    predicate::{Class, Name, Attr},
};
use chrono::{Weekday, DateTime, Datelike, Local};

use crate::lunches::{
    menu::{Menu, MenuBody, MenuLine},
    store::StoreError,
};
use std::time::SystemTime;

pub(crate) fn fetch(tx: Sender<Result<Menu, StoreError>>) {
    tx.send(fetch_data()).unwrap();
}

fn fetch_data() -> Result<Menu, StoreError> {
    let body =
        reqwest::blocking::get("http://www.cernykunhk.cz/tydenni-menu")?.text()?;

    let mut menu = Menu::new("Černý kůň");
    denni_parser(&mut menu, body)?;
    Ok(menu)
}

fn denni_parser(menu: &mut Menu, body: String) -> Result<(), StoreError> {
    let doc = Document::from_read(body.as_bytes())?;
    let date: DateTime<Local> = chrono::DateTime::from(SystemTime::now());
    let day = date.weekday().number_from_monday();

    if day > 5 {
        return Err(StoreError::Parse(String::from("not a weekday")));
    }

    let name = format!("Table_{}", day);
    for node in doc.find(Attr("id", name.as_str())) {
        menu.body.push(MenuLine::Title(String::from("Polévka")));

        let rows = node.find(Class("text_white"));
        for it in rows.skip(1 + (day == 0) as usize).take(1) {
            menu.body.push(MenuLine::Item(MenuBody {
                label:  it.first_child()?.as_text()?.to_string(),
                price: String::new(),
            }));
        }

        menu.body.push(MenuLine::Title(String::from("Jídla")));

        let rows = node.find(Class("text_white")); // TODO moved
        for it in rows.skip(2 + (day == 0) as usize) {
            menu.body.push(MenuLine::Item(MenuBody {
                label:  it.first_child()?.as_text()?.to_string(),
                price: it.last_child()?.as_text()?.to_string(),
            }));
        }

        break;
    }

    Ok(())
}
