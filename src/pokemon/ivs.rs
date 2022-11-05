use crate::all_lower;

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
    pub const fn new(hp: u8, atk: u8, def: u8, sp_atk: u8, sp_def: u8, spd: u8) -> Self {
        if !all_lower!(hp atk def sp_atk sp_def spd <= 31) {
            panic!()
        } else {
            Self { hp,
                   atk,
                   def,
                   sp_atk,
                   sp_def,
                   spd }
        }
    }
}
