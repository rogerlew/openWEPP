# Gate Results

Status: complete.

Already run during implementation:

| Command | Result |
| --- | --- |
| `cargo test --test sim_contract_boundary_unit_registry cqr16 -- --nocapture` | Pass, `6 passed; 0 failed; 15 filtered out` |
| Before `cargo llvm-cov ... lcov_before.info` | Pass |
| Before `cargo crap ... crap_before.json` | Pass with recurring no-matching-LCOV warning |
| After `cargo llvm-cov ... lcov_after.info` | Pass |
| After `cargo crap ... crap_after.json` | Pass with recurring no-matching-LCOV warning |

Final required gate transcript:

| Gate | Status |
| --- | --- |
| `cargo fmt --check` | Pass |
| `cargo clippy --workspace --all-targets -- -D warnings` | Pass |
| `cargo test --workspace` | Pass |
| `cargo deny check` | Pass: `advisories ok, bans ok, licenses ok, sources ok` |
| `markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260615-cqr16-unit-registries-complexity-001 --format json` | Pass: `files_scanned: 22`, `errors: 0`, `warnings: 0` |
| `git diff --check` | Pass |

Ran: final gates passed before package disposition was moved to
complete-with-warnings.
