# Progress

Static: package scaffolded for row #7 WB publication CQR execution.

Ran:

- Reused the final post-row-4 full-workspace LCOV + CRAP JSON:
  `/tmp/openwepp-crap-row4-after-final.json`.

Result:

- Row #7 CRAP-before extraction found 17 unique production offender entries
  above 30. `cargo crap` currently reports each row twice in this workspace
  build, giving 34 duplicated report rows.

## Work Log

- Package scaffolded after row #4 commit `cdb01aeb`.
- Added row #7 typed assertions in runner and orchestrator tests for retained
  publication-frame validation, snow/frost insulation helpers, selector
  parsing, Sturm climate normals, growth/residue projection, Priestley-Taylor
  demand, no-final-frost rebalance, frost carry projection, WB11 frozen-depth
  refresh, and WB16 equivalent-plane alpha.
- Split `DirectProductionDayInputBuilder::build` into behavior-preserving
  climate-day, lane, residue-cover, erosion, and frost-context helpers.
- Split direct-production trace filtering/path handling out of the WB15 trace
  writer, preserving empty-env and day/lane filter semantics.
- Split snow-density selector parsing into helper functions while keeping the
  source markers in `00_builders_and_authority.rs` for existing marker
  contracts.
- Re-measured row #7 after the final clippy-clean code:
  `/tmp/openwepp-crap-row7-after-final.json`.
- Row #7 CRAP-after found `0` owned production entries above 30. Original
  offenders now top out at CRAP `29.724834148678866`
  (`direct_production_typed_growth_crop_authority`) and `29.3948188197701`
  (`DirectProductionDayInputBuilder::build`).
- Full gates passed: `fmt`, `diff --check`, clippy, full nextest, deny,
  authority anti-evasion, and auth11 obligation guard.
- H2637 protected outputs remained byte-identical against the retained
  baseline, with `compatibility_edge_invocations=0`.
- Line-count governance recorded a row-scoped exception for the pre-existing
  `00_builders_and_authority.rs` monolith; see `line-count-governance.md`.
