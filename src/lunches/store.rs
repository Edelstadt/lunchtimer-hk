use std::{sync::mpsc::channel, thread, time::Duration};

use chrono::{Local, Timelike};

use crate::lunches::{menu::Menu, r#impl as menus};
use reqwest::Error;

static mut MENUS: Option<Vec<Menu>> = None; // TODO Mutex

pub fn get_menus() -> &'static [Menu] {
    unsafe {
        match MENUS {
            Some(ref x) => x,
            None => panic!("uninitialized!"),
        }
    }
}

fn set_menus(mut data: Vec<Menu>) {
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

pub async fn update_menus() {
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
        //        let (tx, rx) = channel();

        let c = 1_u8; // Nelze přes Trait -> nepodporují async fn
                      //        let f1 = (menus::fascila(tx.clone()));
        let f2 = (menus::u_kocoura(tx.clone()));
        //        let f3 = (menus::beranek(tx.clone()));
        //        let f4 = (menus::sova(tx.clone()));

        futures::join!(f2);

        let mut data: Vec<Menu> = vec![];
        for _ in 0..c {
            match rx.iter().next().expect("Data push to channel fail") {
                Ok(x) => {
                    println!("Kocour - ok");
                    data.push(x)
                },
                Err(e) => println!("{:?}", e),
            }
        }
        set_menus(data);
    }
}

#[derive(Debug)]
pub enum StoreError {
    Fetch(&'static str),
    Parse(&'static str),
}

impl std::convert::From<reqwest::Error> for StoreError {
    fn from(_: reqwest::Error) -> Self {
        StoreError::Fetch("Request fetch")
    }
}

impl std::convert::From<std::io::Error> for StoreError {
    fn from(_: std::io::Error) -> Self {
        StoreError::Parse("Read data")
    }
}
