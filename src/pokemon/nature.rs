use Nature::*;
use NatureMod::*;

pub enum NatureMod {
    Increase,
    Decrease,
    Neutral,
}
impl NatureMod {
    pub const fn val(&self) -> f32 {
        match self {
            Increase => 1.1,
            Decrease => 0.9,
            Neutral => 1.0,
        }
    }
}

pub enum Nature {
    Adamant,
    Bashful,
    Bold,
    Brave,
    Calm,
    Careful,
    Docile,
    Gentle,
    Hardy,
    Hasty,
    Impish,
    Jolly,
    Lax,
    Lonely,
    Mild,
    Modest,
    Naive,
    Naughty,
    Quiet,
    Quirky,
    Rash,
    Relaxed,
    Sassy,
    Serious,
    Timid,
}
impl Nature {
    pub fn atk(self) -> NatureMod {
        match self {
            Lonely | Brave | Adamant | Naughty => Increase,
            Bold | Timid | Modest | Calm => Decrease,
            _ => Neutral,
        }
    }

    pub fn def(self) -> NatureMod {
        match self {
            Bold | Relaxed | Impish | Lax => Increase,
            Lonely | Hasty | Mild | Gentle => Decrease,
            _ => Neutral,
        }
    }

    pub fn sp_atk(self) -> NatureMod {
        match self {
            Modest | Mild | Rash | Quiet => Increase,
            Adamant | Impish | Jolly | Careful => Decrease,
            _ => Neutral,
        }
    }

    pub fn sp_def(self) -> NatureMod {
        match self {
            Calm | Gentle | Sassy | Careful => Increase,
            Naughty | Lax | Naive | Rash => Decrease,
            _ => Neutral,
        }
    }

    pub fn spd(self) -> NatureMod {
        match self {
            Timid | Hasty | Jolly | Naive => Increase,
            Brave | Relaxed | Quiet | Sassy => Decrease,
            _ => Neutral,
        }
    }
}
