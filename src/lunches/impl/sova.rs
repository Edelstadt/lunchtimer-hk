use std::{io::Read, sync::mpsc::Sender};

use chrono::{Datelike, Utc};
use reqwest::Client;
use select::{
    document::Document,
    predicate::{Attr, Class, Name, Predicate},
};

use crate::lunches::store::Menu;
use select::node::Node;

pub async fn fetch(tx: Sender<Menu>) {
    let c = Client::new();
    let mut res = c
        .get("https://www.sovahk.cz/jidelni_listek")
        .send()
        .expect("Sova - request fail");

    let mut body = String::new();
    res.read_to_string(&mut body);
    tx.send(Menu {
        id:    4,
        title: String::from("Sova"),
        body:  format!("{}", parser(&mut body)),
    })
        .expect("Sova - Not send");
}

fn parser(body: &mut String) -> String {
    let mut doc = Document::from_read(body.as_bytes()).expect("Sova - read body failed");
    let mut gg: usize = 0;
    let mut r = String::new();

    let today = format!("{}.", Utc::now().day());

        doc.find(Class("den")).for_each(|day| {
            let date = day.attr("data_datum").expect("faield to parse date");
            if date.starts_with(today.as_str()) {
                let mut offer = day;
                r += format!("<h3><span>Polévka</span></h3>").as_str();
                r += format_row(&offer.next().unwrap()).as_str();
                r += format!("<h3><span>Denní menu</span></h3>").as_str();

                for i in 0..5 {
                    offer = offer.next().unwrap();
                    r += format_row(&offer.next().unwrap()).as_str();
                }
            }
        });
    r
}

fn format_row(row: &Node) -> String {
    let mut attrs = vec![];
    for (i, ch) in row.children().enumerate() {
        match i {
            0 => { attrs.push(ch.first_child().unwrap().first_child().unwrap().as_text().unwrap()); },
            1 => {
                attrs.push(ch.first_child().unwrap().first_child().unwrap().first_child().unwrap().as_text().unwrap());
            },
            2 => { attrs.push(ch.first_child().unwrap().first_child().unwrap().as_text().unwrap()); },
            _ => break,
        }
    }

    format!("<p>{} {}&nbsp&nbsp&nbsp...<strong>{}</strong></p>", attrs[0], attrs[1], attrs[2])
}
