# lc

## Overview

[![Actions Status](https://github.com/k-nasa/lc/workflows/CI/badge.svg)](https://github.com/k-nasa/lc/actions)
[![crate-name at crates.io](https://img.shields.io/crates/v/lc.svg)](https://crates.io/crates/lc)

Markdown link checker

## Installation

### Pre-compiled executables

Get them [here](https://github.com/k-nasa/lc/releases)

### using homebrew

```
brew install k-nasa/tap/lc
```

### using cargo

Currently it cannot be built with the stable version.

```
cargo install lc
```
## Example

```console
lc README.md src/

=== Verify "README.md" ===
Ok https://github.com/k-nasa/lc/workflows/CI/badge.svg
Ok https://github.com/k-nasa/lc/actions
Ok https://img.shields.io/crates/v/lc.svg
Err  https://crates.io/crates/lc -> status code 403 Forbidden
Ok https://github.com/k-nasa/lc/releases
Ok http://github.com/k-nasa/lc
Ok https://github.com/k-nasa/lc/blob/master/LICENSE
Ok https://github.com/k-nasa
Ok https://k-nasa.me

=== Verify "src/main.rs" ===
Ok https://github.com/k-nasa/lc/workflows/CI/badge.svg
Ok https://github.com/k-nasa/lc/actions
Ok https://img.shields.io/crates/v/lc.svg
Err  https://crates.io/crates/lc -> status code 403 Forbidden
Ok https://github.com/k-nasa/lc/workflows/CI/badge.svg
Ok https://github.com/k-nasa/lc/actions
Ok https://img.shields.io/crates/v/lc.svg
Err  https://crates.io/crates/lc -> status code 403 Forbidden
```

## Contribution

1. Fork it ( http://github.com/k-nasa/lc)
2. Create your feature branch (git checkout -b my-new-feature)
3. Commit your changes (git commit -am 'Add some feature')
4. Push to the branch (git push origin my-new-feature)
5. Create new Pull Request

## Licence

[MIT](https://github.com/k-nasa/lc/blob/master/LICENSE)

## Author

[k-nasa](https://github.com/k-nasa)

[my website](https://k-nasa.me)
