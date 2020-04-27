// Some clarifications on how we play the game:
//
// Players: 1) Computer, 2) Opponent
// Board:   is numerated from 1-24 beginning on the start point of the Computer. Opponent facing,
//          this is switched and visible from the API's perspective. The bar will be used on
//          position 25, if necessary.
//
//
#![warn(future_incompatible)]
#![deny(missing_docs)] // refuse to compile if documentation is missing

extern crate uuid;

mod bg_match;
mod cube;
mod dice;
mod game;
mod player;
mod state;

pub use bg_match::New;
pub use cube::Rules;
pub use player::Player;
