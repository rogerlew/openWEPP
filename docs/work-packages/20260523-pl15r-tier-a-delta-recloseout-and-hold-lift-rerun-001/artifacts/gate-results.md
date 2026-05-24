# gate-results

Status: `complete`
Evidence mode: `Static + Ran`

## Pre-Implementation Contract Gate

```bash
cargo test --test pl15r_tier_a_delta_recloseout_contract -- --nocapture
```

Result: `ok` (`5 passed`, `0 failed`).

## Required Rust Validation Gates

1. `cargo fmt --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test --workspace`
4. `cargo deny check`

Results:
- `cargo fmt --check`: `ok` (after one `cargo fmt` pass on the new PL15R test file).
- `cargo clippy --workspace --all-targets -- -D warnings`: `ok`.
- `cargo test --workspace`: `ok`.
- `cargo deny check`: `ok` (`advisories ok, bans ok, licenses ok, sources ok`) with non-fatal `license-not-encountered` warnings for unmatched allowlist entries.

## Artifact Hygiene Gates

1. Placeholder sweep:

```bash
rg -n '^Reserved for PL15R execution\.' docs/work-packages/20260523-pl15r-tier-a-delta-recloseout-and-hold-lift-rerun-001/artifacts
```

Result: `pass` (no matches).

2. Queued-status sweep:

```bash
rg -n '^Status: .*queued.*$' docs/work-packages/20260523-pl15r-tier-a-delta-recloseout-and-hold-lift-rerun-001/package.md docs/work-packages/20260523-pl15r-tier-a-delta-recloseout-and-hold-lift-rerun-001/artifacts/*.md
```

Result: `pass` (no matches).
