use chrono::{Datelike, Utc};
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};

pub fn menicka_parser(doc: Document) {
    for node in doc.find(Name("div")) {
        let now = Utc::now();
        let rr = format!("{}.{}.{}", now.day(), now.month(), now.year());
        if node.text().contains(rr.as_str()) {
            let t = node.text();
            println!("{}\n", t.trim_matches('\n'));
        }
    }
}

pub fn kocour_denni_parser(doc: Document) {
    for node in doc.find(Class("cms-content")) {
        for tr in node.find(Name("tr")) {
            let mut r = String::new();
            for (_, element) in tr.text().split_whitespace().enumerate() {
                r += format!(" {}", element).as_str()
            }
            println!("{}", r);
        }
    }
}

pub fn beranek_denni_parser(doc: Document) {
    for node in doc.find(Class("nabidkatext")) {
        let now = Utc::now();
        let rr = format!("{}.{}", now.day(), now.month());

        let re = regex::Regex::new(r"(?P<date>\D+\s\d+\.\d+\.)(?P<soup>\D+)(?P<menu1>Menu\s\d+:\s\D+\s\d{1,4},-)(?P<menu2>Menu\s\d:\s\D+\d{1,4},-)").unwrap();
        for part in re.captures_iter(node.text().as_str()) {
            let date = part.name("date").unwrap();
            if date.as_str().contains(rr.as_str()) {
                println!("{}", date.as_str());
                println!("{}", part.name("soup").unwrap().as_str());
                println!("{}", part.name("menu1").unwrap().as_str());
                println!("{}", part.name("menu2").unwrap().as_str());
            }
        }
    }
}

pub fn na_statku_basic_parser(doc: Document) {
    for node in doc.find(Class("menicka")) {
        let uls = node.find(Name("ul"));
        let nadpisy = node.find(Class("nadpis"));
        for it in nadpisy.zip(uls) {
            let (nadpis, ul) = it;
            println!("{}", nadpis.text().as_str());
            for li in ul.find(Name("li")) {
                let mut r = String::new();
                for (_, element) in li.text().split_whitespace().enumerate() {
                    r += format!(" {}", element).as_str()
                }
                println!("{}", r);
            }
        }
    }
}

pub fn fascila(doc: Document) {
    let mut gg: usize = 0;
    for (index, node) in doc.find(Class("wpb_wrapper")).enumerate() {
        node.find(Name("h2")).enumerate().for_each(|(_, _)| {
            let now = Utc::now();
            let rr = format!("{}.{}", now.day(), now.month());
            if node.text().contains(rr.as_str()) {
                gg = index + 1;
            }
        });

        if gg != 0 && index == gg {
            for node_i in node.find(Class("vc_row")) {
                let mut r = String::new();
                for (_, element) in node_i.text().split_whitespace().enumerate() {
                    r += format!(" {}", element).as_str()
                }
                println!("{}", r);
            }
        }
    }
}

pub fn fascila_basic_parser(doc: Document) {
    let mut title = String::new();

    for node in doc.find(Class("wpb_wrapper")) {
        for node_i in node.find(Name("span")) {
            if title.contains(node_i.text().as_str()) {
                continue;
            } else {
                println!("{}", node_i.text());
                title += format!(" {} ", node_i.text()).as_str()
            }
        }
        node.find(Class("vc_row"))
            .enumerate()
            .for_each(|(_, node_i)| {
                let mut r = String::new();
                for (_, element) in node_i.text().split_whitespace().enumerate() {
                    r += format!(" {}", element).as_str()
                }
                println!("{}", r);
            });
    }
}

pub fn na_hrade_basic_parser(doc: Document) {
    for node in doc.find(Attr("id", "cat-detail")) {
        for node_i in node.find(Name("div").descendant(Name("p"))) {
            let mut r = String::new();
            for (_, element) in node_i.text().split_whitespace().enumerate() {
                r += format!(" {}", element).as_str()
            }
            println!("{}", r);
        }
    }
}

pub fn beranek_basic_parser(doc: Document) {
    for node in doc.find(Class("content-text").descendant(Name("p"))) {
        let mut r = String::new();
        for (_, element) in node.text().split_whitespace().enumerate() {
            r += format!(" {}", element).as_str()
        }
        let re = regex::Regex::new(r"(?P<name>\D+)\s(?P<price>\d+,-(?P<n>/\d+,-)?)").unwrap();
        for part in re.captures_iter(r.as_str()) {
            let l: &str;
            if part.name("n").is_none() {
                l = "";
            } else {
                l = part.name("n").unwrap().as_str()
            }
            println!(
                "{} {}{}",
                part.name("name").unwrap().as_str(),
                part.name("price").unwrap().as_str(),
                l
            );
        }
    }
}
