//! # Backgammon: The Oldest Board Game in the World
//! This crate provides a pure, canonical implementation of the game
//! [*Backgammon.*](https://en.wikipedia.org/wiki/Backgammon)
//!
//! <img src="https://upload.wikimedia.org/wikipedia/commons/3/30/Backgammon_lg.png" height="100">
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
//! Start a new backgammon match over the default amount of points and with the default rules, as
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
//! ## Design Philosophy
//! This library is designed to offer completely stateless game functions. This means that it
//! should be easy to implement wrappers using this library in combination with some databases to
//! store the game state.
//!
//! ## Discussions and Support
//! Remember that the APIs are not stable yet. Any support is very welcome. Please open an
//! [Issue](https://github.com/carlostrub/backgammon/issues) to discuss features or ask for help.
//!
//! You can also find me on:
//!
//! IRC: [#backgammon @ libera.chat](ircs://irc.libera.chat/#backgammon) (via
//! [webchat](https://web.libera.chat/#backgammon))

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

/// Implements all possible Backgammon errors
pub(crate) mod error;
pub use error::Error;
/// Implements a Backgammon game
pub mod game;
/// Implements a Backgammon match
pub mod r#match;
/// Implements the board, the dices, the cube, and all other Backgammon rules
pub mod rules;
