# Gate Results

Status: `COMPLETE`
Evidence mode: Ran.

## Runtime And Selector Gates

| Gate | Result | Evidence |
|---|---|---|
| Focused Lane D integration | PASS | `cargo nextest run --workspace --profile full --test laned_shadow_h2637` -> `8` tests run, `8` passed, `2` skipped; wall `38.125s`. |
| All-extended default/no-env active runtime proof | PASS | Ignored H2637 vector `h2637_native_active_owner_routes_and_closes` passed; default/no-env emitted `laned_active` with `days_routed=610`. |
| Default-active equals explicit-active protected outputs | PASS | HBP and pass parquet hashes match: `28897e...eb5dd5` and `bbb0a...961cc`. |
| All-legacy default/no-env fallback | PASS | Focused integration asserts no `laned_active`; HBP SHA `453e44...c072f`, pass parquet SHA `fd47c3...966a`. |
| Mixed coefficient authority fail-closed | PASS | `h2637_default_mixed_routing_coefficients_fails_closed`. |
| Malformed route coefficient extension fail-closed | PASS | `h2637_default_malformed_routing_coefficients_fails_closed`. |
| Explicit active missing coefficients fail-closed | PASS | `h2637_active_fails_closed_without_routing_coefficients`. |
| Active+shadow conflict | PASS | `h2637_active_and_shadow_are_mutually_exclusive`. |
| Active+disable conflict | PASS | `h2637_active_and_disable_are_mutually_exclusive`. |
| Explicit disable rollback | PASS | Ignored H2637 vector explicit-disable leg emitted no `laned_active` block. |
| Active closure evidence | PASS | Default-active H2637 manifest: `max_supply_reconstruction_rel=7.31201193525081e-16`, `max_day_cascade_residual_rel=2.2762831518726353e-13`, `max_day_seam_residual_rel=5.0415846159888125e-14`, `max_day_identity_residual_rel=2.1906143827108124e-13`. |
| DC01-disable / no-double-feed proof | PASS | Static: active executor path unchanged from rev 27; no mixed/default fallback enters active execution. Ran: default-active active summary closed with rev-27 active owner. |
| Routed-hydrograph-to-erosion consumer proof | PASS | Static: active owner path unchanged from D13/D15A; ran default-active active summary includes routed outlet/tail/degenerated-shape counters. |

## Command Gates

| Gate | Result | Evidence |
|---|---|---|
| Contract-first amendment | PASS | `SC-OFEROUTE-001` rev 46 landed before runtime selector closure. |
| Unit/contract-derived selector coverage | PASS | Integration-level contract vectors cover all resolver states available through the public run surface: complete/default active, absent/default fallback, lane-mixed fail-closed, malformed extension fail-closed, explicit active, explicit disable, and selector conflicts. Direct unit construction would require fabricating private included builder state and was not added; this is accepted as the package substitute because it exercises the parser/projection/consumer path, not only a pure enum. |
| `git diff --check` | PASS | No findings. |
| Markdown/doc lint | PASS | `markdown-doc lint --path docs/ROADMAP.md --path docs/work-packages/README.md --path docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md --path docs/work-packages/20260708-laned-router-conditional-default-activation-001 --format json` -> `17` files scanned, `0` errors, `0` warnings. |
| Contract/profile/BEI checks | PASS | Static review in `contract-disposition.md`: front matter, branch/guard, invariant, guard map, test vectors, BEI, and revision history updated. No separate BEI script is defined for SC-OFEROUTE-001 in this scope. |
| `.rs` line-count disposition | PASS | Touched Rust files remain existing long included/test surfaces; new selector logic was extracted to `resolve_laned_active_enabled()` and clippy is clean. Line counts: `05_runner_execution_and_outputs.rs=1540`, `00_builders_and_authority.rs=2804`, `laned_active.rs=403`, `laned_shadow_h2637.rs=652`. No scope-safe split was needed. |
| `cargo fmt --check` | PASS | No findings. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | No warnings. |
| `cargo nextest run --workspace --profile full` | PASS | `1427` tests run, `1427` passed, `3` skipped; wall `627.073s`. |
| `cargo deny check` | PASS | `advisories ok, bans ok, licenses ok, sources ok`. |
| Authority anti-evasion guard | NOT RUN | No required-case binding, cohort fixture, or external-authority suite posture was touched. |

## Review And Verification

| Gate | Result | Evidence |
|---|---|---|
| Review | PASS after disposition | `review-codex.md` raised blocking issues; `disposition.md` records fixes/evidence. |
| Verification | PASS after disposition | `verification-codex.md` raised artifact/gate gaps; this file and updated disposition close them. |
