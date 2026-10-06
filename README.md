## Hidden Line Elemination

https://bjrke.gitlab.io/hidden-line

## How to install

install rustup, you probably have to relogin
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
# Runs tests in Firefox
npm test -- --firefox

# Runs tests in Chrome
npm test -- --chrome

# Runs tests in Safari
npm test -- --safari
```
