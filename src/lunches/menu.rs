#[derive(Serialize)]
pub struct HtmlMenu {
    pub id: usize,
    pub title: String,
    pub body: String,
}

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
    pub fn new(amount: String, label: String, price: usize) -> Self {
        MenuBody {
            amount,
            label,
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

impl HtmlMenu {
    pub fn new(title: String) -> Self {
        HtmlMenu {
            id: 0,
            title,
            body: String::new(),
        }
    }
}
