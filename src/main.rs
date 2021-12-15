#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
)]
#![allow(dead_code)]

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
mod movegen;

mod tests;

fn main() {
    
}
