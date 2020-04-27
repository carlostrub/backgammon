// Some clarifications on how we play the game:
//
// Players: 1) Computer, 2) Opponent
// Board:   is numerated from 1-24 beginning on the start point of the Computer. Opponent facing,
//          this is switched and visible from the API's perspective. The bar will be used on
//          position 25, if necessary.
//
//
#![warn(future_incompatible)]
#![deny(
    missing_docs,
    unused_variables,
    missing_debug_implementations,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unsafe_code,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
    variant_size_differences
)] // be tough on code quality

mod bg_match;
mod cube;
mod dice;
mod game;
mod player;
mod state;

pub use bg_match::New;
pub use cube::Rules;
pub use player::Player;
