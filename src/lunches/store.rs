use std::{sync::mpsc::channel, thread, time::Duration};

use chrono::{Local, Timelike};

use crate::lunches::{menu::Menu, r#impl as menus};
use reqwest::Error;
use crate::lunches::menu::{MenuLine, HtmlMenu};

static mut MENUS: Option<Vec<HtmlMenu>> = None; // TODO Mutex

pub fn get_menus() -> &'static [HtmlMenu] {
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

        let c = 2; // Nelze přes Trait -> nepodporují async fn
                      //        let f1 = (menus::fascila(tx.clone()));
        let f2 = (menus::u_kocoura(tx.clone()));
        //        let f3 = (menus::beranek(tx.clone()));
                let f4 = (menus::sova(tx.clone()));

        futures::join!(f2, f4);

        let mut data: Vec<HtmlMenu> = vec![];
        for i in 0..c {
            match rx.iter().next().expect("Data push to channel fail") {
                Ok(x) => {
                    println!("{} - ok", &x.title);
                    let mut menu = format_html_menu(x);
                    menu.id = i;
                    data.push(menu)
                },
                Err(e) => println!("{:?}", e),
            }
        }
        set_menus(data);
    }
}

fn format_html_menu(menu: Menu) -> HtmlMenu {
    let mut html = HtmlMenu::new(menu.title);
    for line in menu.body {
        match line {
            MenuLine::Title(x) => {
                html.body += &format!("<h3><span>{}</h3></span>", x);
            },
            MenuLine::Item(x) => {
                html.body += &format!(
                    "<p>{} {}&nbsp&nbsp...<strong>{} Kč</strong></p>",
                    x.amount,
                    x.label,
                    x.price
                );
            },
        }
    }

    html
}

#[derive(Debug)]
pub enum StoreError {
    Fetch(&'static str),
    Parse(&'static str),
}

// TODO lepší hlášky
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

impl std::convert::From<std::option::NoneError> for StoreError {
    fn from(_: std::option::NoneError) -> Self {
        StoreError::Parse("None option")
    }
}

impl std::convert::From<std::num::ParseIntError> for StoreError {
    fn from(_: std::num::ParseIntError) -> Self {
        StoreError::Parse("Parse to integer")
    }
}
