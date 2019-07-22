extern crate futures;
extern crate reqwest;
mod parsers;

use parsers::*;
use std::mem;
#[macro_use]
mod macros;
use encoding::all::{UTF_8, WINDOWS_1250};
use encoding::{DecoderTrap, Encoding};
use futures::{Future, Stream};
use reqwest::r#async::{Client, Decoder};
use select::document::Document;

pub fn fetch() -> impl Future<Item = (), Error = ()> {
    let client = Client::new();
    menicka!(
        na_hrade_poledni,
        "1135",
        "--- Na hradě - polední menu ---",
        client
    );
    menicka!(naplavka, "5797", "--- Náplavka menu ---", client);
    menicka!(na_statku, "1236", "--- Na statku polední menu ---", client);

    let na_hrade_basic =
        begin!("http://www.restaurace-nahrade.cz/menu-2/", "", client).map(|body| {
            let r = decode(&body, "utf8");

            println!("--- Na hradě - menu ---");

            match Document::from_read(&mut r.as_bytes()) {
                Err(err) => {
                    println!("read body error: {}", err);
                    return;
                }
                Ok(doc) => na_hrade_basic_parser(doc),
            }
        });
    let fascila = begin!("http://www.restauracefascila.cz/denni-menu/", "", client).map(|body| {
        let r = decode(&body, "utf8");

        println!("--- Fascila - denní menu ---");

        match Document::from_read(&mut r.as_bytes()) {
            Err(err) => {
                println!("read body error: {}", err);
                return;
            }
            Ok(doc) => fascila(doc),
        }
    });

    let fascila_basic = begin!(
        "http://www.restauracefascila.cz/jidelni-listek/",
        "",
        client
    )
    .map(|body| {
        let r = decode(&body, "utf8");

        println!("--- Fascila - menu ---");
        match Document::from_read(&mut r.as_bytes()) {
            Err(err) => {
                println!("read body error: {}", err);
                return;
            }
            Ok(doc) => fascila_basic_parser(doc),
        }
    });

    let na_statku_basic = begin!(
        "https://www.menicka.cz/1236-restaurant-na-statku.html?t=jidelni-listek",
        "",
        client
    )
    .map(|body| {
        let r = decode(&body, "windows1250");

        println!("--- Na statku - menu ---");
        match Document::from_read(&mut r.as_bytes()) {
            Err(err) => {
                println!("read body error: {}", err);
                return;
            }
            Ok(doc) => na_statku_basic_parser(doc),
        }
    });

    let kocour_denni = begin!("https://www.ukocourahk.cz/denni-menu/", "", client).map(|body| {
        let r = decode(&body, "utf8");

        println!("--- U kocoura - denní menu ---");
        match Document::from_read(&mut r.as_bytes()) {
            Err(err) => {
                println!("read body error: {}", err);
                return;
            }
            Ok(doc) => kocour_denni_parser(doc),
        }
    });

    let beranek_denni =
        begin!("https://www.pivovarberanek.cz/#jidelni-listek", "", client).map(|body| {
            let r = decode(&body, "utf8");

            println!("--- Beránek - denní menu ---");
            match Document::from_read(&mut r.as_bytes()) {
                Err(err) => {
                    println!("read body error: {}", err);
                    return;
                }
                Ok(doc) => beranek_denni_parser(doc),
            }
        });

    let beranek_basic =
        begin!("https://www.pivovarberanek.cz/#jidelni-listek", "", client).map(|body| {
            let r = decode(&body, "utf8");

            println!("--- Beránek - menu ---");
            match Document::from_read(&mut r.as_bytes()) {
                Err(err) => {
                    println!("read body error: {}", err);
                    return;
                }
                Ok(doc) => beranek_basic_parser(doc),
            }
        });

    let l = na_hrade_poledni
        .join(na_hrade_basic)
        .join(fascila)
        .join(fascila_basic)
        .join(na_statku)
        .join(na_statku_basic)
        .join(kocour_denni)
        .join(beranek_denni)
        .join(beranek_basic);

    naplavka.join(l)
        .map(|(_res1, _res2)| {})
        .map_err(|err| {
            println!("3 stdout error: {:?}", err);
            }
        )
}

fn decode(body: &[u8], codec: &str) -> String {
    match codec {
        "utf8" => match UTF_8.decode(&body.to_vec(), DecoderTrap::Strict) {
            Ok(ok) => ok,
            Err(_err) => String::new(),
        },
        "windows1250" => match WINDOWS_1250.decode(&body.to_vec(), DecoderTrap::Strict) {
            Ok(ok) => ok,
            Err(_err) => String::new(),
        },
        _ => String::new(),
    }
}
