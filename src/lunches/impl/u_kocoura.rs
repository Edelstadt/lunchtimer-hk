use std::sync::mpsc::{Sender};
use crate::lunches::store::Menu;

pub async fn fetch(tx: Sender<Menu>) {
    tx.send(
        Menu{id: 3, title: String::from("U Kocoura"), body: String::from("Menu of kitty ...")}
    );
}
