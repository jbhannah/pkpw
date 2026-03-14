# pkpw — Copilot Instructions

## Project Overview

`pkpw` is a password generator that uses Pokémon names as words, inspired by
[xkcd 936](https://xkcd.com/936/). It is written in Rust and distributed as:

- A **CLI binary** (`pkpw`)
- A **Rust library** (crates.io)
- A **WebAssembly/npm package** (compiled via `wasm-pack`)
- A **web app** at <https://pkpw.jbhannah.net> (Preact + TypeScript frontend in
  the `web/` directory)

## Repository Structure

```
src/           Rust library source
  lib.rs       Core password generation logic
  wasm.rs      wasm-bindgen bindings
  pokemon/     Pokémon name list and related logic
  bin/         CLI entry point
web/           Preact/TypeScript web frontend
  src/         Frontend source (components, utilities)
Formula/       Homebrew formula
.github/       CI workflows and Copilot setup
```

## Tech Stack

| Layer | Technology |
|---|---|
| Core library | Rust (stable), `rand` |
| CLI | `clap` |
| WASM build | `wasm-pack`, `wasm-bindgen` |
| Frontend | Preact, TypeScript, Tailwind CSS v4, DaisyUI, Rsbuild |
| JS package manager | pnpm (inside `web/`) |
| JS linter/formatter | Biome |
| Rust linter | Clippy |
| Analytics | PostHog |

## Building and Testing

### Rust

```sh
# Lint
cargo clippy --verbose

# Build and test (native)
cargo build --verbose --release
cargo test --verbose --release

# Build and test (WASM)
wasm-pack test --node
```

### Web frontend (`web/` directory)

```sh
pnpm install
pnpm build      # production build
pnpm dev        # dev server
pnpm check      # lint and format with Biome (auto-fix)
pnpm format     # format only
```

## Code Conventions

### Rust

- Use `rustfmt` for formatting; `clippy` for linting (all warnings must pass).
- Tests live in `#[cfg(test)]` modules inside the same file under test.
- Doc-comments use `///` and every public item should have one.
- WASM-only code is gated with `#[cfg(target_arch = "wasm32")]`.
- Non-WASM (CLI/native) code is gated with
  `#[cfg(not(target_arch = "wasm32"))]`.

### TypeScript / Preact

- All formatting and linting is enforced by **Biome** (`pnpm check`).
- Use **Preact** (not React); import from `preact` and `@preact/signals`.
- Components are written as arrow-function components and live in
  `web/src/components/`.
- Use Tailwind CSS utility classes for styling; DaisyUI component classes are
  available.
- No semicolons; double quotes for strings (enforced by Biome).

## CI

The CI workflow (`.github/workflows/ci.yml`) runs:

1. `cargo clippy` — must pass with no errors.
2. `wasm-pack test --node` — WASM tests must pass.
3. `cargo build --release && cargo test --release` — must pass on macOS,
   Ubuntu (x86-64 & ARM), and Windows (x86-64 & ARM).

All pull requests targeting `trunk` must pass CI before merging.
