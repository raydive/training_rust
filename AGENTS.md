# AGENTS GUIDE

## Scope
- This file applies to the entire repository at `/Users/arata_n/Documents/dev/training_rust`.
- Use these rules for every crate; no nested AGENTS exist.
- No Cursor rules found (no `.cursor/rules` or `.cursorrules`).
- No Copilot rules found (no `.github/copilot-instructions.md`).
- These guidelines target agentic coding assistants; keep instructions concise and actionable.

## Repository map
- Each directory with `Cargo.toml` is its own crate; no workspace manifest.
- Crates: `guessing_game`, `functions`, `branches`, `chapter3/temperature`, `chapter12/minigrep`, `chapter13/iterator`, `chapter13/startup`, `chapter15/cons`.
- Build/run/test commands must be executed inside the crate directory unless noted.

## Toolchain
- Rust edition 2021 for all crates.
- Requires `cargo`, `rustc`, `rustfmt`, and optionally `clippy`.
- Prefer `rustup default stable` unless project constraints say otherwise.

## Build commands (per crate)
- Build debug: `cargo build`.
- Build release: `cargo build --release`.
- Clean: `cargo clean` (use sparingly; prefer incremental builds).

## Run commands (per crate)
- Standard run: `cargo run`.
- Run with args (e.g., `minigrep`): `cargo run -- <query> <filename>`.
- Env var example (`minigrep` case-insensitive search): `CASE_INSENSITIVE=1 cargo run -- <query> <filename>`.

## Test commands (per crate)
- Full test suite: `cargo test`.
- Single test by name (substring filter): `cargo test <test_name_fragment>`.
- Example: `cargo test iterator_sum` in `chapter13/iterator`.
- Run tests without output capture when debugging: `cargo test -- --nocapture`.
- Doc tests (if added later): `cargo test --doc`.

## Linting
- Clippy (if installed): `cargo clippy --all-targets --all-features -D warnings`.
- Fixable lints: `cargo clippy --fix --allow-dirty --allow-staged` (review edits before commit).
- Treat warnings as errors during lint runs.

## Formatting
- Format all crates: `cargo fmt` from the crate root (or repo root to format all detected crates).
- Do not hand-format; rely on `rustfmt` defaults unless crate-specific config is introduced.

## Dependency management
- Add deps with `cargo add <crate>` inside each crate (if `cargo-edit` is available) or edit `Cargo.toml` then `cargo update`.
- Keep versions minimal; prefer caret ranges from examples.

## Imports and module hygiene
- Order imports: standard library, external crates, then local modules; separate groups with a blank line.
- Use specific paths over glob imports; avoid `use super::*` except in small test modules.
- Re-export thoughtfully; prefer explicit `pub use` when creating crate APIs.
- Remove unused imports; Clippy will flag them.

## Naming conventions
- Types/traits/enums: `CamelCase` (e.g., `Config`, `Cacher`).
- Functions/methods/variables: `snake_case` (e.g., `generate_workout`, `case_sensitive`).
- Constants/statics: `SCREAMING_SNAKE_CASE`.
- Modules and files: `snake_case`.
- Avoid one-letter identifiers except for well-known iterators (`i`, `j`, `k`, `x`, `y`) in small scopes.

## Types and generics
- Prefer concrete types when possible; use generics with clear bounds (see `Cacher<T> where T: Fn(u32) -> u32`).
- Avoid needless `clone`; prefer borrowing and references.
- Use `&str` for read-only string data; `String` when owned/modified.
- Use `usize` for indexing; `u32`/`i32` for small integers, matching book examples.

## Error handling
- Return `Result` from fallible public functions; bubble errors with `?` where possible.
- Reserve `expect`/`unwrap` for tests, examples, or when failing is a valid crash strategy (message required).
- For CLI entry points, print user-friendly errors to `stderr` and exit non-zero (see `minigrep::main`).
- Avoid panics in library code unless invariants are violated.
- Prefer `Box<dyn Error>` for simple CLI error propagation unless a richer error type is needed.

