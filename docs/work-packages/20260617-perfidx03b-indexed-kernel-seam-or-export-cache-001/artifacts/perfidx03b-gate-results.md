# PERFIDX03B Gate Results

Ran: package and repository gates on 2026-06-17.

| Gate | Result | Evidence |
| --- | --- | --- |
| `cargo fmt --check` | PASS | exited 0 |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | exited 0 |
| `cargo test --workspace` | PASS | exited 0 |
| `cargo deny check` | PASS | `advisories ok, bans ok, licenses ok, sources ok` |
| `git diff --check` | PASS | exited 0 |
| `markdown-doc lint --path docs/work-packages/20260617-perfidx03b-indexed-kernel-seam-or-export-cache-001 --path docs/work-packages/README.md` | PASS | 16 files validated, 0 errors, 0 warnings |
| Focused indexed tests | PASS | `cargo test -p openwepp-kernel-contract indexed -- --nocapture` |
| Focused persistent indexed refresh test | PASS | `cargo test -p openwepp-hillslope-orchestrator perfidx03b_persistent_state_refreshes_indexed_writeback_surface -- --nocapture` |
| Focused CLI03 regression subset | PASS | `cargo test -p openwepp --test cli03_runner_contract_derived_tests cli03_m -- --nocapture`, 8 passed |
| OFE5 speed | PASS | current mean `25.45s` vs baseline mean `26.82s` |
| OFE5 same-run-name identity | PASS | byte-stable outputs equal; pass parquet rows equal |
| H2637 anchors | PASS | both UI variants completed |
| OFE1-OFE5 ladder | PASS | all five cases completed |

## Intermediate Failures Disposition

- Clippy initially rejected a redundant `continue` in the new indexed merge
  loop. Disposition: accepted and fixed.
- Full workspace tests initially exposed missing registry coverage for
  `frost.runtime_fgfrst_0002_0017`. Disposition: accepted and fixed by adding
  conservative valid frost fine-layer registry coverage.
- Initial H2637 no-UI run failed at manifest write because the manifest
  directory was absent after completing simulation output generation.
  Disposition: harness setup fixed and H2637 rerun cleanly.
