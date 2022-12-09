use crate::pokemon::stats::Stats;
use crate::pokemon::PokeType;

pub mod evolution;
// pub mod species;

// use evolution::LineInner;
// pub use species::*;


#[derive(Clone, Copy)]
pub enum PokeClass {
    Normal,
    Mythical,
    Legendary,
    Baby,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy)]
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


mod builder2 {
    use super::*;


    #[derive(Clone, Copy)]
    pub struct Builder<Name, Genus, Gen, Stats, PType, Class, Form> {
        id: u16,

        name:  Name,
        genus: Genus,
        gen:   Gen,
        stats: Stats,
        ptype: PType,

        class:   Class,
        is_form: Form,
    }

    #[derive(Clone, Copy)]
    struct HasName(&'static str);
    #[derive(Clone, Copy)]
    pub struct NoName;

    #[derive(Clone, Copy)]
    struct HasGenus(&'static str);
    #[derive(Clone, Copy)]
    pub struct NoGenus;

    #[derive(Clone, Copy)]
    struct HasGen(Gen);
    #[derive(Clone, Copy)]
    pub struct NoGen;

    #[derive(Clone, Copy)]
    struct HasStats(Stats);
    #[derive(Clone, Copy)]
    pub struct NoStats;

    #[derive(Clone, Copy)]
    struct HasType(PokeType);
    #[derive(Clone, Copy)]
    pub struct NoType;

    #[const_trait]
    pub trait MayClass {
        fn get(self) -> PokeClass;
    }
    #[derive(Clone, Copy)]
    struct HasClass(PokeClass);
    impl const MayClass for HasClass {
        fn get(self) -> PokeClass {
            self.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct NoClass;
    impl const MayClass for NoClass {
        fn get(self) -> PokeClass {
            PokeClass::Normal
        }
    }

    #[const_trait]
    pub trait MayForm {
        fn get(self) -> bool;
    }
    #[derive(Clone, Copy)]
    struct IsForm;
    impl const MayForm for IsForm {
        fn get(self) -> bool {
            true
        }
    }
    #[derive(Clone, Copy)]
    pub struct NotForm;
    impl const MayForm for NotForm {
        fn get(self) -> bool {
            false
        }
    }


    pub type Start = Builder<NoName, NoGenus, NoGen, NoStats, NoType, NoClass, NotForm>;
    impl Start {
        pub const fn new(id: u16) -> Self {
            Self { id,

                   name: NoName,
                   genus: NoGenus,
                   gen: NoGen,
                   stats: NoStats,
                   ptype: NoType,

                   class: NoClass,
                   is_form: NotForm }
        }
    }

    impl<_1, _2, _3, _4, _5: MayClass, _6: MayForm> Builder<NoName, _1, _2, _3, _4, _5, _6> {
        pub const fn name(self, name: &'static str) -> Builder<HasName, _1, _2, _3, _4, _5, _6> {
            Builder { name: HasName(name),
                      ..self }
        }
    }

    impl<_1, _2, _3, _4, _5: MayClass, _6: MayForm> Builder<_1, NoGenus, _2, _3, _4, _5, _6> {
        pub const fn genus(self, genus: &'static str) -> Builder<_1, HasGenus, _2, _3, _4, _5, _6> {
            Builder { genus: HasGenus(genus),
                      ..self }
        }
    }

    impl<_1, _2, _3, _4, _5: MayClass, _6: MayForm> Builder<_1, _2, NoGen, _3, _4, _5, _6> {
        pub const fn gen(self, gen: Gen) -> Builder<_1, _2, HasGen, _3, _4, _5, _6> {
            Builder { gen: HasGen(gen),
                      ..self }
        }
    }

    impl<_1, _2, _3, _4, _5: MayClass, _6: MayForm> Builder<_1, _2, _3, NoStats, _4, _5, _6> {
        pub const fn stats(self, stats: Stats) -> Builder<_1, _2, _3, HasStats, _4, _5, _6> {
            Builder { stats: HasStats(stats),
                      ..self }
        }
    }

    impl<_1, _2, _3, _4, _5: MayClass, _6: MayForm> Builder<_1, _2, _3, _4, NoType, _5, _6> {
        pub const fn ptype<T: ~const Into<PokeType>>(self, ptype: T) -> Builder<_1, _2, _3, _4, HasType, _5, _6> {
            Builder { ptype: HasType(ptype.into()),
                      ..self }
        }
    }

    impl<_1, _2, _3, _4, _5, _6: MayForm> Builder<_1, _2, _3, _4, _5, NoClass, _6> {
        pub const fn class(self, class: PokeClass) -> Builder<_1, _2, _3, _4, _5, HasClass, _6> {
            Builder { class: HasClass(class),
                      ..self }
        }
    }

    impl<_1, _2, _3, _4, _5, _6: MayClass> Builder<_1, _2, _3, _4, _5, _6, NotForm> {
        pub const fn form(self) -> Builder<_1, _2, _3, _4, _5, _6, IsForm> {
            Builder { is_form: IsForm,
                      ..self }
        }
    }

    impl<Class, Form> Builder<HasName, HasGenus, HasGen, HasStats, HasType, Class, Form> {
        pub const fn finish(self) -> Species {
            Species { id:    self.id,
                      name:  self.name.0,
                      stats: self.stats.0,
                      ptype: self.ptype.0, }
        }
    }
}
