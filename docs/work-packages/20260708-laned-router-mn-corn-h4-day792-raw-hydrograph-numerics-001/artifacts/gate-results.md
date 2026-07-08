# Gate Results

Evidence mode: Ran.

## Package Gates

| Gate | Result | Evidence |
|---|---|---|
| `git diff --check` | PASS | No output. |
| Markdown/doc lint | PASS | `markdown-doc lint --path docs/work-packages/20260708-laned-router-mn-corn-h4-day792-raw-hydrograph-numerics-001 --path docs/work-packages/README.md --path docs/ROADMAP.md --format plain`: 21 files, 0 errors, 0 warnings. |
| Exact release-binary provenance | PASS | `cargo build --release -p openwepp-runner --bins` via harness; `target/release/openwepp-cli-hill` SHA256 `df6fa6cd7fcfb2312cfc9d1fb75f9e1a79372d0c2cd7b1d61618ba7c07c698fd`. |
| `mn_corn_h4` step-trace reruns | PASS | Harness ran `dx2p5`, `dx1p25`, `dx0p625`; 3/3 PASS in `raw-hydrograph-numerics-summary.json`. |
| Mechanism-attribution replay | PASS | After rerun harness, `.venv/bin/python .../analyze_raw_hydrograph_numerics.py` regenerated `mechanism-attribution.{md,json}`. |
| Clipped outlet-bin reconstruction | PASS | Linf `3.4694469519536142e-18 m3` at `dx1p25`; `1.5265566588595902e-16 m3` at `dx0p625`. |
| Focused active trace tests | PASS | `cargo nextest run -p openwepp-runner -p openwepp-hillslope-orchestrator --profile quick laned_active`: 11 passed. |
| Focused `ofe_routing` tests | PASS | `cargo nextest run -p openwepp-hillslope-orchestrator --profile quick ofe_routing`: 70 passed. |
| `cargo fmt --check` | PASS | No output. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Completed with no warnings after narrow diagnostic-function lint fixes. |
| `cargo nextest run --workspace --profile full` | PASS | 1422 tests passed, 3 skipped. |
| `cargo deny check` | PASS | `advisories ok, bans ok, licenses ok, sources ok`. |

## Not Applicable

- Contract/profile/BEI checks: no `SC-*` contract was modified.
- Authority anti-evasion guard: no required-case binding, cohort fixture, or
  external-authority suite posture was modified.
- Production closure gates: no production activation, tolerance amendment, or
  target-`dx` flip landed.

## Hold Gate

The package intentionally exits `EXECUTED-HOLD-CFL-TIMESTEP-TRANSITION`.
The full workspace gate is green, but the fidelity blocker is not closed:
`dx1p25` vs `dx0p625` still has routed-shape L1
`0.020944940478490041` against threshold `0.0166667`.
