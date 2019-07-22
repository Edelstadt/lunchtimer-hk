//use reqwest;
use std::sync::mpsc::{Sender};

pub async fn fetch(tx: Sender<String>) {
//    let body = reqwest::get("https://www.rust-lang.org").unwrap()
//        .text().unwrap();
//    println!("body2 = {:#?}", body);

    tx.send(String::from("Something happened...")).unwrap();
}
