#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::enum_glob_use)]
#![allow(incomplete_features)]
#![feature(const_option)]
#![feature(const_option_ext)]
#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]
#![feature(const_mut_refs)]
#![feature(const_ptr_read)]
#![feature(const_slice_index)]
#![feature(type_changing_struct_update)]
#![feature(associated_type_bounds)]

mod pokemon;

// use pokemon::species::{self, Species};
// use pokemon::{PokeType, Pokemon, Stats};

#[macro_export]
macro_rules! all_is {
	($var:ident $($varr:ident)* <= $bound:literal) => {
		($var <= $bound) $(&& ($varr <= $bound))*
	};
	($var:ident $($varr:ident)* < $bound:literal) => {
		($var < $bound) $(&& ($varr < $bound))*
	};
	($var:ident $($varr:ident)* >= $bound:literal) => {
		($var >= $bound) $(&& ($varr >= $bound))*
	};
	($var:ident $($varr:ident)* > $bound:literal) => {
		($var > $bound) $(&& ($varr > $bound))*
	};
}
#[macro_export]
macro_rules! any_is {
	($var:ident $($varr:ident)* <= $bound:literal) => {
		($var <= $bound) $(|| ($varr <= $bound))*
	};
	($var:ident $($varr:ident)* < $bound:literal) => {
		($var < $bound) $(|| ($varr < $bound))*
	};
	($var:ident $($varr:ident)* >= $bound:literal) => {
		($var >= $bound) $(|| ($varr >= $bound))*
	};
	($var:ident $($varr:ident)* > $bound:literal) => {
		($var > $bound) $(|| ($varr > $bound))*
	};
}
#[macro_export]
macro_rules! sum_is {
	($var:ident $($varr:ident)* <= $bound:literal) => {
		(($var as u16) $(+ ($varr as u16))*) <= $bound
	};
	($var:ident $($varr:ident)* < $bound:literal) => {
		(($var as u16) $(+ ($varr as u16))*) < $bound
	};
	($var:ident $($varr:ident)* >= $bound:literal) => {
		(($var as u16) $(+ ($varr as u16))*) >= $bound
	};
	($var:ident $($varr:ident)* > $bound:literal) => {
		(($var as u16) $(+ ($varr as u16))*) > $bound
	};
}


fn main() {
    // let pika: Pokemon = Pokemon::build(&species::PIKACHU).level(5)
    //                                                      .shiny()
    //                                                      .nickname("Pika")
    //                                                      .finish();

    // println!("Pikachu's name is {}", pika.get_name());
}
