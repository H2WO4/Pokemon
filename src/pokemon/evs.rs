macro_rules! all_lower {
	($var:ident $($varr:ident)* <= $bound:literal) => {
		($var <= $bound) $(&& ($varr <= $bound))*
	};
}
macro_rules! sum_lower {
	($var:ident $($varr:ident)* <= $bound:literal) => {
		(($var as u16) $(+ ($varr as u16))*) <= $bound
	};
}

pub struct EVs {
    hp:     u8,
    atk:    u8,
    def:    u8,
    sp_atk: u8,
    sp_def: u8,
    spd:    u8,
}
impl EVs {
    pub const fn new(hp: u8, atk: u8, def: u8, sp_atk: u8, sp_def: u8, spd: u8) -> Option<Self> {
        if all_lower!(hp atk def sp_atk sp_def spd <= 252)
           && sum_lower!(hp atk def sp_atk sp_def spd <= 510)
        {
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
