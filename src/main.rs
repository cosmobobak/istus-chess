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
mod movebuffer;
mod movegen;

fn main() {
    println!("Hi! I am Istus version 1");
    
}
