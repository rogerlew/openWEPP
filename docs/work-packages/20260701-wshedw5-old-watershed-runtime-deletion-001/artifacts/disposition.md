# Disposition

Status: `EXECUTED-COMPLETE`

Evidence mode: `static + ran`

WSHED-W5 is complete.

Completed scope:

- deleted watershed-specific old request/writeback runtime surfaces;
- deleted obsolete old-surface tests;
- added typed direct runtime replacement coverage;
- widened source guards against old runtime reintroduction;
- preserved actual direct WS11/WS12/WS18/WS20 physics and WS12 guard taxonomy;
- recorded deletion, retained out-of-scope surfaces, review disposition, and
  gate evidence.

Final gate summary:

- `cargo fmt --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo nextest run --workspace --profile full`: `1196` passed, `1` skipped.
- `cargo deny check`: passed.

No W5 blocker remains.
