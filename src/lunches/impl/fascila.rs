use crate::lunches::store::Menu;
use std::sync::mpsc::Sender;

pub async fn fetch(tx: Sender<Menu>) {
    tx.send(Menu {
        id:    1,
        title: String::from("Fascila"),
        body:  String::from("Menu of fazilly ..."),
    })
    .unwrap();
}
