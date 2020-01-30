<a href="https://github.com/open-flash/open-flash">
    <img src="https://raw.githubusercontent.com/open-flash/open-flash/master/logo.png"
    alt="Open Flash logo" title="Open Flash" align="right" width="64" height="64" />
</a>

# SWF Bitmap (Rust)

[![crates.io](https://img.shields.io/crates/v/swf-bitmap.svg)](https://crates.io/crates/swf-bitmap)
[![GitHub repository](https://img.shields.io/badge/Github-open--flash%2Fswf--bitmap-blue.svg)](https://github.com/open-flash/swf-bitmap)
[![Build status](https://img.shields.io/travis/com/open-flash/swf-bitmap/master.svg)](https://travis-ci.com/open-flash/swf-bitmap)

SWF bitmap library implemented in Rust and Typescript (Node and browser).
Decodes and encodes all bitmap types supported by SWF.

## Usage

```rust
// TODO
```

## Contributing

This repo uses Git submodules for its test samples:

```sh
# Clone with submodules
git clone --recurse-submodules git://github.com/open-flash/swf-bitmap.git
# Update submodules for an already-cloned repo
git submodule update --init --recursive --remote
```

This library is a standard Cargo project. You can test your changes with
`cargo test`.  **The commands must be run from the `rs` directory.**

Prefer non-`master` branches when sending a PR so your changes can be rebased if
needed. All the commits must be made on top of `master` (fast-forward merge).
CI must pass for changes to be accepted.

[swf-types]: https://github.com/open-flash/swf-types
