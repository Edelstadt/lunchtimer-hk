use std::sync::Mutex;
use core::borrow::Borrow;

#[derive(Serialize)]
pub struct Menu {
    pub id: u8, // TODO private
    pub title: String,
    pub body: String,
}

static mut MENUS: Option<Vec<Menu>> = None; // TODO Mutex

pub fn get_menus() -> &'static [Menu] {
    unsafe {
        match MENUS {
            Some(ref x) => x,
            None => panic!("uninitialized!"),
        }
    }
}

pub fn set_menus(mut data: Vec<Menu>) {
    unsafe {
        match MENUS {
            Some(ref mut x) => {
                x.clear();
                x.append(&mut data);
            },
            None => panic!("uninitialized!"),
        }
    }
}

pub fn init_menus() {
    unsafe {
        MENUS = Some(vec![]);
    }
}
