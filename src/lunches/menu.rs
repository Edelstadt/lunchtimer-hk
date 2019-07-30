#[derive(Serialize)]
pub struct Menu {
    pub title: String,
    pub body:  Vec<MenuLine>,
}

pub struct MenuBody {
    pub amount: String,
    pub label:  String,
    pub price:  usize,
}

pub enum MenuLine {
    Title(String),
    Item(MenuBody),
}

impl Menu {
    pub fn new(title: &str) -> Self {
        Menu {
            title: title.to_string(),
            body:  vec![],
        }
    }
}

impl MenuBody {
    pub fn new(amount: &str, label: &str, price: usize) -> Self {
        MenuBody {
            amount: amount.to_string(),
            label: label.to_string(),
            price,
        }
    }

    pub fn empty() -> Self {
        MenuBody {
            amount: String::new(),
            label:  String::new(),
            price:  0,
        }
    }
}
