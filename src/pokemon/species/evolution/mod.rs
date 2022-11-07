use super::Species;

pub mod lines;
pub use lines::*;

pub struct Line {
    base:   &'static Species,
    second: LineStage,
    third:  Option<LineStage>,
}
impl Line {
    pub const fn build() -> builder::Empty {
        builder::Empty
    }
}

pub struct LineStage {
    species: &'static Species,
    level:   u8,
}
impl LineStage {
    pub const fn new(species: &'static Species, level: u8) -> Self {
        Self { species, level }
    }
}

pub enum Stage {
    Base,
    Second,
    Third,
}


mod builder {
    use super::*;

    pub struct Empty;
    impl Empty {
        pub const fn base(self, species: &'static Species) -> WithBase {
            WithBase { base: species }
        }
    }

    pub struct WithBase {
        base: &'static Species,
    }
    impl WithBase {
        pub const fn second(self, stage: LineStage) -> WithSecond {
            WithSecond { base:   self.base,
                         second: stage, }
        }
    }

    pub struct WithSecond {
        base:   &'static Species,
        second: LineStage,
    }
    impl WithSecond {
        pub const fn third(self, stage: LineStage) -> Normal {
            Normal { base:   self.base,
                     second: self.second,
                     third:  stage, }
        }

        pub const fn finish(self) -> Line {
            Line { base:   self.base,
                   second: self.second,
                   third:  None, }
        }
    }

    pub struct Normal {
        base:   &'static Species,
        second: LineStage,
        third:  LineStage,
    }
    impl Normal {
        pub const fn finish(self) -> Line {
            Line { base:   self.base,
                   second: self.second,
                   third:  Some(self.third), }
        }
    }
}
