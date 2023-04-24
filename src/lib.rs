//! # Backgammon: The Oldest Board Game of the World
//! This crate provides a pure, canonical implementation of the game
//! [*Backgammon*](https://en.wikipedia.org/wiki/Backgammon). It allows to
//! implement fast Backgammon games in various clients.
//!
//! ## Supported Doubling Cube Rules
//! This library supports the following rules on the doubling cube:
//!
//! * Beaver
//! * Raccoon
//! * Murphy
//! * Jacoby
//! * Crawford
//! * Holland
//!
//! ## Examples
//! Start a new backgammon match over default (3) points and with no extra rules:
//! ```
//! use backgammon::Match;
//!
//! let mut m = Match::new()
//!
//! ```
//! Typically, you want to define the points for a match, hence:
//! ```
//! use backgammon::Match;
//!
//! let mut m = Match::new().
//! set_points(3);
//!
//! ```
//! Depending on the style of tournament you decide to play, it makes sense to select one or more
//! rules too:
//! ```
//! use backgammon::Match;
//!
//! let mut m = Match::new().
//! set_points(13).
//! set_jacoby();
//!
//! ```
//!
//! ## Discussions and Support
//! Remember that the APIs are not stable yet. Any support is very welcome. Please use [Bitbucket
//! Issues](https://bitbucket.org/carlostrub/backgammon/issues?status=new&status=open) to discuss
//! features or ask for help.

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

/// Implements all Backgammon rules
mod bg_error;
/// Implements a Backgammon game
pub mod bg_game;
/// Implements a Backgammon match
pub mod bg_match;
/// Implements all Backgammon rules
pub mod bg_rules;

//#[cfg(test)]
//mod tests {
//
//    #[test]
//    use crate::bg_match::Match;
//    fn debug_match() {
//        let m = Match::new().set_jacoby();
//
//        assert_eq!(
//            format!("The match rules are: {:?}", m.rules),
//            "The match rules are: Rules { beaver: false, raccoon: false, murphy: false, murphy_limit: 0, jacoby: true, crawford: true, holland: false }"
//        );
//    }
//
//    #[test]
//    fn debug_current_rules() {
//        let r = Box::new(Rules)::default().set_jacoby();
//
//        assert_eq!(
//            format!("The match rules are: {:?}", r),
//            "The match rules are: Rules { beaver: false, raccoon: false, murphy: false, murphy_limit: 0, jacoby: true, crawford: true, holland: false }"
//        );
//    }
//
//    #[test]
//    fn debug_cubeowner() {
//        let o = Player::Nobody;
//
//        assert_eq!(
//            format!("The cube is owned by: {:?}", o),
//            "The cube is owned by: Nobody"
//        );
//    }
//    #[test]
//    fn debug_game() {
//        let g = Game::default();
//        let g_beginning = format!("{:?}", g);
//
//        assert_eq!(
//            g_beginning.get(0..16).unwrap(),
//            String::from("Game { points: 0")
//        );
//    }
//}
