use std::{io::Read, sync::mpsc::Sender};

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
    let mut res = c.get("https://www.ukocourahk.cz/denni-menu/").send()?;

    let mut body = String::new();
    res.read_to_string(&mut body)?;

    let mut menu = Menu::new("U kocoura");
    kocour_denni_parser(&mut menu, body)?;

    Ok(menu)
}

fn kocour_denni_parser(menu: &mut Menu, body: String) -> Result<(), StoreError> {
    let doc = Document::from_read(body.as_bytes())?;

    for node in doc.find(Class("cms-content")) {
        for tr in node.find(Name("tr")) {
            let line = tr.text().trim().to_string();
            if !line.ends_with("Kč") {
                menu.body.push(MenuLine::Title(line));
            } else {
                let mut am = line.find('\t')?;

                let mut c = line.chars().rev().collect::<String>().find("\t")?;

                c = line.chars().count() - c;

                // TODO lepší ohlídání by bolo fajn (pokud chybí množství, např. u salátu)
                if am > c {
                    am = 0;
                }

                menu.body.push(MenuLine::Item(MenuBody {
                    amount: line.chars().take(am).collect::<String>(),
                    label:  line.chars().skip(am).take(c - am).collect::<String>(),
                    price:  line
                        .chars()
                        .skip(c + 1)
                        .collect::<String>()
                        .split(' ')
                        .next()?
                        .parse::<usize>()?, // TODO char::is_numberic
                }));
            }
        }
    }

    Ok(())
}
