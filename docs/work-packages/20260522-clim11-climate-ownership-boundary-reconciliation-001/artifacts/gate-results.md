# Gate Results

Status: `complete`
Evidence mode: `Ran`

Required gate commands (when code changes are in scope):
1. `cargo fmt --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test --workspace`
4. `cargo deny check`

## Scope Check
- CLIM11 write set is docs/ADR-only for ownership-contract reconciliation.
- No Rust source or test code was changed in CLIM11 scope.
- Verification command: `git status --short docs/decisions/README.md docs/decisions/0013-climate-forcing-ownership-boundary.md docs/work-packages/20260522-clim11-climate-ownership-boundary-reconciliation-001/artifacts`

## Results
1. `cargo fmt --check`
- result: `not-run`
- rationale: docs/ADR-only scope; code-change gates not in scope.

2. `cargo clippy --workspace --all-targets -- -D warnings`
- result: `not-run`
- rationale: docs/ADR-only scope; code-change gates not in scope.

3. `cargo test --workspace`
- result: `not-run`
- rationale: docs/ADR-only scope; code-change gates not in scope.

4. `cargo deny check`
- result: `not-run`
- rationale: docs/ADR-only scope; code-change gates not in scope.

## Outcome
Ownership-boundary closure evidence is documentation-contract based for CLIM11;
no runtime gate replay required by package scope.
