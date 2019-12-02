use std::{sync::mpsc::channel, thread, time::Duration};

use chrono::{Local, Timelike};

use crate::lunches::{
    menu::{HtmlMenu, Menu},
    r#impl as menus,
};

static mut MENUS: Option<Vec<HtmlMenu>> = None;

pub(crate) fn get_menus<'a>() -> &'a [HtmlMenu] {
    unsafe {
        match MENUS {
            Some(ref x) => x,
            None => panic!("uninitialized!"),
        }
    }
}

fn set_menus(mut data: Vec<HtmlMenu>) {
    unsafe {
        match MENUS {
            Some(ref mut x) => {
                x.clear();
                x.append(&mut data);
            },
            None => panic!("uninitialized!"),
        }
    }
}

fn init_menus() {
    unsafe {
        MENUS = Some(vec![]);
    }
}

pub(crate) async fn update_menus() {
    init_menus();
    let mut last_hour: u32 = 99;

    loop {
        // Updates once an hour
        let curr = Local::now();
        if curr.hour() == last_hour {
            thread::sleep(Duration::from_secs(60 * 15));
            continue;
        }
        last_hour = curr.hour();

        let (tx, rx) = channel::<Result<Menu, StoreError>>();

        let c = 8_usize; // Nelze přes Trait -> nepodporují async fn
                         // let f1 = (menus::fascila(tx.clone()));
        menus::menicka(tx.clone(), "1132", "U Kocoura");
        menus::menicka(tx.clone(), "1135", "Na Hradě");
        menus::menicka(tx.clone(), "1824", "Kozlovka");
        menus::menicka(tx.clone(), "1236", "Na Statku");
        menus::menicka(tx.clone(), "1779", "Mexita");
        menus::menicka(tx.clone(), "5797", "Náplavka");
        menus::menicka(tx.clone(), "1843", "Fascila");
        menus::cerny_kun(tx.clone());

        let mut data: Vec<HtmlMenu> = vec![];
        for i in 0..c {
            match rx.iter().next().expect("Data push to channel fail") {
                Ok(x) => {
                    println!("{} - ok", &x.title);
                    let mut menu = HtmlMenu::from(x);
                    menu.set_id(i);
                    data.push(menu)
                },
                Err(e) => println!("{:?}", e), // TODO hláška která restauračka padla
            }
        }
        set_menus(data);
    }
}

#[derive(Debug)]
pub(crate) enum StoreError {
    Fetch(String),
    Parse(String),
}

impl std::convert::From<reqwest::Error> for StoreError {
    fn from(_: reqwest::Error) -> Self {
        StoreError::Fetch(String::from("Request fetch"))
    }
}

impl std::convert::From<std::io::Error> for StoreError {
    fn from(_: std::io::Error) -> Self {
        StoreError::Parse(String::from("Read data"))
    }
}

impl std::convert::From<std::option::NoneError> for StoreError {
    fn from(_: std::option::NoneError) -> Self {
        StoreError::Parse(String::from("None option"))
    }
}

impl std::convert::From<std::num::ParseIntError> for StoreError {
    fn from(_: std::num::ParseIntError) -> Self {
        StoreError::Parse(String::from("Parse to integer"))
    }
}

impl std::convert::From<regex::Error> for StoreError {
    fn from(_: regex::Error) -> Self {
        StoreError::Parse(String::from("Parse regex"))
    }
}
