pub(crate) use self::{
    beranek::fetch as beranek,
    fascila::fetch as fascila,
    naplavka::fetch as naplavka,
    petr::fetch as petr,
    sova::fetch as sova,
    u_kocoura::fetch as u_kocoura,
    menicka::fetch as menicka,
};

mod beranek;
mod fascila;
mod naplavka;
mod petr;
mod sova;
mod u_kocoura;
mod menicka;
