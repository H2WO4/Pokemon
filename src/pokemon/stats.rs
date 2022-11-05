#[derive(Clone, Copy)]
pub struct Stats {
    hp:     u16,
    atk:    u16,
    def:    u16,
    sp_atk: u16,
    sp_def: u16,
    spd:    u16,
}
impl Stats {
    pub const fn new(hp: u16, atk: u16, def: u16, sp_atk: u16, sp_def: u16, spd: u16) -> Self {
        Self { hp,
               atk,
               def,
               sp_atk,
               sp_def,
               spd }
    }
}
