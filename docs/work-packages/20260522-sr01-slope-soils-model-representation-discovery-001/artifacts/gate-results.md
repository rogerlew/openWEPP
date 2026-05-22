# Gate Results

Status: `complete`
Evidence mode: `Ran`

Static:
- None.

Ran:
- Executed docs-only gate checks for required-file completeness, placeholder-token sweep, truthfulness markers, and decision-value validity.

## Package Type

`docs-only`

## Results

1. Required artifacts:
- `REQUIRED_FILES_OK=1`
- `REQUIRED_FILE_COUNT=14`
- `ARTIFACT_MD_COUNT=15` (includes `artifacts/README.md`)

2. Placeholder-token sweep:
- First pass flagged unresolved tokens only because `gate-results.md` still contained stub content at that moment.
- Final pass after rewrite: `PLACEHOLDER_TOKEN_OK=1`.

3. Truthfulness markers:
- `TRUTHFULNESS_MARKER_HITS >= 1` for each authored artifact (`Static:` and/or `Ran:` present).

4. Boundary decision value:
- `DECISION_VALUE_OK=1`
- `DECISION_VALUE=BOUNDARY_EXTEND_SERIES_REQUIRED`

5. Code gates:
- Not applicable for this execution (`docs-only`; no code files changed):
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`
