#![feature(async_await, await_macro, futures_api)]
mod lunches;

use lunches::fetch;
use std::sync::mpsc::{channel};

#[runtime::main]
async fn main() {
    let (tx, rx) = channel();

    let c = 10;
    for i in 0..c { // Spawn jobs
        runtime::spawn(fetch(tx.clone()));
    }

    for i in 0..c { // Await jobs
        dbg!(rx.recv());
    }
}

