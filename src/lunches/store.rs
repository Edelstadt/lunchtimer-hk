use crate::lunches::r#impl as menus;
use chrono::{Local, Timelike};
use std::{sync::mpsc::channel, thread, time::Duration};

#[derive(Serialize)]
pub struct Menu {
    pub id:    u8, // TODO private
    pub title: String,
    pub body:  String,
}

static mut MENUS: Option<Vec<Menu>> = None; // TODO Mutex

pub fn get_menus() -> &'static [Menu] {
    unsafe {
        match MENUS {
            Some(ref x) => x,
            None => panic!("uninitialized!"),
        }
    }
}

pub fn set_menus(mut data: Vec<Menu>) {
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

pub fn init_menus() {
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

        // Fetch new data
        let (tx, rx) = channel();

        let c = 2;
        runtime::spawn(menus::fascila(tx.clone()));
        runtime::spawn(menus::u_kocoura(tx.clone()));

        let mut data: Vec<Menu> = vec![];
        for _ in 0..c {
            data.push(rx.recv().unwrap());
        }
        set_menus(data);
    }
}
