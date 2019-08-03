use std::{sync::mpsc::channel, thread, time::Duration};

use chrono::{Local, Timelike};

use crate::lunches::{
    menu::{HtmlMenu, Menu, MenuLine},
    r#impl as menus,
};

static mut MENUS: Option<Vec<HtmlMenu>> = None; // TODO Mutex

pub fn get_menus<'a>() -> &'a [HtmlMenu] {
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

        let c = 3; // Nelze přes Trait -> nepodporují async fn
                   //        let f1 = (menus::fascila(tx.clone()));
        let f2 = menus::u_kocoura(tx.clone());
        let f3 = menus::beranek(tx.clone());
        let f4 = menus::sova(tx.clone());

        futures::join!(f2, f3, f4);

        let mut data: Vec<HtmlMenu> = vec![];
        for i in 0..c {
            match rx.iter().next().expect("Data push to channel fail") {
                Ok(x) => {
                    println!("{} - ok", &x.title);
                    let mut menu = format_html_menu(x);
                    menu.id = i;
                    data.push(menu)
                },
                Err(e) => println!("{:?}", e), // TODO hláška která restauračka padla
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
            MenuLine::Item(ref x) if x.price > 0 => {
                html.body += &format!(
                    "<p>{} {}&nbsp&nbsp<strong>{} Kč</strong></p>",
                    x.amount, x.label, x.price
                );
            },
            MenuLine::Item(x) => {
                html.body += &format!(
                    "<p>{} {}</p>",
                    x.amount, x.label
                );
            },
        }
    }

    html
}

#[derive(Debug)]
pub enum StoreError {
    Fetch(String),
    Parse(String),
}

// TODO lepší hlášky
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
