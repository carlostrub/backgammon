[![](https://img.shields.io/crates/v/backgammon.svg)](https://crates.io/crates/backgammon) [![](https://badgen.net/codecov/c/bitbucket/carlostrub/backgammon)]() [![](https://docs.rs/backgammon/badge.svg)](https://docs.rs/backgammon) [![](https://img.shields.io/badge/License-BSD-brightgreen)](https://bitbucket.org/carlostrub/backgammon/src/develop/COPYRIGHT)
# Backgammon: The Oldest Board Game of the World
This crate provides a pure, canonical implementation of the game
[*Backgammon*](https://en.wikipedia.org/wiki/Backgammon). It allows to
implement fast Backgammon games in various clients.

## Supported Doubling Cube Rules
This library supports the following rules on the doubling cube:

* Beaver
* Raccoon
* Murphy
* Jacoby
* Crawford
* Holland

## Examples
Start a new match with no extra rules:
```
use backgammon::{Match,Rules};

let mut m = Match::new().
with_points(3);

```
Start a new match with specific rules:
```
use backgammon::{Match,Rules};

let mut m = Match::new().
with_points(13).
with_jacoby();

```

## Discussions and Support
Remember that the APIs are not stable yet. Any support is very welcome. Please use [Bitbucket
Issues](https://bitbucket.org/carlostrub/backgammon/issues?status=new&status=open) to discuss
features or ask for help.
