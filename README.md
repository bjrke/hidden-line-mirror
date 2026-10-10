## Hidden Line Elimination

[gitlab.com/bjrke/hidden-line](https://gitlab.com/bjrke/hidden-line) · [License](LICENSE)
[![pipeline status](https://gitlab.com/bjrke/hidden-line/badges/main/pipeline.svg)](https://gitlab.com/bjrke/hidden-line/-/pipelines)

## How to install

install rust via rustup, you probably have to relogin
https://rustup.rs/

install wasm-pack
```sh
cargo install wasm-pack
```

install npm
https://www.npmjs.com/get-npm

```sh
npm install
```

## How to run in development mode

```sh
# Builds the project and opens it in a new browser tab. Auto-reloads when the project changes.
# Uses an optimized build (no debug symbols) for smooth rendering of CPU intensive formulas.
npm start
```

```sh
# Same, but unoptimized and with debug symbols, for debugging.
npm run debug
```

## How to build in release mode

```sh
# Builds the project and places it into the `dist` folder.
npm run build
```

## How to run unit tests

```sh
# Runs the Rust tests and the node wasm tests.
npm test
```

```sh
# Runs tests in Firefox
npm test -- --firefox
```

```sh
# Runs tests in Chrome
npm test -- --chrome
```

```sh
# Runs tests in Safari
npm test -- --safari
```

## How to lint

```sh
cargo clippy --all-targets --all-features -- -D warnings
```
