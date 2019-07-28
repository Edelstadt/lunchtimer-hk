pub use self::{
    beranek::fetch as beranek,
    fascila::fetch as fascila,
    u_kocoura::fetch as u_kocoura,
    sova::fetch as sova,
};

mod beranek;
mod fascila;
mod u_kocoura;
mod sova;
