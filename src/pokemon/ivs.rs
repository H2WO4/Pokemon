use crate::any_is;

#[derive(Clone, Copy)]
pub struct IVs {
    hp:     u8,
    atk:    u8,
    def:    u8,
    sp_atk: u8,
    sp_def: u8,
    spd:    u8,
}
impl IVs {
    pub const fn new(hp: u8, atk: u8, def: u8, sp_atk: u8, sp_def: u8, spd: u8) -> Option<Self> {
        if any_is!(hp atk def sp_atk sp_def spd > 31) {
            None
        } else {
            Some(Self { hp,
                        atk,
                        def,
                        sp_atk,
                        sp_def,
                        spd })
        }
    }
}
#[allow(clippy::derivable_impls)]
impl const Default for IVs {
    fn default() -> Self {
        Self { hp:     0,
               atk:    0,
               def:    0,
               sp_atk: 0,
               sp_def: 0,
               spd:    0, }
    }
}
