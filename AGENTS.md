# AGENTS.md

Hidden Line Elimination: an interactive web app that renders implicit surfaces `z = f(x, y)` (formula typed into a textarea, evaluated via JS `eval`) as vectorized polygons, hiding hidden lines. Rust core compiled to WebAssembly, rendered as SVG. Deployed to GitLab Pages.

## Layout

- `hidden-line-wasm/` — the active app. Cargo workspace member (see root `Cargo.toml`) plus a webpack project.
- `hidden-line-pascal/` — legacy FreePascal reference implementation of the algorithm. Not in the Cargo workspace, not built by CI. Requires FreePascal; `./build.sh` compiles `dreidplot.pas`.
- Root `Cargo.toml` is a workspace whose only member is `hidden-line-wasm`; `cargo test --all` from root runs the wasm crate's tests.

## Commands (run from `hidden-line-wasm/`)

Requires `rustup`, `wasm-pack`, and `npm` installed (README install steps; `cargo`/`rustc`/`wasm-pack` are on PATH only after `source ~/.cargo/env`).

- `npm install` — install deps
- `npm start` — dev server (`webpack-dev-server --open`), auto-reloads on change
- `npm run build` — production build into `dist/` (removes `dist/` and `pkg/` first)
- `npm test` — runs `cargo test && wasm-pack test --node`; append `-- --firefox` / `-- --chrome` / `-- --safari` to run the wasm tests in a browser instead
- `cargo fmt --all` — format the workspace; run `cargo fmt --all -- --check` to verify (CI does the check)

CI (GitLab, `.gitlab-ci.yml`) mirrors this: `cargo fmt --all -- --check`, `cargo test --all`, then in `hidden-line-wasm/` `npm i --cache .npm --prefer-offline`, `npm run test`, `npm run build`. The `pages` job deploys `hidden-line-wasm/dist` to GitLab Pages on `main`. A manual job builds the CI Docker image from the repo `Dockerfile`.

## Architecture

- JS entry is `hidden-line-wasm/js/index.js` (webpack entry). It `eval`s the formula into an `(x, y) => z` function, calls `wasm.lets_go(svg)`, then `hiddenLine.set_function(f)`.
- Rust entry is `hidden-line-wasm/src/lib.rs`, using the old `wasm-bindgen` 0.2.60 API (not modern `wasm-bindgen`): `lets_go(svg) -> HiddenLine`, with `on_key` / `set_function` methods. It defines its own `print!`/`println!`/`eprint!`/`eprintln!` macros.
- Pipeline: `plot::init_scene` samples the surface into a triangle mesh (`Scene3` in `dreidext.rs`) → `AppContext` (keyboard state) → `CalcContext` (`calcctontext.rs`, the hidden-line elimination core using `QuadTree` + `RangeSet`) → `DrawContext` trait (`drawcontext.rs`) → `SvgContext` (`svgcontext.rs`) writes SVG paths.
- `float.rs` sets `pub type Float = f32`; all geometry math is single-precision.

## Workflow

Committing is a required part of every task — never finish without it.

1. Before making any changes: if not already on a branch, create one off `main` (e.g. `git checkout -b <short-description>`).
2. Make your changes, then run `cargo fmt --all` so the tree stays format-clean.
3. Verify with the test suite: `cargo test --all` from the repo root (and `wasm-pack test --node` in `hidden-line-wasm/` if you touched wasm-facing code).
4. Before telling the user you are done, commit: `git add` the intended files (check `git status` and `git diff` first), then commit with a headline at most 50 chars and a body wrapped at 72 chars.

Never leave the working tree dirty at the end of a task.

## Gotchas

- The math structs (`Vector2`, `Vector3`, `Matrix2/3`, `FloatRange`, `Line`, `Triangle`, `RangeSet`, `SvgPoint`) use named fields for readability, but are deliberately equipped with positional `new(x, y, ...)` constructors and `From`/`Into` tuple conversions so they can still be built and decomposed positionally (`Vector2::new(x, y)`, `let (x, y) = v.into();`). Keep the constructors and tuple `From` impls in sync with the fields.
- `pkg/` (wasm-bindgen output) and `dist/` (webpack output) are generated build artifacts, gitignored.
- The root `Cargo.lock` is committed so CI builds are deterministic; do not delete it. `hidden-line-wasm/Cargo.toml` sets `rust-version = "1.85"` and `.cargo/config.toml` sets `resolver.incompatible-rust-version-fallback = false`, so a `cargo update` refuses to pick deps that need a newer rustc than the pinned `rust:1.85-slim` image (`rust_wasm_npm`). Bump both the `Dockerfile` and `rust-version` together.
- `opencode.json` is gitignored (local-only config).
- Every text file must end with a single trailing newline — a missing final newline is an antipattern to avoid.
