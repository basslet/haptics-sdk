# builder context — haptics-sdk

Rust, no venv: this replaces the global Python mechanics.

- Full suite: `bash tests/run_all.sh`. A subset: `cargo test -p <crate> <test name>`.
- With `_tooling/bin` on `PATH`, a link step takes the session launcher `_tooling/bin/cc` for the C compiler (`_tooling` `DEBT-2026-11-04-cc-shadows-c-compiler`). `tests/run_all.sh` strips that directory; a direct cargo call needs the same `PATH`.
- A test on upstream's code goes in `core/<crate>/tests/requirements.rs`, with `// r[verify <id>]` directly above `#[test]`. Upstream's inline tests take only the marker.
- `just` is not installed and the suite does not use upstream's `justfile`.
