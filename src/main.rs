#![feature(async_await)]
mod lunches;

use lunches::fetch;
use std::sync::mpsc::{channel};
use nickel::{Nickel, HttpRouter, Request, Response, MiddlewareResult};

#[runtime::main]
async fn main() {
    let mut server = Nickel::new();
    server.get("**", lunch);
    server.listen("127.0.0.1:80").expect("");

    let (tx, rx) = channel();

    let c = 10;
    for i in 0..c { // Spawn jobs
        runtime::spawn(fetch(tx.clone()));
    }

    for i in 0..11 { // Await jobs
        dbg!(rx.recv());
    }
}

fn lunch<'mw>(_req: &mut Request, res: Response<'mw>) -> MiddlewareResult<'mw> {
    res.send("Fuck world!")
}

