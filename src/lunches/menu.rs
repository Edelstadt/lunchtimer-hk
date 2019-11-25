#[derive(Serialize)]
pub struct HtmlMenu {
    id:        usize,
    pub title: String,
    pub body:  Vec<HtmlBodyType>,
}

#[derive(Serialize)]
pub struct HtmlTitleLine {
    type_title: bool,
    pub title:  String,
}

#[derive(Serialize)]
pub struct HtmlBodyLine {
    type_body: bool,
    pub label: String,
    pub price: String,
}

#[derive(Serialize)]
pub enum HtmlBodyType {
    Title(HtmlTitleLine),
    Line(HtmlBodyLine),
}

pub struct Menu {
    pub title: String,
    pub body:  Vec<MenuLine>,
}

pub struct MenuBody {
    pub label: String,
    pub price: String,
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
    pub fn new() -> Self {
        MenuBody {
            label: String::new(),
            price: String::new(),
        }
    }
}

impl HtmlMenu {
    pub fn new(title: String) -> Self {
        HtmlMenu {
            id: 0,
            title,
            body: vec![],
        }
    }

    pub fn set_id(&mut self, id: usize) {
        self.id = id;
    }
}

impl From<Menu> for HtmlMenu {
    fn from(menu: Menu) -> Self {
        let mut html = HtmlMenu::new(menu.title);

        for line in menu.body {
            match line {
                MenuLine::Title(x) => {
                    html.body.push(HtmlBodyType::Title(HtmlTitleLine {
                        type_title: true,
                        title:      x,
                    }));
                },
                MenuLine::Item(x) => {
                    html.body.push(HtmlBodyType::Line(HtmlBodyLine {
                        type_body: true,
                        label:     x.label,
                        price:     x.price,
                    }));
                },
            }
        }

        html
    }
}
