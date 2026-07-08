# Gate Results

Status: `PASS`
Evidence mode: Ran + Static.

## Summary

All required completion gates passed for the dx5 active production mesh-policy
ratification package. `SC-OFEROUTE-001` rev 45 authorizes the active
production default as target `dx = 5.0 m`, with the retained `10` cell floor,
`4096` cell cap, and `300 s` active max-substep cap.

The package did not touch required-case bindings, cohort fixtures, or
external-authority suite posture; the authority anti-evasion guard was not
triggered.

## Evidence And Runtime Gates

| Gate | Result | Evidence |
|---|---:|---|
| Package analyzer compile and replay | PASS | `PYTHONPYCACHEPREFIX=/tmp/openwepp-dx5-policy-pycache .venv/bin/python -m py_compile .../analyze_dx5_production_matrix.py .../run_default_dx5_evidence.py` and analyzer replay wrote `artifacts/rev44-promotion-matrix.json`. |
| Promotion matrix | PASS | `DX5_PRODUCTION_RATIFIED_BY_EVIDENCE`, `21` rows, `0` blockers, `0` missing annual rows. |
| Exact release-binary provenance | PASS | `cargo build --release -p openwepp-runner --bins`; `target/release/openwepp-cli-hill` SHA256 `3f60d8bd064a11c514edd1558951051782f2e757f4ce71ce4b2e7be292c9524b`. |
| Selected-cohort active default/no-env dx5 evidence | PASS | `artifacts/default-dx5-evidence.md` records all three selected real-cohort members using `mesh_policy.mode = target_dx`, `target_dx_m = 5.0`, `min_cells = 10`, `max_cells = 4096`, `max_dt_s = 300.0`. |
| Default/no-env versus explicit target dx5 identity | PASS | `3/3` active comparisons passed across HBP, loss JSON, pass parquet, WAT parquet, and active trace JSONL; mismatch count `0`. |
| Protected default/off byte identity | PASS | `3/3` off-mode comparisons passed across HBP, loss JSON, pass parquet, and WAT parquet; mismatch count `0`. |
| Active closure and `INV-OFEROUTE-012` | PASS | Active default closure assertions passed for all selected real-cohort members; max no-env closure residuals are below `1e-10` class gates and clamp/source max is `8.44135663405994e-19`. |
| DC01-disable / no-double-feed proof | PASS | Static proof in `artifacts/consumer-path-proof.md`; runtime active runs completed through the guarded active publication path without double-feed guard failure. |
| Routed-hydrograph-to-erosion consumer proof | PASS | Static proof in `artifacts/consumer-path-proof.md`; runtime manifests record routed days, trace rows, terminal outlet totals, tail-fold/end-window surfaces, and counted shape classes. |
| Shadow mesh decision | PASS | Shadow mesh remains unchanged and out of scope: `LANED_SHADOW_CELLS = 10`. |

## Rust And Test Gates

| Gate | Result | Command |
|---|---:|---|
| Focused orchestrator Lane D mesh-policy tests | PASS | `cargo test -p openwepp-hillslope-orchestrator laned_active --lib` (`7` passed). |
| Focused runner Lane D mesh-policy tests | PASS | `cargo test -p openwepp-runner laned_active --lib` (`6` passed). |
| Formatting | PASS | `cargo fmt --check`. |
| Clippy | PASS | `cargo clippy --workspace --all-targets -- -D warnings`. |
| Full workspace test suite | PASS | `cargo nextest run --workspace --profile full`: `1424` tests passed, `3` skipped, elapsed `585.150s`. |
| Dependency policy | PASS | `cargo deny check`: advisories, bans, licenses, and sources passed. |

## Documentation And Contract Gates

| Gate | Result | Command |
|---|---:|---|
| Tracked diff whitespace | PASS | `git diff --check`. |
| Markdown/doc lint | PASS | `markdown-doc lint --path docs/ROADMAP.md --path docs/work-packages/README.md --path docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md --path docs/work-packages/20260708-laned-router-dx5-production-mesh-policy-ratification-001 --format json`: `19` files scanned, `0` errors, `0` warnings. |
| Contract binding exposure | PASS | `.venv/bin/python tools/check_sc_binding_exposure.py docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`: `PASS-DEFERRED` with `8` BEI rows and `7` science-review-follow-on rows not yet consolidated. |
| Unit compliance | PASS | `bash tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`: no findings. |
| Unit registry | PASS | `bash tools/release/check_unit_registry.sh`: `21` registry tests passed. |
| Authority anti-evasion guard | NOT RUN | Not triggered; no required-case binding, cohort fixture, or external-authority suite posture was touched. |

## Review And Verification Gates

| Gate | Result | Evidence |
|---|---:|---|
| Comparator verification | PASS | `artifacts/verification-comparator.md`. |
| QA verification | PASS after disposition | `artifacts/verification-codex.md` findings were accepted and fixed; see `artifacts/disposition.md`. |
| Code/process review | PASS after disposition | `artifacts/review-codex.md` findings were accepted and fixed; see `artifacts/disposition.md`. |

## Cost Record

Cost is priced but not gating under the standing fidelity-first operator
posture.

Selected real-cohort aggregate user time from the promotion matrix:

| Policy | User seconds |
|---|---:|
| Fixed 10-cell baseline | `18.18` |
| dx5 production policy | `88.68` |
| Ratio | `4.877887788778878` |
