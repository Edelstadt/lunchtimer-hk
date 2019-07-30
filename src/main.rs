#![feature(async_await, try_trait)]
#[macro_use]
extern crate nickel;
#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;

use nickel::{HttpRouter, Nickel};

use crate::lunches::store::{get_menus, update_menus};
use chrono::{Datelike, Utc, Weekday};
use crate::lunches::menu::HtmlMenu;

mod lunches;

#[runtime::main]
async fn main() {
    runtime::spawn(update_menus());
    server().await;
}

async fn server() {
    let mut server = Nickel::new();

    server.get(
        "**",
        middleware! {|_, response|
            let mut data: HashMap<&str, TemplateValues> = HashMap::new();
            let date = Utc::now();
            let formatted_date = format!(
                "{}: {}.{}.",
                translate_weekday(date.weekday()),
                date.day(),
                date.month()
            );

            data.insert("menus", TemplateValues::Menus(get_menus()));
            data.insert("date", TemplateValues::Str(&formatted_date));
            return response.render("assets/lunches.tpl", &data)
        },
    );

    server.listen("127.0.0.5:8000").expect("Server fail");
}

#[derive(Serialize)]
enum TemplateValues<'a> {
    Str(&'a str),
    Menus(&'a [HtmlMenu]),
}

fn translate_weekday(day: Weekday) -> &'static str {
    match day {
        Weekday::Mon => "Pondělí",
        Weekday::Tue => "Úterý",
        Weekday::Wed => "Středa",
        Weekday::Thu => "Čtvrtek",
        Weekday::Fri => "Pátek",
        Weekday::Sat => "Sobota",
        Weekday::Sun => "Neděle",
    }
}
