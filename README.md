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

## Example
Start a new match with rules:
```
use backgammon::{Match,Rules};

let mut m = Match::new().
with_points(13).
with_jacoby();

```

## Discussions and Support
Any support is very welcome. Please use [Bitbucket
Issues](https://bitbucket.org/carlostrub/backgammon/issues?status=new&status=open) to discuss
features or ask for help.

## Source Code Integrity
All commits are signed with the following GPG key (find the respective key for example in the
    [FreeBSD keyring](https://docs.freebsd.org/pgpkeys/pgpkeys.txt)):

`59A6 2B5D B2FE B9CA 2358  4FA1 1C7A 2F39 D966 052B`

You can verify the integrity of the code by running:

`git log --show-signature`
