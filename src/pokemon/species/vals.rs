use crate::pokemon::species::{Gen, PokeClass};
use crate::pokemon::Single::*;
use crate::pokemon::{Dual, Species, Stats};


pub const SQUIRTLE: Species = Species::build(1).name("Squirtle")
                                               .genus("Tiny Turtle Pokemon")
                                               .gen(Gen::I)
                                               .stats(Stats::new(44, 48, 65, 50, 64, 43))
                                               .ptype(Water)
                                               .evolve_to(WARTORTLE)
                                               .finish();
pub const WARTORTLE: Species = Species::build(2).name("Wartortle")
                                                .genus("Turtle Pokemon")
                                                .gen(Gen::I)
                                                .stats(Stats::new(59, 63, 80, 65, 80, 58))
                                                .ptype(Water)
                                                .evolve_to(BLASTOISE)
                                                .finish();
pub const BLASTOISE: Species = Species::build(3).name("Blastoise")
                                                .genus("Shellfish Pokemon")
                                                .gen(Gen::I)
                                                .stats(Stats::new(79, 83, 100, 85, 105, 78))
                                                .ptype(Water)
                                                .finish();
pub const MEGA_BLASTOISE: Species = Species::build(3).name("Mega Blastoise")
                                                     .genus("Shellfish Pokemon")
                                                     .gen(Gen::I)
                                                     .stats(Stats::new(79, 103, 120, 135, 115, 78))
                                                     .ptype(Water)
                                                     .form()
                                                     .finish();

pub const CHARIZARD: Species = Species::build(9).name("Charizard")
                                                .genus("Flame Pokemon")
                                                .gen(Gen::I)
                                                .stats(Stats::new(78, 84, 78, 109, 85, 100))
                                                .ptype(Dual(Fire, Flying))
                                                .finish();

pub const PIKACHU: Species = Species::build(25).name("Pikachu")
                                               .genus("Mouse Pokemon")
                                               .gen(Gen::I)
                                               .stats(Stats::new(35, 55, 40, 50, 50, 90))
                                               .ptype(Electric)
                                               .finish();

pub const ARCEUS: Species = Species::build(493).name("Arceus")
                                               .genus("Alpha Pokemon")
                                               .gen(Gen::IV)
                                               .stats(Stats::new(120, 120, 120, 120, 120, 120))
                                               .ptype(Normal)
                                               .class(PokeClass::Legendary)
                                               .finish();
