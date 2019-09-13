#![feature(async_await, try_trait)]
#[macro_use]
extern crate nickel;
#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;

use chrono::{Datelike, Utc};
use nickel::{HttpRouter, Nickel};

use crate::lunches::{
    helpers::translate_weekday,
    menu::HtmlMenu,
    store::{get_menus, update_menus},
};

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

    server.listen("0.0.0.0:8080").expect("Server fail");
}

#[derive(Serialize)]
enum TemplateValues<'a> {
    Str(&'a str),
    Menus(&'a [HtmlMenu]),
}
