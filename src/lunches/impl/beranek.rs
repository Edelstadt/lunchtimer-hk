use std::io::Read;
use std::sync::mpsc::Sender;

use chrono::{Datelike, Utc};
use regex::{Captures, Regex};
use reqwest::Client;
use select::{document::Document, predicate::Class};

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
        .get("http://www.pivovarberanek.cz/")
        .send()?;

    let mut body = String::new();
    res.read_to_string(&mut body)?;

    let mut menu = Menu::new("Beránek");
    parser(&mut menu, body).expect("Beranek - parse error");

    Ok(menu)
}

fn parser(menu: &mut Menu, body: String) -> Result<(), StoreError> {
    let doc = Document::from_read(body.as_bytes())?;

    for node in doc.find(Class("nabidkatext")) {
        let now = Utc::now();
        let rr = format!("{}.{}", now.day(), now.month());

        let re = Regex::new(r"(?P<date>\D+\s\d+\.\d+\.)(?P<soup>\D+)(?P<menu1>Menu\s\d+:\s\D+\s\d{1,4},-)(?P<menu2>Menu\s\d:\s\D+\d{1,4},-)")?;
        for part in re.captures_iter(node.text().as_str()) {
            let date = part.name("date")?;
            if date.as_str().contains(rr.as_str()) {
                menu.body.push(MenuLine::Title(String::from("Polévka")));
                menu.body.push(MenuLine::Item(MenuBody {
                    price:  0,
                    amount: String::new(),
                    label:  part
                        .name("soup")?
                        .as_str()
                        .chars()
                        .skip(8)
                        .collect::<String>(),
                }));

                menu.body.push(MenuLine::Title(String::from("Denní menu")));

                menu.body.push(MenuLine::Item(parse_line(&part, "menu1")?));
                menu.body.push(MenuLine::Item(parse_line(&part, "menu2")?));
            }
        }
    }

    Ok(())
}

fn parse_line(part: &Captures, tag: &str) -> Result<MenuBody, StoreError> {
    let mut line = part.name(tag)?.as_str().chars().skip(8).collect::<String>();
    line = line.trim_end_matches(",-").to_string();
    let price = line
        .chars()
        .rev()
        .take_while(|ch| ch.is_numeric())
        .collect::<String>()
        .chars()
        .rev()
        .collect::<String>();

    Ok(MenuBody {
        price:  price.parse::<usize>()?,
        amount: String::new(),
        label:  line
            .chars()
            .take(line.chars().count() - price.chars().count())
            .collect(),
    })
}
