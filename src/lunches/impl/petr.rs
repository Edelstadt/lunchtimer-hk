extern crate lopdf;
extern crate pdf_extract;

use std::{
    sync::mpsc::Sender,
};

use chrono::{Datelike, Utc, Weekday};
use lopdf::*;
use pdf_extract::*;
use regex::Regex;
use reqwest::Client;

use crate::lunches::{
    menu::{Menu, MenuBody, MenuLine},
    store::StoreError,
};

//use select::{document::Document, node::Node, predicate::Class};

pub(crate) async fn fetch(tx: Sender<Result<Menu, StoreError>>) {
    tx.send(fetch_data()).unwrap();
}

fn fetch_data() -> Result<Menu, StoreError> {
    let c = Client::new();
    let res = c.get("https://www.penzionpetr-hk.cz/wp-content/uploads/denni-a-poledni-menu-ke-stazeni-kopie.pdf").send()?;

    let pdf_doc: Document = Document::load_from(res)?;

    let mut out: String = String::new();

    output_doc(&pdf_doc, &mut PlainTextOutput::new(&mut out));
    let mut menu = Menu::new("Petr");
    parser(&mut menu, out);
    Ok(menu)
}

fn translate_weekday(day: Weekday) -> &'static str {
    match day {
        Weekday::Mon => "Pondělí",
        Weekday::Tue => "Úterý",
        Weekday::Wed => "Středa",
        Weekday::Thu => "Čtvrtek",
        Weekday::Fri => "Pátek",
        Weekday::Sat => "Sobota",
        Weekday::Sun => "Neděle",
    }
}

fn parser(menu: &mut Menu, mut row: String) -> Result<(), StoreError> {
    let weekday = translate_weekday(Utc::now().weekday());
    let beta_offset = row.find(weekday).unwrap_or(row.len());
    let t: String = row.drain(..beta_offset).collect();
    let re_for_body =
        Regex::new(r"(?ms)(?P<amount>\d{1,4}g)(?P<empty>\s+)(?P<label>\D+)(?P<price>\d{1,3},-)")
            .unwrap();
    let re_for_soup =
        Regex::new(r"(?ms)(?P<amount>\D+:)(?P<label>\D+)(?P<price>\d{1,3}),-").unwrap();
    let re_for_main =
        Regex::new(r"(?ms)(?P<amount>\d+g)-(?P<label>\D+)(?P<price>\d{1,3}),-").unwrap();
    let before = "2012-03-14, 2013-01-01 and 2014-07-05";
    let cleanupbody = re_for_body.replace_all(row.as_str(), "$amount-$label-$price");
    for (i, line) in cleanupbody.lines().enumerate() {
        if i == 0 {
            let mut body = MenuBody::new();
            menu.body.push(MenuLine::Title(String::from("Polévka")));
            for cap in re_for_soup.captures_iter(line) {
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
            menu.body.push(MenuLine::Item(body));
        } else {
            let first = line.chars().next();
            match first {
                Some(p) => {
                    if p.is_digit(10) {
                        for cap in re_for_main.captures_iter(line) {
                            let mut body = MenuBody::new();
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
                            match cap.name("amount") {
                                Some(ok) => {
                                    body.amount = String::from(ok.as_str());
                                }
                                _ => {}
                            }
                            menu.body.push(MenuLine::Item(body));
                        }
                    } else {
                        break;
                    }
                }
                None => (),
            }
        }
    }
    Ok(())
}