## Control flow and matching
- Use `match` or `if let` for enum handling; ensure exhaustive patterns on non-`Option`/`Result` enums.
- Keep arms simple; extract helpers when match arms grow.

## Collections and iterators
- Prefer iterator adapters over indexed loops when readable (see `for element in a.iter()` patterns).
- Collect into typed targets (`Vec<_>` okay when type is obvious; otherwise specify).
- Avoid unnecessary allocations; reuse buffers where practical for performance-sensitive code.

## Tests
- Place unit tests in `#[cfg(test)] mod tests` or `mod test` alongside code, following examples in `iterator` and `startup`.
- Name tests descriptively with `snake_case` (e.g., `case_insensitive`, `iterator_sum`).
- For single-test runs: `cargo test <name>`; combine with `-- --nocapture` to see prints.
- Keep tests deterministic; avoid sleeping or randomness unless seeded.

## Logging and output
- Use `println!` for standard output in examples; prefer `eprintln!` for errors.
- Keep output user-facing and concise for CLI crates.
- Avoid debug prints in library code; gate debugging with cfgs if needed.

## Comments and documentation
- Keep comments meaningful; avoid redundant translations unless required.
- Use doc comments `///` for public APIs if added; include examples that compile.
- Maintain bilingual notes only where already present; otherwise default to concise English.

## CLI behavior (`minigrep` reference)
- Argument parsing currently manual via `env::args`; validate length and report friendly errors.
- Honor environment toggles like `CASE_INSENSITIVE`; document any new env vars added.
- Exit with status 1 on recoverable CLI errors.

## Ownership, borrowing, and lifetimes
- Prefer borrowing over cloning; make ownership explicit in function signatures.
- Use lifetime annotations only when required by references that escape scope.
- Avoid interior mutability unless necessary; prefer mutable locals or structs.

## Concurrency (future guidance)
- Prefer std primitives; use threads only with clear ownership and `Send + Sync` considerations.
- When sharing data, prefer `Arc`/`Mutex`/`RwLock` with minimal critical sections.

## Reference patterns from codebase
- CLI parsing pattern in `chapter12/minigrep/src/main.rs`: construct config, validate, return `Result`, exit on error.
- Iterator usage in `chapter13/iterator/src/lib.rs`: favor iterators and adapters, assert expected outputs.
- Caching pattern in `chapter13/startup/src/main.rs`: struct with generic closure and map to memoize results.
- Shared ownership in `chapter15/cons/src/main.rs`: prefer `Rc` cloning via `Rc::clone` over `Rc::new`.

## Testing data and fixtures
- Keep sample files small (e.g., `chapter12/minigrep/poem.txt`, `output.txt`).
- Inline strings are acceptable for simple tests; add fixture files only when clarity improves.

## Performance and allocation
- Avoid premature optimization; prioritize clarity in educational examples.
- When needed, minimize allocations by reusing buffers and iterators.

## Code review checklist
- Commands documented in this file are followed and runnable.
- Imports grouped and unused items removed.
- Formatting clean via `cargo fmt`; lint clean via `cargo clippy` when available.
- Tests updated/added and passing; single-test filter documented.
- Error messages clear and surfaced to stderr for CLI paths.

## When adding new crates
- Create a new directory with `Cargo.toml` and `src/main.rs` or `src/lib.rs`.
- Ensure edition is `2021`; keep dependency versions consistent with existing crates.
- Update this guide if adding repo-wide conventions or workspace manifests.

## Notes on files to avoid committing
- Do not commit `target/` artifacts; respect `.gitignore` defaults.
- Avoid adding large binaries or secrets; this is an educational repo.

## Quick command cheatsheet (run inside target crate)
- Build: `cargo build`
- Run: `cargo run`
- Run with args: `cargo run -- <args>`
- Test all: `cargo test`
- Test one: `cargo test <name>`
- Lint: `cargo clippy --all-targets --all-features -D warnings`
- Format: `cargo fmt`

## Updating this guide
- Keep length ~150 lines; update when new crates, tools, or style rules are introduced.
- Preserve clarity and directness for future agents.
