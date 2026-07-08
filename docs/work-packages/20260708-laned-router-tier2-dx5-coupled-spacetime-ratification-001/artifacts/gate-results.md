# Gate Results

Status: `EXECUTED-HOLD-DX5-UNRATIFIED`
Evidence mode: Ran.

| Gate | Status | Evidence |
|---|---|---|
| `git diff --check` | PASS | Ran locally; exit code 0 |
| Markdown/doc lint | PASS | `markdown-doc lint --path docs/work-packages/20260708-laned-router-tier2-dx5-coupled-spacetime-ratification-001 --path docs/work-packages/README.md --path docs/ROADMAP.md` -> 21 files, 0 errors, 0 warnings |
| Handoff/analyzer scripts compile | PASS | `.venv/bin/python -m py_compile .../run_coupled_spacetime_ladder.py .../analyze_coupled_spacetime.py` |
| Exact release-binary provenance | PASS | `coupled-spacetime-summary.md`: `cargo build --release -p openwepp-runner --bins`; `target/release/openwepp-cli-hill`; SHA256 `8876fa04ca520126b958d83a7c5777da6f793e51fba4c346432f065b31647aaa`; execution HEAD `8faa56b43ed42f54fd40e64a94a002ad372240cc` |
| Selected real-cohort coupled ladder | PASS | Comparator subagent ran `.venv/bin/python .../run_coupled_spacetime_ladder.py --members mn_corn_h4 n_idaho_forest_h1 wa_cascades_forest_h1`; 21/21 rungs PASS; no active closure/clamp-source failure |
| Same-`dt` spatial fine-reference adequacy | FAIL | `mesh-policy-ratification.md`: `mn_corn_h4` `dx2p5_dt300` vs `dx1p25_dt300` shape `0.020180511 > 0.016666667`; `wa_cascades_forest_h1` `dx2p5_dt75` vs `dx1p25_dt75` annual sediment `tdep:4` `0.022131684 > 0.0066666667` |
| Same-`dt` `dx5` candidate comparison | PASS | `dx5_dt300` vs `dx2p5_dt300` and `dx5_dt75` vs `dx2p5_dt75` pass production tolerance class for all selected real-cohort members |
| Same-`dx` timestep-refinement controls | PASS | `timestep_control_dx5`, `timestep_control_dx2p5`, and `timestep_control_dx1p25` all pass production tolerance class after analyzer replay |
| Coupled adjudication replay | PASS | `.venv/bin/python .../analyze_coupled_spacetime.py` -> `EXECUTED-HOLD-DX5-UNRATIFIED`, 2 blockers |
| Focused active trace / selector tests | PASS | `cargo test -p openwepp-runner laned_active --lib` -> 6 passed |
| Focused Lane D / `ofe_routing` tests | PASS | `cargo test -p openwepp-hillslope-orchestrator laned_active --lib` -> 7 passed |
| Contract/profile/BEI checks | NOT RUN | No `SC-*` contract changed; `contract-disposition.md` records no amendment |
| Protected default/off byte identity | NOT RUN | No production/default/off surface changed; no default flip landed |
| DC01/no-double-feed proof | NOT RUN | No production active routing change landed; existing active-owner proof remains unchanged |
| Routed-hydrograph-to-erosion consumer proof | NOT RUN | No production active routing change landed; existing active-owner proof remains unchanged |
| Source-level anti-evasion guards | NOT RUN | Did not touch required-case bindings, cohort fixtures, or external-authority suite posture |
| `cargo fmt --check` | PASS | Delegated full-gate runner reported PASS, exit code 0 |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Delegated full-gate runner reported PASS, exit code 0 |
| `cargo nextest run --workspace --profile full` | PASS | Delegated full-gate runner reported PASS, exit code 0; 1424 tests started across 163 binaries, 3 skipped |
| `cargo deny check` | PASS | Delegated full-gate runner reported PASS, exit code 0 |
| Line-count governance | PASS | `line-count-governance.md`; no `.rs` edits and inspected relevant active-router files below WARN threshold |
