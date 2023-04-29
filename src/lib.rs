//! # Backgammon: The Oldest Board Game of the World
//! This crate provides a pure, canonical implementation of the game
//! [*Backgammon.*](https://en.wikipedia.org/wiki/Backgammon)
//!
//! ## Supported Doubling Cube Rules
//! The following [`rules`](`crate::rules::Rules`) on the doubling cube are supported:
//!
//! * Beaver
//! * Raccoon
//! * Murphy
//! * Jacoby
//! * Crawford
//! * Holland
//!
//! ## Examples
//! Start a new backgammon match over default amount of points and with the default rules, as
//! defined in [`Rules`](`crate::rules::Rules`):
//! ```
//! use backgammon::r#match::Match;
//!
//! let mut m = Match::new();
//!
//! ```
//! Typically, you want to define the points for a match, hence:
//! ```
//! use backgammon::r#match::Match;
//! use backgammon::rules::SetRules;
//!
//! let mut m = Match::new().
//! with_points(13);
//!
//! ```
//! Depending on the style of tournament you decide to play, it makes sense to select one or more
//! rules too:
//! ```
//! use backgammon::r#match::Match;
//! use backgammon::rules::SetRules;
//!
//! let mut m = Match::new().
//! with_points(13).
//! with_jacoby();
//!
//! ```
//!
//! ## Discussions and Support
//! Remember that the APIs are not stable yet. Any support is very welcome. Please open an
//! [Issue](https://github.com/carlostrub/backgammon/issues) to discuss features or ask for help.

#![warn(future_incompatible)]
#![deny(
    rustdoc::broken_intra_doc_links,
    rustdoc::invalid_codeblock_attributes,
    rustdoc::invalid_html_tags,
    rustdoc::missing_crate_level_docs,
    missing_debug_implementations,
    missing_docs,
    rustdoc::private_intra_doc_links,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unsafe_code,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
    unused_variables,
    variant_size_differences
)] // be tough on code quality

/// Implements all Backgammon rules
mod error;
/// Implements a Backgammon game
pub mod game;
/// Implements a Backgammon match
pub mod r#match;
/// Implements all Backgammon rules
pub mod rules;

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
