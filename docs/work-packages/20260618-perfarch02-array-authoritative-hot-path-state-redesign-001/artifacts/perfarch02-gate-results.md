# PERFARCH02 Gate Results

Evidence class: Ran locally on 2026-06-18.

## Prototype Rust Gates

| Gate | Result |
|---|---|
| `cargo fmt --manifest-path docs/work-packages/20260618-perfarch02-array-authoritative-hot-path-state-redesign-001/artifacts/perfarch02-floor-prototype/Cargo.toml --check` | PASS |
| `cargo check --manifest-path docs/work-packages/20260618-perfarch02-array-authoritative-hot-path-state-redesign-001/artifacts/perfarch02-floor-prototype/Cargo.toml` | PASS |
| `cargo clippy --manifest-path docs/work-packages/20260618-perfarch02-array-authoritative-hot-path-state-redesign-001/artifacts/perfarch02-floor-prototype/Cargo.toml -- -D warnings` | PASS |

## Prototype Timing

| Gate | Result |
|---|---|
| `cargo run --release --manifest-path docs/work-packages/20260618-perfarch02-array-authoritative-hot-path-state-redesign-001/artifacts/perfarch02-floor-prototype/Cargo.toml` | PASS; raw output recorded in `perfarch02-floor-prototype.tsv` |

The prototype validates success-path exported-map identity and failure-path
rejection semantics before printing timing results. A failure in either check
would abort the timing run.

## Documentation Gates

| Gate | Result |
|---|---|
| `markdown-doc lint --path docs/ROADMAP.md --path docs/work-packages/README.md --path docs/work-packages/20260618-perfarch02-array-authoritative-hot-path-state-redesign-001 --format json` | PASS; 14 files scanned, 0 errors, 0 warnings |
| `git diff --check` | PASS |

## Full Workspace Gates

Full production workspace gates were not run because PERFARCH02 did not edit
production Rust crates. The scoped Rust gates above apply to the
artifact-local prototype crate only, matching the package acceptance criteria.

Not run:

- `cargo fmt --check` for the full workspace;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `cargo deny check`;
- H2637 production timing.

The next production migration package must run the full affected Rust gates and
H2637 timing once it edits production crates.
