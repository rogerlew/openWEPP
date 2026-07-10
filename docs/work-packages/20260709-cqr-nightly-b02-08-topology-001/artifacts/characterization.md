# Characterization Plan

Cover every topology parse error display, fixture grammar/reference failure,
pre-execution validation result, and cycle/reference closure through public APIs
before structural extraction.

Planned focused test path:

- `tests/integration/topology_graph_validation_gate.rs`

Initial gap from static read:

- existing tests cover the canonical valid path, several validation violations,
  and the missing-file `ReadError` variant;
- they do not yet cover all parser error variants, display strings, duplicate
  headers/nodes, marker failures, invalid numeric fields, or all reference
  existence/domain branches needed to safely decompose the high-CRAP parser,
  display implementation, and validation gate.

Implemented characterization:

- Added seven focused public tests in
  `tests/integration/topology_graph_validation_gate.rs`.
- Covered missing/header format/header value/duplicate header parser errors.
- Covered node record format, marker format, unknown kind, hillslope row
  rejection, node value parse, duplicate node, and exact display strings.
- Covered missing-file display prefix and `Error::source` behavior.
- Covered the topology validation error adapter display/source wrapper.
- Covered hillslope count, impoundment count, hillslope domain, channel
  zero-upper-bound, missing channel, and missing impoundment validation
  message IDs.

Test-first evidence:

| Evidence | Result |
|---|---|
| Focused local run before production edits | `cargo nextest run --test topology_graph_validation_gate --profile quick`; 12/12 passed for the parser/validator characterization set. |
| Final detached pre-refactor proof | Worktree `/tmp/openwepp-cqr-b02-t08-predecomp-final` at `87e15ffb469c27b74e47cc69e09e7ac26cff3523`; applied only the final `tests/integration/topology_graph_validation_gate.rs` diff; command `CARGO_TARGET_DIR=/tmp/openwepp-cqr-b02-t08-predecomp-final-target cargo nextest run --manifest-path /tmp/openwepp-cqr-b02-t08-predecomp-final/Cargo.toml --test topology_graph_validation_gate --profile quick`; 13/13 passed; worktree removed. |
| Review-response adapter coverage | Added `topology_validation_error_wraps_status_source` and `Error::source` assertions for `TopologyParseError` after source decomposition to close the observed adapter coverage floor; final focused runs below cover 13/13. |

Source/test hashes after characterization and refactor:

- `crates/openwepp-topology/src/lib.rs`:
  `0b5f8f94fbd20589c2cff561aa8ad80104aee87667e908216086a181b5ca1c8e`
- `tests/integration/topology_graph_validation_gate.rs`:
  `866cffd5c4a781ed5ab2d7b9f3c9b3ccd2b55971437b795e305c9a47dd131501`
