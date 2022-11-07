use super::super::species::*;
use super::{Line, LineStage};

pub const SQUIRTLE_LINE: Line = Line::build().base(&SQUIRTLE)
                                             .second(LineStage::new(&WARTORTLE, 16))
                                             .third(LineStage::new(&BLASTOISE, 36))
                                             .finish();
