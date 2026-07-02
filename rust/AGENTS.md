## Learned User Preferences

- User is learning Rust; when helping with HackerRank problems, explain language concepts and guide without giving away full solutions.
- Prefers a flat two-crate workspace (`algorithms`, `data_structures`) rather than nested member crates per category.
- Categorize problems by prepending the category to the filename (e.g. `warm_up_staircase.rs`, `arrays_2d_array.rs`) inside each crate's `src/bin/`.
- Do not create placeholder `.rs` files; empty placeholder folders for future categories are acceptable.
- Do not edit plan files when implementing a plan.

## Learned Workspace Facts

- Cargo workspace root is `rust/` with members `algorithms` and `data_structures` (virtual workspace, no root package).
- Package names are `hr-algorithms` and `hr-data-structures`; edition 2024.
- Each crate has one `Cargo.toml` and one `src/bin/` folder; all problem files live there as auto-discovered binaries.
- Binary names match the file stem (e.g. `warm_up_staircase`, `strings_reduce`).
- Run a problem with `cargo run -p hr-algorithms --bin <name>` or `cargo run -p hr-data-structures --bin <name>`.
- External crate dependencies belong in the package `Cargo.toml` that uses them, not the workspace root.
