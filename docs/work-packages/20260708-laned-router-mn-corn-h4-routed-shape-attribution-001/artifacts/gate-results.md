# Gate Results

Evidence mode: Ran.

Status: `PASS-WITH-INTENDED-HOLD`.

## Gates

| Gate | Status | Evidence |
|---|---|---|
| `git diff --check` | PASS | Ran clean before review and after disposition fixes. |
| Markdown/doc lint for touched docs | PASS | `markdown-doc lint --path docs/work-packages/20260708-laned-router-mn-corn-h4-routed-shape-attribution-001 --path docs/work-packages/README.md --path docs/ROADMAP.md` passed after disposition fixes: 18 files, 0 errors, 0 warnings. |
| Exact release-binary provenance | PASS | `shape-attribution-summary.md` records `cargo build --release -p openwepp-runner --bins`, binary path, SHA256 `319fbe119e89193018ce9b2894dc7dab56babb7fee2543a0ec9f06f62674b56c`, and git HEAD `69813293686fcbdb7d46cfab02b5daa5d500d5d6`. |
| `mn_corn_h4` `dx2p5`, `dx1p25`, `dx0p625` active trace reruns | PASS | `OPENWEPP_LANED_ACTIVE_TRACE_DETAIL=792:1 .venv/bin/python artifacts/run_shape_attribution_ladder.py --members mn_corn_h4 --rungs dx2p5 dx1p25 dx0p625` returned `{"status": "PASS", "runs": 3}`. |
| Material run environment provenance | PASS | Each summary run record now includes `material_environment` with active routing, trace output, trace detail selector, shadow profile, and rung target dx. |
| Day-792 analysis replay tooling | PASS | `.venv/bin/python artifacts/analyze_day792_attribution.py` regenerated `day792-attribution.json` and `.md`. |
| Normalization-amplification test | PASS | Test executed; result is `FAIL-METRIC-CLASS` for metric repair because `0.011445388178193001 m3` hourly movement is about `126.6x` the `9.04e-5 m3` storage delta. |
| Hour-edge aliasing CDF test | PASS | Test executed; result is `FAIL-PROJECTION-ALIASING` because hourly CDF Linf worsens from `0.001801155375319774` to `0.009920733019868733`. |
| Raw unbinned outlet-hydrograph convergence test | PASS | Test executed; result is `SOLVER-CLASS-HOLD` because raw bin L1 and sampled hydrograph L1 worsen on the fine pair. |
| Contract/profile/BEI checks for `SC-OFEROUTE-001` | NOT RUN | `SC-OFEROUTE-001` did not change; metric repair branch was not authorized after solver-class attribution. |
| Focused contract-derived tests if contract text lands | NOT RUN | No contract text landed. |
| Focused Lane D / `ofe_routing` tests for Rust changes | PASS | Runner 3-test focused command passed; orchestrator 5-test focused command passed. |
| `cargo fmt --check` | PASS | Ran clean. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Ran clean. |
| `cargo nextest run --workspace --profile full` | PASS | `1421 tests run: 1421 passed (4 slow), 3 skipped`. |
| `cargo deny check` | PASS | `advisories ok, bans ok, licenses ok, sources ok`. |
| Source-level anti-evasion guards | NOT RUN | No required-case binding, cohort fixture, or external-authority suite posture was touched. |
| Full closure/protected-output gates | NOT RUN | No contract amendment, production mesh default flip, active ownership change, or output-surface cutover landed. |

## Intended Hold

The package gates pass for an executed-hold classification package. The
remaining blocker is the classified solver/day raw-hydrograph nonconvergence,
which is handed off to the next numerics package.
