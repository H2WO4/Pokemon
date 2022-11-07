use rand::seq::SliceRandom;
use rand::thread_rng;
use strum::EnumCount;
use strum_macros::EnumCount;
use Modifier::*;
use Nature::*;

pub enum Modifier {
    Increase,
    Decrease,
    Neutral,
}
impl Modifier {
    pub const fn val(&self) -> f32 {
        match self {
            Increase => 1.1,
            Decrease => 0.9,
            Neutral => 1.0,
        }
    }
}

#[derive(Clone, Copy, EnumCount)]
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
    pub const fn atk(self) -> Modifier {
        match self {
            Lonely | Brave | Adamant | Naughty => Increase,
            Bold | Timid | Modest | Calm => Decrease,
            _ => Neutral,
        }
    }

    pub const fn def(self) -> Modifier {
        match self {
            Bold | Relaxed | Impish | Lax => Increase,
            Lonely | Hasty | Mild | Gentle => Decrease,
            _ => Neutral,
        }
    }

    pub const fn sp_atk(self) -> Modifier {
        match self {
            Modest | Mild | Rash | Quiet => Increase,
            Adamant | Impish | Jolly | Careful => Decrease,
            _ => Neutral,
        }
    }

    pub const fn sp_def(self) -> Modifier {
        match self {
            Calm | Gentle | Sassy | Careful => Increase,
            Naughty | Lax | Naive | Rash => Decrease,
            _ => Neutral,
        }
    }

    pub const fn spd(self) -> Modifier {
        match self {
            Timid | Hasty | Jolly | Naive => Increase,
            Brave | Relaxed | Quiet | Sassy => Decrease,
            _ => Neutral,
        }
    }
}
impl Nature {
    pub fn random() -> Self {
        const VALS: [Nature; Nature::COUNT] = [Adamant, Bashful, Bold, Brave, Calm, Careful,
                                               Docile, Gentle, Hardy, Hasty, Impish, Jolly, Lax,
                                               Lonely, Mild, Modest, Naive, Naughty, Quiet,
                                               Quirky, Rash, Relaxed, Sassy, Serious, Timid];
        *VALS.choose(&mut thread_rng()).unwrap()
    }
}
