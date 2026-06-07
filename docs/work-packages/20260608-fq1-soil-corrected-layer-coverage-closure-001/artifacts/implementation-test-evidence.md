# Implementation And Test Evidence

Evidence mode: `Static:` and `Ran:`.

Static: `map_corrected_layer_runtime_symbols_to_parser_layers` now extends only
the deepest normalized corrected interval to the parser profile bottom when:

- it is processing the last normalized corrected interval,
- the parser profile bottom is finite, and
- parser bottom is deeper than the normalized interval bottom.

The mapping still advances `normalized_top_mm` by the normalized grid thickness,
so hydrology seed-grid normalization is not changed. Invalid parser layers still
fail through the existing coverage guard.

Ran:

```text
cargo build -p openwepp-runner --bin openwepp-cli-hill
cargo test -p openwepp-hillslope-orchestrator fq1_ --lib
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo deny check
```

All commands returned `0`. `cargo deny check` emitted existing duplicate/unmatched
license warnings and ended with `advisories ok, bans ok, licenses ok, sources ok`.
