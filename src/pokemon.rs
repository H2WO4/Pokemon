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


#[macro_export]
macro_rules! all_lower {
	($var:ident $($varr:ident)* <= $bound:literal) => {
		($var <= $bound) $(&& ($varr <= $bound))*
	};
}
#[macro_export]
macro_rules! sum_lower {
	($var:ident $($varr:ident)* <= $bound:literal) => {
		(($var as u16) $(+ ($varr as u16))*) <= $bound
	};
}


pub struct Pokemon {
    species: Species,
    name:    Option<String>,
    level:   u8,

    is_shiny: bool,
}
impl Pokemon {
    pub const fn build(species: Species) -> builder::Const {
        builder::Const::new(species)
    }

    pub fn get_name(&self) -> &str {
        self.name
            .as_ref()
            .map_or(self.species.name, |name| name)
    }

    pub fn nickname<T: Into<String>>(&mut self, name: T) {
        self.name = Some(name.into());
    }
}

mod builder {
    use super::*;

    #[derive(Clone, Copy)]
    pub struct Const {
        species: Species,
        level:   Option<u8>,

        is_shiny: bool,
        ivs:      Option<IVs>,
    }
    impl Const {
        pub(super) const fn new(species: Species) -> Self {
            Self { species,
                   level: None,

                   is_shiny: false,
                   ivs: None }
        }

        pub const fn level(self, level: u8) -> Self {
            Self { level: Some(level),
                   ..self }
        }

        pub const fn shiny(self) -> Self {
            Self { is_shiny: true,
                   ..self }
        }

        pub const fn ivs(self, ivs: IVs) {
            todo!()
        }

        pub const fn finish(self) -> Pokemon {
            Pokemon { species:  self.species,
                      name:     None,
                      level:    self.level.unwrap_or(100),
                      is_shiny: self.is_shiny, }
        }

        pub fn name<T: Into<String>>(self, name: T) -> Normal {
            Normal { species:  self.species,
                     name:     Some(name.into()),
                     level:    self.level,
                     is_shiny: self.is_shiny, }
        }
    }

    pub struct Normal {
        species: Species,
        level:   Option<u8>,
        name:    Option<String>,

        is_shiny: bool,
    }
    impl Normal {
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
