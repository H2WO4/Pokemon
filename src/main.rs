#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::enum_glob_use)]
#![feature(const_option)]
#![feature(const_option_ext)]
#![feature(const_trait_impl)]

mod pokemon;
mod types;

use pokemon::{species, Pokemon};


fn main() {
    let pika: Pokemon = Pokemon::build(species::PIKACHU).level(5)
                                                        .shiny()
                                                        .name("Pika")
                                                        .finish();
    println!("Pikachu's name is {}", pika.get_name());
}
