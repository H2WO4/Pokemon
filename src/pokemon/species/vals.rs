use super::{Generation, PokeClass, Species};
use crate::pokemon::Stats;
use crate::types::Dual;
use crate::types::Single::*;


pub const SQUIRTLE: Species = Species::build(1).name("Squirtle")
                                               .gen(Generation::I)
                                               .stats(Stats::new(44, 48, 65, 50, 64, 43))
                                               .ptype(Water)
                                               .evolve_to(WARTORTLE)
                                               .finish();
pub const WARTORTLE: Species = Species::build(2).name("Wartortle")
                                                .gen(Generation::I)
                                                .stats(Stats::new(59, 63, 80, 65, 80, 58))
                                                .ptype(Water)
                                                .evolve_to(BLASTOISE)
                                                .finish();
pub const BLASTOISE: Species = Species::build(3).name("Blastoise")
                                                .gen(Generation::I)
                                                .stats(Stats::new(79, 83, 100, 85, 105, 78))
                                                .ptype(Water)
                                                .finish();

pub const CHARIZARD: Species = Species::build(9).name("Charizard")
                                                .gen(Generation::I)
                                                .stats(Stats::new(78, 84, 78, 109, 85, 100))
                                                .ptype(Dual(Fire, Flying))
                                                .finish();

pub const PIKACHU: Species = Species::build(25).name("Pikachu")
                                               .gen(Generation::I)
                                               .stats(Stats::new(35, 55, 40, 50, 50, 90))
                                               .ptype(Electric)
                                               .finish();

pub const ARCEUS: Species = Species::build(493).name("Arceus")
                                               .gen(Generation::IV)
                                               .stats(Stats::new(120, 120, 120, 120, 120, 120))
                                               .ptype(Normal)
                                               .class(PokeClass::Legendary)
                                               .finish();
