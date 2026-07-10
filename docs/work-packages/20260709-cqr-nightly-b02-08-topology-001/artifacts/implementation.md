# Implementation

Static:

- `TopologyParseError::fmt` now delegates exact string construction to a private
  helper, preserving every public display string covered by characterization
  tests.
- `parse_topology_fixture_str` now owns only the line loop and delegates
  fixture state to `TopologyFixtureBuilder`.
- Node parsing, header parsing, marker validation, contributor parsing, and
  final required-header assembly are private helpers.
- `validate_pre_execution_topology` now preserves the original violation order
  while delegating count, node, reference-domain, reference-existence, and cycle
  collection to private helpers.
- Channel/impoundment declared-count conversion is centralized in
  the extracted count helpers while preserving the prior fail-closed overflow
  branch for unsupported targets where a declared `u32` count cannot fit
  `usize`.
- Removed the two previous `#[allow(clippy::too_many_lines)]` attributes because
  the target functions are no longer long-function exceptions.

Protected behavior:

- no public topology API changes;
- no fixture grammar changes;
- no message ID changes;
- no typed error variant changes;
- no graph identity, edge identity, validation policy, or fail-closed status
  changes.

Ran:

- `cargo nextest run --test topology_graph_validation_gate --profile quick`;
  13/13 passed after implementation.
- `cargo fmt --check`; passed after applying rustfmt.
- `cargo clippy --test topology_graph_validation_gate -- -D warnings`; passed.
- `cargo clippy -p openwepp-topology --all-targets -- -D warnings`; passed.
- `CARGO_TARGET_DIR=/tmp/openwepp-cqr-b02-t08-cov4 cargo llvm-cov --workspace --test topology_graph_validation_gate --lcov --output-path /tmp/openwepp-cqr-b02-t08-final4.lcov`; 13/13 passed.
