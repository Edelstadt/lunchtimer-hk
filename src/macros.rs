#[macro_export]
macro_rules! menicka {
    ($var_name: ident, $id:expr, $name: expr, $client:expr) => {
        let $var_name = $client
            .get(format!("{}{}", "https://www.menicka.cz/api/iframe/?id=", $id).as_str())
            .send()
            .and_then(|mut res| {
                let body = mem::replace(res.body_mut(), Decoder::empty());
                body.concat2()
            })
            .map_err(|err| println!("request error: {}", err))
            .map(|body| {
                let r = decode(&body, "windows1250");

                println!($name);
                match Document::from_read(&mut r.as_bytes()) {
                    Err(err) => {
                        println!("read body error: {}", err);
                        return;
                    }
                    Ok(doc) => menicka_parser(doc),
                }
            });
    };
}

#[macro_export]
macro_rules! begin {
    ($url:expr, $name: expr, $client:expr) => {
        $client
            .get($url)
            .send()
            .and_then(|mut res| {
                println!($name);
                let body = mem::replace(res.body_mut(), Decoder::empty());
                body.concat2()
            })
            .map_err(|err| println!("request error: {}", err))
    };
}
