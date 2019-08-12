pub(crate) use self::{
    beranek::fetch as beranek,
    fascila::fetch as fascila,
    naplavka::fetch as naplavka,
    sova::fetch as sova,
    u_kocoura::fetch as u_kocoura,
};

mod beranek;
mod fascila;
mod naplavka;
mod sova;
mod u_kocoura;
