#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
)]
#![allow(dead_code)]

use crate::{magicnumbers::{BB_RANK_ATTACKS, sliding_attacks}};

#[macro_use]
extern crate lazy_static;

mod magicnumbers;
mod bitmethods;
mod bitboards;
mod squares;
mod board;
mod cmove;
mod colour;
mod piece;
mod movebuffer;
mod movegen;

fn main() {
    println!("Hi! I am Istus version 1");
    println!("{:?}", BB_RANK_ATTACKS[0]);
    println!("{}", sliding_attacks(0, 0, &[-9, -7, 7, 9]));
}
