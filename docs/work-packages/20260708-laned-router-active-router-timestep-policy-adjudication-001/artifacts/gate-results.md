# Gate Results

Evidence mode: Ran.

## Required Gates

| Gate | Result | Evidence |
|---|---|---|
| `git diff --check` | PASS | no output |
| Markdown/doc lint, package | PASS | `markdown-doc lint --path docs/work-packages/20260708-laned-router-active-router-timestep-policy-adjudication-001` -> 13 files, 0 errors, 0 warnings |
| Markdown/doc lint, contract | PASS | `markdown-doc lint --path docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` -> 1 file, 0 errors, 0 warnings |
| Markdown/doc lint, catalogs | PASS | `markdown-doc lint --path docs/work-packages/README.md`; `markdown-doc lint --path docs/ROADMAP.md` -> 0 errors, 0 warnings |
| Contract/profile compliance | PASS | `artifacts/kernel-profile-compliance.md` |
| BEI check | PASS | `.venv/bin/python tools/check_sc_binding_exposure.py docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` -> tool output `PASS-DEFERRED`: 8 rows, 7 science-review-follow-on rows not yet consolidated |
| Focused selector tests | PASS | `cargo test -p openwepp-runner laned_active --lib` -> 6 passed |
| Focused active router tests | PASS | `cargo test -p openwepp-hillslope-orchestrator laned_active --lib` -> 7 passed |
| Handoff/analyzer scripts compile | PASS | `.venv/bin/python -m py_compile .../run_timestep_policy_ladder.py .../analyze_timestep_policy.py` |
| Controlled timestep ladder | PASS | 6/6 `mn_corn_h4` rungs passed |
| Analyzer replay from raw traces | PASS | `.venv/bin/python .../analyze_timestep_policy.py` |
| Analyzer replay without raw traces | PASS | Temporarily moved ignored `artifacts/timestep-policy-runs/`; analyzer replayed from `timestep-policy-analysis-inputs.json` |
| `cargo fmt --check` | PASS | no output |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | finished dev profile |
| `cargo nextest run --workspace --profile full` | PASS | 1424 tests run, 1424 passed, 3 skipped, 4 slow; 579.742 s |
| `cargo deny check` | PASS | advisories, bans, licenses, sources ok |

## Ladder Provenance

Release binary:

- Build: `cargo build --release -p openwepp-runner --bins`
- Binary: `target/release/openwepp-cli-hill`
- SHA256: `8876fa04ca520126b958d83a7c5777da6f793e51fba4c346432f065b31647aaa`
- Git HEAD at build: `07a12de694040e0e30edc714f297cfdc79a67674`

Rungs:

| Rung | dx m | max dt s | Wall | Solver steps | Manifest max dt |
|---|---:|---:|---:|---:|---:|
| `dx1p25_dt300` | 1.25 | 300 | 0:05.20 | 149657 | 300 |
| `dx1p25_dt150` | 1.25 | 150 | 0:05.39 | 178103 | 150 |
| `dx1p25_dt75` | 1.25 | 75 | 0:06.20 | 242245 | 75 |
| `dx0p625_dt300` | 0.625 | 300 | 0:22.11 | 319784 | 300 |
| `dx0p625_dt150` | 0.625 | 150 | 0:23.18 | 349886 | 150 |
| `dx0p625_dt75` | 0.625 | 75 | 0:24.04 | 408066 | 75 |

## Anti-Evasion Guard

Not run. This package did not touch required-case bindings, cohort fixtures,
external-authority suite posture, or authority-suite required-case wiring.
