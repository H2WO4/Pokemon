use crate::pokemon::stats::Stats;
use crate::pokemon::PokeType;

pub mod evolution;
pub mod species;

// use evolution::LineInner;
pub use species::*;


pub enum PokeClass {
    Normal,
    Mythical,
    Legendary,
    Baby,
}

#[allow(clippy::upper_case_acronyms)]
pub enum Gen {
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
    // pub(super) evolution: Option<LineInner<2>>,
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
        pub const fn genus(self, genus: &'static str) -> WithGenus {
            WithGenus { id: self.id,
                        name: self.name,
                        genus }
        }
    }

    pub struct WithGenus {
        id:    u16,
        name:  &'static str,
        genus: &'static str,
    }
    impl WithGenus {
        pub const fn gen(self, gen: Gen) -> WithGen {
            WithGen { id: self.id,
                      name: self.name,
                      genus: self.genus,
                      gen }
        }
    }

    pub struct WithGen {
        id:    u16,
        name:  &'static str,
        genus: &'static str,
        gen:   Gen,
    }
    impl WithGen {
        pub const fn stats(self, stats: Stats) -> WithStats {
            WithStats { id: self.id,
                        name: self.name,
                        genus: self.genus,
                        gen: self.gen,
                        stats }
        }
    }

    pub struct WithStats {
        id:    u16,
        name:  &'static str,
        genus: &'static str,
        gen:   Gen,
        stats: Stats,
    }
    impl WithStats {
        pub const fn ptype<T: ~const Into<PokeType>>(self, ptype: T) -> Normal {
            Normal { id:    self.id,
                     name:  self.name,
                     genus: self.genus,
                     gen:   self.gen,
                     stats: self.stats,
                     ptype: ptype.into(),

                     class:     PokeClass::Normal,
                    //  evolution: None,
                     is_form:   false, }
        }
    }

    pub struct Normal {
        id:    u16,
        name:  &'static str,
        genus: &'static str,
        gen:   Gen,
        stats: Stats,
        ptype: PokeType,

        class:     PokeClass,
        // evolution: Option<LineInner<2>>,
        is_form:   bool,
    }
    impl Normal {
        pub const fn class(self, class: PokeClass) -> Self {
            Self { class, ..self }
        }

        // pub const fn evolution(self, evolution: LineInner<2>) -> Self {
        //     Self { evolution: Some(evolution),
        //            ..self }
        // }

        pub const fn form(self) -> Self {
            Self { is_form: true, ..self }
        }

        pub const fn finish(self) -> Species {
            Species { id:    self.id,
                      name:  self.name,
                      stats: self.stats,
                      ptype: self.ptype, }
            //   evolution: self.evolution, }
        }
    }
}
