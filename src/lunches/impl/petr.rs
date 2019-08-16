use std::sync::mpsc::Sender;

use chrono::{Datelike, Utc};
use lopdf::Document;
use pdf_extract::{output_doc, PlainTextOutput};
use regex::Regex;
use reqwest::Client;

use crate::lunches::{
    helpers::translate_weekday,
    menu::{Menu, MenuBody, MenuLine},
    store::StoreError,
};

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
    parser(&mut menu, out)?;

    Ok(menu)
}

fn parser(menu: &mut Menu, mut row: String) -> Result<(), StoreError> {
    let weekday = translate_weekday(Utc::now().weekday());
    let beta_offset = row.find(weekday).unwrap_or_else(|| row.len());
    row.drain(..beta_offset);

    let re_for_body =
        Regex::new(r"(?ms)(?P<amount>\d{1,4}g)(?P<empty>\s+)(?P<label>\D+)(?P<price>\d{1,3},-)")?;
    let re_for_soup = Regex::new(r"(?ms)(?P<amount>\D+:)(?P<label>\D+)(?P<price>\d{1,3}),-")?;
    let re_for_main = Regex::new(r"(?ms)(?P<amount>\d+g)-(?P<label>\D+)(?P<price>\d{1,3}),-")?;

    let cleanup_body = re_for_body.replace_all(row.as_str(), "$amount-$label-$price");
    for (i, line) in cleanup_body.lines().enumerate() {
        match i {
            0 => {
                menu.body.push(MenuLine::Title(String::from("Polévka")));

                let mut body = MenuBody::new();
                for cap in re_for_soup.captures_iter(line) {
                    if let Some(ok) = cap.name("label") {
                        body.label = String::from(ok.as_str());
                    }

                    if let Some(ok) = cap.name("price") {
                        body.price = ok.as_str().parse().clone()?;
                    }
                }

                menu.body.push(MenuLine::Item(body));
                menu.body.push(MenuLine::Title(String::from("Denní menu")));
            },
            1 => (),
            _ => {
                let first = line.chars().next();
                if let Some(p) = first {
                    if p.is_digit(10) {
                        for cap in re_for_main.captures_iter(line) {
                            let mut body = MenuBody::new();
                            if let Some(ok) = cap.name("label") {
                                body.label = String::from(ok.as_str());
                            }
                            if let Some(ok) = cap.name("price") {
                                body.price = ok.as_str().parse().clone()?;
                            }
                            if let Some(ok) = cap.name("amount") {
                                body.amount = String::from(ok.as_str());
                            }

                            menu.body.push(MenuLine::Item(body));
                        }
                    } else {
                        break;
                    }
                }
            },
        }
    }
    Ok(())
}
