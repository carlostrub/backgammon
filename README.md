[![](https://img.shields.io/crates/v/backgammon.svg)](https://crates.io/crates/backgammon) [![](https://docs.rs/backgammon/badge.svg)](https://docs.rs/backgammon) [![](https://img.shields.io/badge/License-BSD-brightgreen)](https://github.com/carlostrub/backgammon/blob/master/COPYRIGHT)
# Backgammon: The Oldest Board Game of the World
This crate provides a pure, canonical implementation of the game
[*Backgammon.*](https://en.wikipedia.org/wiki/Backgammon)

## Supported Doubling Cube Rules
The following [`rules`](`crate::rules::Rules`) on the doubling cube are supported:

* Beaver
* Raccoon
* Murphy
* Jacoby
* Crawford
* Holland

## Examples
Start a new backgammon match over default amount of points and with the default rules, as
defined in [`Rules`](`crate::rules::Rules`):
```
use backgammon::r#match::Match;

let mut m = Match::new();

```
Typically, you want to define the points for a match, hence:
```
use backgammon::r#match::Match;
use backgammon::rules::SetRules;

let mut m = Match::new().
with_points(13);

```
Depending on the style of tournament you decide to play, it makes sense to select one or more
rules too:
```
use backgammon::r#match::Match;
use backgammon::rules::SetRules;

let mut m = Match::new().
with_points(13).
with_jacoby();

```

## Discussions and Support
Remember that the APIs are not stable yet. Any support is very welcome. Please open an
[Issue](https://github.com/carlostrub/backgammon/issues) to discuss features or ask for help.
