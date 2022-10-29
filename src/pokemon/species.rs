use crate::pokemon::Stats;
use crate::types::PokeType;

pub mod vals;
pub use vals::*;

pub enum PokeClass {
    Normal,
    Mythical,
    Legendary,
    Baby,
}

pub enum Generation {
    I,
    II,
    III,
    IV,
    V,
    VI,
    VII,
    VIII,
    IX,
}

#[derive(Clone, Copy)]
pub struct Species {
    pub(super) id:    u16,
    pub(super) name:  &'static str,
    pub(super) stats: Stats,
    pub(super) ptype: PokeType,
}
impl Species {
    const fn build(id: u16) -> builder::WithId {
        builder::WithId::new(id)
    }
}

mod builder {
    use super::*;

    pub struct WithId {
        id: u16,
    }
    impl WithId {
        pub(super) const fn new(id: u16) -> Self {
            Self { id }
        }

        pub const fn name(self, name: &'static str) -> WithName {
            WithName { id: self.id, name }
        }
    }

    pub struct WithName {
        id:   u16,
        name: &'static str,
    }
    impl WithName {
        pub const fn gen(self, gen: Generation) -> WithGen {
            WithGen { id: self.id,
                      name: self.name,
                      gen }
        }
    }

    pub struct WithGen {
        id:   u16,
        name: &'static str,
        gen:  Generation,
    }
    impl WithGen {
        pub const fn stats(self, stats: Stats) -> WithStats {
            WithStats { id: self.id,
                        name: self.name,
                        gen: self.gen,
                        stats }
        }
    }

    pub struct WithStats {
        id:    u16,
        name:  &'static str,
        gen:   Generation,
        stats: Stats,
    }
    impl WithStats {
        pub const fn ptype<T: ~const Into<PokeType>>(self, ptype: T) -> Normal {
            Normal::new(self.id, self.name, self.gen, self.stats, ptype.into())
        }
    }

    pub struct Normal {
        id:    u16,
        name:  &'static str,
        gen:   Generation,
        stats: Stats,
        ptype: PokeType,

        class:     PokeClass,
        evolve_to: Option<Species>,
    }
    impl Normal {
        const fn new(id: u16,
                     name: &'static str,
                     gen: Generation,
                     stats: Stats,
                     ptype: PokeType)
                     -> Self {
            Self { id,
                   name,
                   gen,
                   stats,
                   ptype,
                   class: PokeClass::Normal,
                   evolve_to: None }
        }

        pub const fn class(self, class: PokeClass) -> Self {
            Self { class, ..self }
        }

        pub const fn evolve_to(self, evolve_to: Species) -> Self {
            Self { evolve_to: Some(evolve_to),
                   ..self }
        }

        pub const fn finish(self) -> Species {
            Species { id:    self.id,
                      name:  self.name,
                      stats: self.stats,
                      ptype: self.ptype, }
        }
    }
}
