use crate::pokemon::Stats;
use crate::types::PokeType;

pub mod vals;
pub use vals::*;

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
        pub const fn stats(self, stats: Stats) -> WithStats {
            WithStats { id: self.id,
                        name: self.name,
                        stats }
        }
    }

    pub struct WithStats {
        id:    u16,
        name:  &'static str,
        stats: Stats,
    }
    impl WithStats {
        pub const fn ptype<T: ~const Into<PokeType>>(self, ptype: T) -> Normal {
            Normal { id:        self.id,
                     name:      self.name,
                     stats:     self.stats,
                     ptype:     ptype.into(),
                     evolve_to: None, }
        }
    }

    pub struct Normal {
        id:    u16,
        name:  &'static str,
        stats: Stats,
        ptype: PokeType,

        evolve_to: Option<Species>,
    }
    impl Normal {
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
