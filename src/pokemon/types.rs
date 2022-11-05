#[derive(Clone, Copy)]
pub enum Single {
    Normal,
    Fire,
    Water,
    Electric,
    Grass,
    Ice,
    Fighting,
    Poison,
    Ground,
    Flying,
    Psychic,
    Bug,
    Rock,
    Ghost,
    Dragon,
    Dark,
    Steel,
    Fairy,
}
impl Single {
    #[allow(clippy::too_many_lines)]
    pub fn against(self, other: PokeType) -> f32 {
        use Single::*;

        const NOT_EFFECTIVE: f32 = 0.5;
        const SUPER_EFFECTIVE: f32 = 2.0;
        const IMMUNE: f32 = 0.0;
        const NORMAL: f32 = 1.0;

        match other {
            PokeType::Single(other) => match self {
                Normal => match other {
                    Rock | Steel => NOT_EFFECTIVE,
                    Ghost => IMMUNE,
                    _ => NORMAL,
                },
                Fire => match other {
                    Fire | Water | Rock | Dragon => NOT_EFFECTIVE,
                    Grass | Ice | Bug | Steel => SUPER_EFFECTIVE,
                    _ => NORMAL,
                },
                Water => match other {
                    Water | Grass | Dragon => NOT_EFFECTIVE,
                    Fire | Ground | Rock => SUPER_EFFECTIVE,
                    _ => NORMAL,
                },
                Electric => match other {
                    Electric | Grass | Dragon => NOT_EFFECTIVE,
                    Water | Flying => SUPER_EFFECTIVE,
                    Ground => IMMUNE,
                    _ => NORMAL,
                },
                Grass => match other {
                    Fire | Grass | Poison | Flying | Bug | Dragon | Steel => NOT_EFFECTIVE,
                    Water | Ground | Rock => SUPER_EFFECTIVE,
                    _ => NORMAL,
                },
                Ice => match other {
                    Fire | Water | Ice | Steel => NOT_EFFECTIVE,
                    Grass | Ground | Flying | Dragon => SUPER_EFFECTIVE,
                    _ => NORMAL,
                },
                Fighting => match other {
                    Poison | Flying | Psychic | Bug | Fairy => NOT_EFFECTIVE,
                    Normal | Ice | Rock | Dark | Steel => SUPER_EFFECTIVE,
                    Ghost => IMMUNE,
                    _ => NORMAL,
                },
                Poison => match other {
                    Poison | Ground | Rock | Ghost => NOT_EFFECTIVE,
                    Grass | Fairy => SUPER_EFFECTIVE,
                    Steel => IMMUNE,
                    _ => NORMAL,
                },
                Ground => match other {
                    Grass | Bug => NOT_EFFECTIVE,
                    Fire | Electric | Poison | Rock | Steel => SUPER_EFFECTIVE,
                    Flying => IMMUNE,
                    _ => NORMAL,
                },
                Flying => match other {
                    Electric | Rock | Steel => NOT_EFFECTIVE,
                    Grass | Fighting | Bug => SUPER_EFFECTIVE,
                    _ => NORMAL,
                },
                Psychic => match other {
                    Psychic | Steel => NOT_EFFECTIVE,
                    Fighting | Poison => SUPER_EFFECTIVE,
                    Dark => IMMUNE,
                    _ => NORMAL,
                },
                Bug => match other {
                    Fire | Fighting | Poison | Flying | Ghost | Steel | Fairy => NOT_EFFECTIVE,
                    Grass | Psychic | Dark => SUPER_EFFECTIVE,
                    _ => NORMAL,
                },
                Rock => match other {
                    Fighting | Ground | Steel => NOT_EFFECTIVE,
                    Fire | Ice | Flying | Bug => SUPER_EFFECTIVE,
                    _ => NORMAL,
                },
                Ghost => match other {
                    Dark => NOT_EFFECTIVE,
                    Psychic | Ghost => SUPER_EFFECTIVE,
                    Normal => IMMUNE,
                    _ => NORMAL,
                },
                Dragon => match other {
                    Steel => NOT_EFFECTIVE,
                    Dragon => SUPER_EFFECTIVE,
                    Fairy => IMMUNE,
                    _ => NORMAL,
                },
                Dark => match other {
                    Fighting | Dark | Fairy => NOT_EFFECTIVE,
                    Psychic | Ghost => SUPER_EFFECTIVE,
                    _ => NORMAL,
                },
                Steel => match other {
                    Fire | Water | Electric | Steel => NOT_EFFECTIVE,
                    Ice | Rock | Fairy => SUPER_EFFECTIVE,
                    _ => NORMAL,
                },
                Fairy => match other {
                    Fire | Poison | Steel => NOT_EFFECTIVE,
                    Fighting | Dragon | Dark => SUPER_EFFECTIVE,
                    _ => NORMAL,
                },
            },
            PokeType::Dual(Dual(t1, t2)) => t1.against(other) * t2.against(other),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Dual(pub Single, pub Single);

#[derive(Clone, Copy)]
pub enum PokeType {
    Single(Single),
    Dual(Dual),
}
impl const From<Single> for PokeType {
    fn from(val: Single) -> Self {
        Self::Single(val)
    }
}
impl const From<Dual> for PokeType {
    fn from(val: Dual) -> Self {
        Self::Dual(val)
    }
}
