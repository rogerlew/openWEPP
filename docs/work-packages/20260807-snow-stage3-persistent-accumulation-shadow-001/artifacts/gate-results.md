# Gate Results

Status: complete

Evidence mode: Ran

Focused persistent/orchestrator tests (5), runner Stage 3 tests (11), focused
consumer (1), and persistent contract tests (3) pass. Cargo check, formatting,
targeted Clippy with warnings denied, and diff hygiene pass.

Exact implementation-byte campaign results:

- `cargo nextest run --workspace --profile quick`: 2249/2249 passed, 40
  skipped, 57 slow, 2271.21 seconds.
- `cargo nextest run --workspace --profile frost`: 361/361 passed, 1982
  skipped, one slow, 537.14 seconds.
- `cargo nextest run --workspace`: 2338/2338 passed, five skipped, 58 slow,
  2665.64 seconds.

Assurance validation passes at generation
`87b4980692cced1c915dd4c4b99b1737a768fc1912ddf11aaee27de7f22a0782`
with DRAFT lifecycle and zero public reports. Canonical Markdown lint, cargo
deny (one non-blocking unmatched MIT-0 allowance warning), and terminal
verification pass.
