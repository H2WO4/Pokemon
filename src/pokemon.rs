use species::Species;

pub mod evs;
pub mod species;

#[derive(Clone, Copy)]
pub struct Stats {
    hp:     u8,
    atk:    u8,
    def:    u8,
    sp_atk: u8,
    sp_def: u8,
    spd:    u8,
}
impl Stats {
    pub const fn new(hp: u8, atk: u8, def: u8, sp_atk: u8, sp_def: u8, spd: u8) -> Self {
        Self { hp,
               atk,
               def,
               sp_atk,
               sp_def,
               spd }
    }
}

pub struct IVs {
    hp:     u8,
    atk:    u8,
    def:    u8,
    sp_atk: u8,
    sp_def: u8,
    spd:    u8,
}
impl IVs {
    pub const fn new(hp: u8, atk: u8, def: u8, sp_atk: u8, sp_def: u8, spd: u8) -> Self {
        if hp > 31 || atk > 31 || def > 31 || sp_atk > 31 || sp_def > 31 || spd > 31 {
            panic!()
        } else {
            Self { hp,
                   atk,
                   def,
                   sp_atk,
                   sp_def,
                   spd }
        }
    }
}

pub struct Pokemon {
    species: Species,
    name:    Option<String>,
    level:   u8,

    is_shiny: bool,
}
impl Pokemon {
    pub const fn build(species: Species) -> builder::Normal {
        builder::Normal::new(species)
    }

    pub fn get_name(&self) -> &str {
        self.name
            .as_ref()
            .map_or(self.species.name, |name| name)
    }
}

mod builder {
    use super::*;

    pub struct Normal {
        species: Species,
        level:   Option<u8>,
        name:    Option<String>,

        is_shiny: bool,
    }
    impl Normal {
        pub(super) const fn new(species: Species) -> Self {
            Self { species,
                   level: None,
                   name: None,
                   is_shiny: false }
        }

        pub fn level(self, level: u8) -> Self {
            Self { level: Some(level),
                   ..self }
        }

        pub fn shiny(self) -> Self {
            Self { is_shiny: true,
                   ..self }
        }

        pub fn name<T: Into<String>>(self, name: T) -> Self {
            Self { species:  self.species,
                   level:    self.level,
                   name:     Some(name.into()),
                   is_shiny: self.is_shiny, }
        }

        pub fn finish(self) -> Pokemon {
            Pokemon { species: self.species,
                      name:    self.name,
                      level:   self.level.unwrap_or(1),

                      is_shiny: self.is_shiny, }
        }
    }
}
