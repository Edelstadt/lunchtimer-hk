extern crate tokio;
use lunchtime::fetch;

fn main() {
//    let pattern = std::env::args().nth(1).expect("no pattern given");
    tokio::run(fetch());
}
