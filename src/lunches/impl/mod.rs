pub(crate) use self::{
    beranek::fetch as beranek,
    sova::fetch as sova,
    u_kocoura::fetch as u_kocoura,
};

mod beranek;
mod sova;
mod u_kocoura;
