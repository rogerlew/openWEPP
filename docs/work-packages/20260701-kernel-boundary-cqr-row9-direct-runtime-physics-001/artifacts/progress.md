# Progress

Static: package executed for row #9 direct-runtime physics CQR closure.

Ran:

- Reused the final post-row-7 full-workspace LCOV + CRAP JSON:
  `/tmp/openwepp-crap-row7-after-final.json`.

Result:

- Row #9 CRAP-before extraction found 14 unique production offender entries
  above 30. `cargo crap` currently reports each row twice in this workspace
  build, giving 28 duplicated report rows.

## Work Log

- Package scaffolded after row #7 commit `bc2ea07e`.
- Added focused row #9 unit coverage in
  `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs` for:
  PMET compute branch helpers and guard paths, R4N surface ET PMET/manual staged
  demand paths, populated day/lane constructor validator branches, snow/frost
  carry guards, `DirectLaneFrame::commit_day`, and R4A frost rebalance.
- Refactored behavior-preserving high-complexity direct-runtime functions:
  surface ET demand component selection, day constructor guard groups, and R7H
  trace writer gating/append helpers.
- Full-workspace CRAP-after reduced row #9 owned offenders from `14` unique
  entries (`28` duplicated report rows) to `0` entries above CRAP 30.
- H2637 protected outputs remained byte-identical against
  `/tmp/typed-direct-carrier-identity/base/output`, with
  `compatibility_edge_invocations=0`.
- Full gates passed: `cargo fmt --check`, `cargo clippy --workspace
  --all-targets -- -D warnings`, `cargo nextest run --workspace --profile
  full`, `cargo deny check`, authority anti-evasion, Auth11 required-suite
  obligation guard, markdown-doc lint/validate, and line-count governance.

## Disposition

Result: `EXECUTED-COMPLETE-ROW9-CQR`.

Row #9 is complete without ADR-0021 warnings.
