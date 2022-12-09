use super::Species;

pub struct Stage {
    species: &'static Species,
}

pub enum Method {
    Level(u8),
}
