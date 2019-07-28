use std::{io::Read, sync::mpsc::Sender};

use chrono::{Datelike, Utc};
use reqwest::Client;
use select::{
    document::Document,
    predicate::{Attr, Class, Name, Predicate},
};

use regex::Regex;

use crate::lunches::store::Menu;

pub async fn fetch(tx: Sender<Menu>) {
    let c = Client::new();
    let mut res = c
        .get("https://www.pivovarberanek.cz/#jidelni-listek")
        .send()
        .expect("Beranek - request fail");

    let mut body = String::new();
    res.read_to_string(&mut body);

    tx.send(Menu {
        id: 2,
        title: String::from("Beranek"),
        body: format!("{}", beranek_parser(&mut body)),
    })
        .expect("Beranek - Not send");
}

fn beranek_parser(body: &mut String) -> String {
    let mut doc = Document::from_read(body.as_bytes()).expect("Beranek - read body failed");

    let mut r = String::new();
    for node in doc.find(Class("nabidkatext")) {
        let now = Utc::now();
        let rr = format!("{}.{}", now.day(), now.month());

        let re = Regex::new(r"(?P<date>\D+\s\d+\.\d+\.)(?P<soup>\D+)(?P<menu1>Menu\s\d+:\s\D+\s\d{1,4},-)(?P<menu2>Menu\s\d:\s\D+\d{1,4},-)").unwrap();
        for part in re.captures_iter(node.text().as_str()) {
            let date = part.name("date").unwrap();
            if date.as_str().contains(rr.as_str()) {
                r += format!("{}", date.as_str()).as_str();
                r += format!("{}", part.name("soup").unwrap().as_str()).as_str();
                r += format!("{}", part.name("menu1").unwrap().as_str()).as_str();
                r += format!("{}", part.name("menu2").unwrap().as_str()).as_str();
            }
        }
    }

    r
}
