use std::{io::Read, sync::mpsc::Sender};

use reqwest::Client;
use select::{
    document::Document,
    predicate::{Attr, Class, Name, Predicate},
};

use crate::lunches::store::Menu;

pub async fn fetch(tx: Sender<Menu>) {
    let c = Client::new();
    let mut res = c
        .get("https://www.ukocourahk.cz/denni-menu/")
        .send()
        .expect("U kocoura - request fail");

    let mut body = String::new();
    res.read_to_string(&mut body);

    tx.send(Menu {
        id:    3,
        title: String::from("U Kocoura"),
        body:  format!("{}", kocour_denni_parser(&mut body)),
    })
        .expect("Kocour - Not send");
}

fn kocour_denni_parser(body: &mut String) -> String {
    let mut doc = Document::from_read(body.as_bytes()).expect("Kocour - read body failed");

    let mut r = String::new();
    for node in doc.find(Class("cms-content")) {
        for tr in node.find(Name("tr")) {
            let line = tr.text().trim().to_string();
            if !line.ends_with("Kč") {
                r += format!("<h3><span>{}</span></h3>", line).as_str();
            } else {
                // TODO předělat
                let mut c = line.chars().rev().skip(3).collect::<String>().find(" ").unwrap();
                c = line.len() - c;

                r += format!("<p>{}&nbsp&nbsp&nbsp...<strong>{}</strong></p>", line[..c].to_string(), line[c..].to_string()).as_str();
            }
        }
    }

    r
}
