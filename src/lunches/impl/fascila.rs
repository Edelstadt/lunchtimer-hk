use crate::lunches::store::Menu;
use chrono::{Datelike, Utc};
use reqwest::Client;
use select::{
    document::Document,
    predicate::{Attr, Class, Name, Predicate},
};
use std::{io::Read, sync::mpsc::Sender};

pub async fn fetch(tx: Sender<Menu>) {
    let c = Client::new();
    let mut res = c
        .get("http://www.restauracefascila.cz/denni-menu/")
        .send()
        .unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body);
    tx.send(Menu {
        id:    1,
        title: String::from("Fascila"),
        body:  format!("{}", fascila_parser(&mut body)),
    })
    .unwrap();
}

fn fascila_parser(body: &mut String) -> String {
    let mut doc = Document::from_read(body.as_bytes()).unwrap();
    let mut gg: usize = 0;
    let mut r = String::new();
    for (index, node) in doc.find(Class("wpb_wrapper")).enumerate() {
        node.find(Name("h2")).enumerate().for_each(|(_, _)| {
            let now = Utc::now();
            let rr = format!("{}.{}", now.day(), now.month());
            if node.text().contains(rr.as_str()) {
                gg = index + 1;
            }
        });

        if gg != 0 && index == gg {
            for (i, node_i) in node.find(Class("vc_row")).enumerate() {
                let line = node_i.text().trim().to_string();

                match i {
                    0 => { r += format!("<h3><span>Polévka</span></h3>").as_str(); },
                    1 => { r += format!("<h3><span>Hlavní jídla</span></h3>").as_str(); },
                    _ => {},
                }

                // TODO předělat
                let mut c = line.chars().rev().collect::<String>().find(" ").unwrap();
                c = line.len() - c;

                r += format!("<p>{}&nbsp&nbsp&nbsp...<strong>{}</strong></p>", line[..c].to_string(), line[c..].to_string()).as_str();
            }
        }
    }
    r
}
