use species::Species;

pub mod evs;
pub mod ivs;
pub mod nature;
pub mod species;
pub mod stats;
pub mod types;

pub use evs::EVs;
pub use ivs::IVs;
pub use nature::Nature;
pub use stats::Stats;
pub use types::{Dual, PokeType, Single};


pub struct Pokemon {
    species:  &'static Species,
    nickname: Option<String>,
    level:    u8,

    nature: Nature,
    ivs:    IVs,
    evs:    EVs,

    is_shiny: bool,
}
impl Pokemon {
    pub const fn build(species: &'static Species) -> builder::Const {
        builder::Const::new(species)
    }

    pub fn get_name(&self) -> &str {
        self.nickname
            .as_ref()
            .map_or(self.species.name, |name| name)
    }

    pub fn nickname<T: Into<String>>(&mut self, name: T) {
        self.nickname = Some(name.into());
    }
}

mod builder {
    use super::*;

    #[derive(Clone, Copy)]
    pub struct Const {
        species: &'static Species,
        level:   Option<u8>,

        nature: Option<Nature>,
        ivs:    Option<IVs>,
        evs:    Option<EVs>,

        is_shiny: bool,
    }
    impl Const {
        pub(super) const fn new(species: &'static Species) -> Self {
            Self { species,
                   level: None,

                   nature: None,
                   ivs: None,
                   evs: None,

                   is_shiny: false }
        }

        pub const fn level(self, level: u8) -> Self {
            Self { level: Some(level),
                   ..self }
        }

        pub const fn nature(self, nature: Nature) -> Self {
            Self { nature: Some(nature),
                   ..self }
        }

        pub const fn ivs(self, ivs: IVs) -> Self {
            Self { ivs: Some(ivs), ..self }
        }

        pub const fn evs(self, evs: EVs) -> Self {
            Self { evs: Some(evs), ..self }
        }

        pub const fn shiny(self) -> Self {
            Self { is_shiny: true, ..self }
        }

        pub const fn finish(self) -> Pokemon {
            Pokemon { species:  self.species,
                      nickname: None,
                      level:    self.level.unwrap_or(1),

                      nature: self.nature.unwrap(),
                      ivs:    self.ivs.unwrap_or_default(),
                      evs:    self.evs.unwrap_or_default(),

                      is_shiny: self.is_shiny, }
        }

        pub fn nickname<T: Into<String>>(self, name: T) -> Normal {
            Normal { species:  self.species,
                     nickname: Some(name.into()),
                     level:    self.level,

                     nature: self.nature,
                     ivs:    self.ivs,
                     evs:    self.evs,

                     is_shiny: self.is_shiny, }
        }
    }

    pub struct Normal {
        species:  &'static Species,
        level:    Option<u8>,
        nickname: Option<String>,

        nature: Option<Nature>,
        ivs:    Option<IVs>,
        evs:    Option<EVs>,

        is_shiny: bool,
    }
    #[allow(clippy::missing_const_for_fn)]
    impl Normal {
        pub fn level(self, level: u8) -> Self {
            Self { level: Some(level),
                   ..self }
        }

        pub fn ivs(self, ivs: IVs) -> Self {
            Self { ivs: Some(ivs), ..self }
        }

        pub fn evs(self, evs: EVs) -> Self {
            Self { evs: Some(evs), ..self }
        }

        pub fn shiny(self) -> Self {
            Self { is_shiny: true, ..self }
        }

        pub fn nickname<T: Into<String>>(self, name: T) -> Self {
            Self { nickname: Some(name.into()),
                   ..self }
        }

        pub fn finish(self) -> Pokemon {
            Pokemon { species:  self.species,
                      nickname: self.nickname,
                      level:    self.level.unwrap_or(1),

                      nature: self.nature.unwrap_or_else(Nature::random),
                      ivs:    self.ivs.unwrap_or_default(),
                      evs:    self.evs.unwrap_or_default(),

                      is_shiny: self.is_shiny, }
        }
    }
}
