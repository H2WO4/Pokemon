use crate::pokemon::species::{Gen, PokeClass};
use crate::pokemon::Single::*;
use crate::pokemon::{Dual, Species, Stats};


pub const SQUIRTLE: Species = Species::build(1).name("Squirtle")
                                               .genus("Tiny Turtle Pokemon")
                                               .gen(Gen::I)
                                               .stats(Stats::new(44, 48, 65, 50, 64, 43))
                                               .ptype(Water)
                                               .finish();
pub const WARTORTLE: Species = Species::build(2).name("Wartortle")
                                                .genus("Turtle Pokemon")
                                                .gen(Gen::I)
                                                .stats(Stats::new(59, 63, 80, 65, 80, 58))
                                                .ptype(Water)
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

pub const CHARMANDER: Species = Species::build(4).name("Charmander")
                                                 .genus("Lizard Pokemon")
                                                 .gen(Gen::I)
                                                 .stats(Stats::new(39, 52, 43, 60, 50, 65))
                                                 .ptype(Fire)
                                                 .finish();
pub const CHARMELEON: Species = Species::build(5).name("Charmeleon")
                                                 .genus("Flame Pokemon")
                                                 .gen(Gen::I)
                                                 .stats(Stats::new(58, 64, 58, 80, 65, 80))
                                                 .ptype(Fire)
                                                 .finish();
pub const CHARIZARD: Species = Species::build(6).name("Charizard")
                                                .genus("Flame Pokemon")
                                                .gen(Gen::I)
                                                .stats(Stats::new(78, 84, 78, 109, 85, 100))
                                                .ptype(Dual(Fire, Flying))
                                                .finish();
pub const MEGA_CHARIZARD_X: Species = Species::build(6).name("Mega Charizard X")
                                                       .genus("Flame Pokemon")
                                                       .gen(Gen::I)
                                                       .stats(Stats::new(78, 130, 111, 130, 85, 100))
                                                       .ptype(Dual(Fire, Dragon))
                                                       .form()
                                                       .finish();
pub const MEGA_CHARIZARD_Y: Species = Species::build(6).name("Mega Charizard Y")
                                                       .genus("Flame Pokemon")
                                                       .gen(Gen::I)
                                                       .stats(Stats::new(78, 104, 78, 159, 115, 100))
                                                       .ptype(Dual(Fire, Flying))
                                                       .form()
                                                       .finish();

pub const BULBASAUR: Species = Species::build(7).name("Bulbasaur")
                                                .genus("Seed Pokemon")
                                                .gen(Gen::I)
                                                .stats(Stats::new(45, 49, 49, 65, 65, 45))
                                                .ptype(Dual(Grass, Poison))
                                                .finish();
pub const IVYSAUR: Species = Species::build(8).name("Ivysaur")
                                              .genus("Seed Pokemon")
                                              .gen(Gen::I)
                                              .stats(Stats::new(60, 62, 63, 80, 80, 60))
                                              .ptype(Dual(Grass, Poison))
                                              .finish();
pub const VENUSAUR: Species = Species::build(9).name("Venusaur")
                                               .genus("Seed Pokemon")
                                               .gen(Gen::I)
                                               .stats(Stats::new(80, 82, 83, 100, 100, 80))
                                               .ptype(Dual(Grass, Poison))
                                               .finish();
pub const MEGA_VENUSAUR: Species = Species::build(9).name("Mega Venusaur")
                                                    .genus("Seed Pokemon")
                                                    .gen(Gen::I)
                                                    .stats(Stats::new(80, 100, 123, 122, 120, 80))
                                                    .ptype(Dual(Grass, Poison))
                                                    .form()
                                                    .finish();

pub const PICHU: Species = Species::build(172).name("Pichu")
                                              .genus("Tiny Mouse Pokemon")
                                              .gen(Gen::II)
                                              .stats(Stats::new(20, 40, 15, 35, 35, 60))
                                              .ptype(Electric)
                                              .finish();
pub const PIKACHU: Species = Species::build(25).name("Pikachu")
                                               .genus("Mouse Pokemon")
                                               .gen(Gen::I)
                                               .stats(Stats::new(35, 55, 40, 50, 50, 90))
                                               .ptype(Electric)
                                               .finish();
pub const RAICHU: Species = Species::build(26).name("Raichu")
                                              .genus("Mouse Pokemon")
                                              .gen(Gen::I)
                                              .stats(Stats::new(60, 90, 55, 90, 80, 110))
                                              .ptype(Electric)
                                              .finish();

pub const ARCEUS: Species = Species::build(493).name("Arceus")
                                               .genus("Alpha Pokemon")
                                               .gen(Gen::IV)
                                               .stats(Stats::new(120, 120, 120, 120, 120, 120))
                                               .ptype(Normal)
                                               .class(PokeClass::Legendary)
                                               .finish();
