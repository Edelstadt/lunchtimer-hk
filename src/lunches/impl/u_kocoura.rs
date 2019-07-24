use crate::lunches::store::Menu;
use reqwest::Client;
use select::{
    document::Document,
    predicate::{Attr, Class, Name, Predicate},
};
use std::{io::Read, sync::mpsc::Sender};

pub async fn fetch(tx: Sender<Menu>) {
    let c = Client::new();
    let mut res = c
        .get("https://www.ukocourahk.cz/denni-menu/")
        .send()
        .unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body);

    tx.send(Menu {
        id:    3,
        title: String::from("U Kocoura"),
        body:  format!("{}", kocour_denni_parser(&mut body)),
    })
    .unwrap();
}

pub fn kocour_denni_parser(body: &mut String) -> String {
    let mut doc = Document::from_read(body.as_bytes()).unwrap();

    let mut r = String::new();
    for node in doc.find(Class("cms-content")) {
        for tr in node.find(Name("tr")) {
            for (_, element) in tr.text().split_whitespace().enumerate() {
                r += format!(" {}", element).as_str()
            }
            r += "<br />";
        }
    }

    r
}
