# Review Agent A

Status: complete
Evidence mode: Static

Static: reviewed current uncommitted HPHYS0290 changes in the requested contract, runner, orchestrator, unit-registry, test, and Cargo files. Ran: no commands that execute tests or validators.

## Findings

### High: WB13 missing-publication guard is masked by reset-seeded state fallback

Files:
- `crates/openwepp-runner/src/hillslope/mod.rs:1714`
- `crates/openwepp-runner/src/hillslope/mod.rs:3854`
- `crates/openwepp-runner/src/hillslope/mod.rs:6243`
- `crates/openwepp-runner/src/hillslope/mod.rs:6434`
- `crates/openwepp-runner/src/hillslope/mod.rs:6548`

The HPHYS0290 contract amendment requires missing, stale-state-shadowed, negative, or non-finite `snow.post_winter_rain_m` to fail closed before WB13 `RM` publication (`SC-WATBAL-001#INV-WATBAL-065`, `SC-RUNOFFPART-001#INV-RUNOFFPART-020`, `SC-SNOWFREEZE-001#INV-SNOWFREEZE-023`). The runner now calls `reset_daily_snow_publication_fluxes` before the scheduler and inserts `snow.post_winter_rain_m = 0.0` into the state surface while removing only the flux surface entry. WB13 then reads `snow.post_winter_rain_m` with `require_runtime_surface_scalar_prefer_flux`, which falls back from flux to state.

That means an absent producer flux can be converted into a valid zero rather than a typed publication failure. In the normal canonical scheduler, `runoff_reconciliation` currently publishes a flux before WB13, but the WB13 guard itself no longer protects the science seam if the producer fails to publish or a future phase/scheduler path omits the flux. On a nonzero post-winter rain day, that would silently under-publish `RM` by the omitted direct-rain depth.

Disposition recommendation: accepted, blocking. Hold closure until daily stale-flux reset no longer satisfies the WB13 publication requirement by state fallback. WB13 should require the explicit same-day producer flux for `snow.post_winter_rain_m` (and likely `snow.routed_melt_m`) or otherwise carry a producer-presence marker that fails closed when the flux is absent. Add a regression that removes or suppresses the producer flux after daily reset and proves WB13 fails instead of accepting the reset zero.

### Medium: HPHYS0290 validation is mostly source-text and does not prove producer lineage or stale-flux behavior

Files:
- `tests/integration/hphys0290_post_winter_rain_publication_contract.rs:9`
- `crates/openwepp-runner/src/hillslope/mod.rs:7716`
- `crates/openwepp-runner/src/hillslope/mod.rs:7754`
- `crates/openwepp-runner/src/hillslope/mod.rs:7786`
- `crates/openwepp-runner/src/hillslope/mod.rs:7818`

The new integration test checks that source files contain selected substrings, and the runner unit tests exercise WB13 publication with hand-built surfaces. Those tests do not prove that WB12/WB14 publishes the same `snow.post_winter_rain_m` term used by direct-rain hyetograph/runoff forcing, do not exercise the scheduler lifecycle after `reset_daily_snow_publication_fluxes`, and do not cover the expected stale-flux no-leak behavior across days. The substring test would still pass if the producer wrote the wrong value, if the producer omitted the flux on a real path, or if WB13 accepted the reset state fallback.

Disposition recommendation: accepted. Add functional scheduler/kernel coverage for warm rain with no snow, rain-on-snow with retained/released rain, missing/non-finite post-winter rain at the WB13 boundary after daily reset, and two-day stale-flux clearing. Record those results as `Ran:` evidence before final package disposition.

## Residual Risk And Missing Tests

Static review found the canonical contract citations for `INV-WATBAL-065`, `INV-RUNOFFPART-020`, and `INV-SNOWFREEZE-023` present in the amended contracts and the new `snow.post_winter_rain_m` unit-registry entry. I did not run `cargo fmt`, `cargo clippy`, `cargo test`, `cargo deny`, or comparator diagnostics.

The untracked `tests/integration/hphys0290_post_winter_rain_publication_contract.rs` file must be included in any eventual commit, but it should not be treated as sufficient closure evidence by itself.

## Approval Statement

Blocking finding present. Recommendation: do not approve or close HPHYS0290 until the missing-publication masking issue is fixed and the functional validation gap is addressed or explicitly dispositioned.
