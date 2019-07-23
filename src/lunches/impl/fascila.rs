use std::sync::mpsc::{Sender};
use crate::Menu;

pub async fn fetch(tx: Sender<Menu>) {
    tx.send(
        Menu{id: 3, title: String::from("RestaurantC"), body: String::from("Menu of C ...")}
    );
}
