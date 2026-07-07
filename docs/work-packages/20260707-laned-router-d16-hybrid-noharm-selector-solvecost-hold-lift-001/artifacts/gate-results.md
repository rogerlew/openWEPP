# Gate Results

Status: COMPLETE. Evidence mode: Ran.

| Gate | Status | Evidence |
|---|---:|---|
| `git diff --check` | PASS | Final run after review/disposition fixes: exit 0. |
| Markdown/doc lint for touched docs | PASS | `markdown-doc lint --path docs/work-packages/20260707-laned-router-d16-hybrid-noharm-selector-solvecost-hold-lift-001 --path docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md --path docs/work-packages/README.md` -> 19 files validated, 0 errors, 0 warnings. |
| `SC-OFEROUTE-002` unit compliance | PASS | `bash tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md` -> PASS. |
| `SC-OFEROUTE-002` BEI check | PASS | `python3 tools/check_sc_binding_exposure.py docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md` -> tool output `PASS-DEFERRED` for 4 BEI rows with existing `science-review-follow-on` posture; no missing BEI row. |
| Focused selector tests | PASS | `cargo nextest run -p openwepp-hillslope-orchestrator hybrid_request_selects_exact_bare_skin_lane_day hybrid_request_falls_back_to_plain_on_post_growth_vegetation` -> 2/2 passed. |
| Focused Lane-D / `ofe_routing` tests | PASS | `cargo nextest run -p openwepp-hillslope-orchestrator direct_runtime::laned_active ofe_routing` -> 103/103 passed. |
| H2637 active timing | PASS | H2637 active plain `40.05 s` user; hybrid request `33.62 s`; selected hybrid `11590/11590`; `980804` implicit steps; `0` map evals. |
| Selected-cohort active timing | PASS | Aggregate active plain `57.01 s` user; hybrid request `50.58 s`; timing no-harm lifted. |
| Protected default/subsystem-off identity | PASS | Static isolation: new branch reachable only through active opt-in request; active plain manifests carry zero request counters; full suite passed. |
| Active-mode closure | PASS | Selected cohort hybrid-request max residuals <= `4.58e-13` cascade, `4.08e-14` seam, `4.44e-13` identity. |
| DC01-disable / no-double-feed proof | PASS | Fallback path remains `laned_active_route_lane`, which runs `laned_active_assert_no_dc01_surface_feed` before both hybrid and plain route calls. |
| Routed-hydrograph-to-erosion proof | PASS | Fallback and selected branches share the same post-route D13 producer flip to `RoutedHydrograph`; selected-cohort outputs verify active manifests and pass surfaces. |
| `cargo fmt --check` | PASS | Ran after code edits and after clippy fix. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Clean after adding `#[must_use]`. |
| `cargo nextest run --workspace --profile full` | PASS | 1442/1442 passed, 4 skipped. |
| `cargo deny check` | PASS | advisories, bans, licenses, sources OK. |
| Authority anti-evasion guard | PASS | `bash tools/release/check_authority_suite_antievasion.sh` -> PASS. |

All package-required gates have current direct evidence. Remaining holds are
outside this package's no-harm selector scope and are carried in
`final-disposition.md`.
