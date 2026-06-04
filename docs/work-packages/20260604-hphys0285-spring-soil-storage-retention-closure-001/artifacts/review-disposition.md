# Review Disposition

Status: complete
Evidence mode: Static + Ran

## Agent A Findings

Static + Ran:
- Finding: HPHYS0285 contracts overclaimed carry/runon same-pass storage ingress while code did not implement carry/runon infiltration; broad WB12 carry infiltration broke existing erosion contract vectors.
- Disposition: accepted.
- Resolution: narrowed `SC-PERC-001`, `SC-RUNOFFPART-001`, and `SC-WATBAL-001` HPHYS0285 amendments from all-liquid/carry-runon scope to local-liquid single-OFE scope: direct rain, routed snowmelt, and irrigation. Explicitly deferred MOFE carry/runon storage-ingress promotion to follow-up under existing carry-array invariants.
- Verification: `cargo test --test erod13_wave1_core_kernel_contract -- --nocapture` passed after reverting broad WB12 carry behavior; full `cargo test --workspace` passed.

Static:
- Finding: HPHYS0285 lacked direct 24-substep coverage.
- Disposition: accepted.
- Resolution: `tests/integration/hphys0285_spring_soil_storage_retention_contract.rs` now sets `wb18_perc_lane_substeps = 24.0`, so the direct-rain vector exercises substep ingress.

## Agent B Findings

Static:
- Finding: review/verification artifacts were queued.
- Disposition: accepted.
- Resolution: review artifacts, review disposition, verification artifacts, final disposition, and handoff are completed after final verification artifact updates.

Static:
- Finding: HPHYS0285 should remain HOLD, not parity closure.
- Disposition: accepted.
- Resolution: `disposition.md` remains `Status: hold`; `full-39-suite-metrics.md` records final semantic pass `0/39`.

Static:
- Finding: future snowpack exhaustion vector would improve confidence.
- Disposition: superseded by Claude Code review finding `CLAUDE-0285-001`.
- Resolution: implemented bounded pack-exhaustion canonicalization and a synthetic large-overdraw fail-closed test.

## Claude Code Review Findings

Static + Ran:
- Finding: `CLAUDE-0285-001` — HPHYS0285 unbounded finite negative SWE canonicalization reversed the HPHYS0284 fail-closed intent for material snowpack overdraw.
- Disposition: accepted.
- Resolution: `SC-SNOWFREEZE-001#INV-SNOWFREEZE-019` now bounds corrected negative-melt carried state-loss overdraw to `0.005 m` water equivalent per day. Production code caps within-tolerance exhaustion to zero but emits a typed domain failure for material overdraw beyond that bound.
- Test: added `hphys0284_large_negative_melt_state_overdraw_fails_closed` to prove large synthetic overdraw fails closed.
- Verification: `cargo test --test hphys0284_negative_melt_snowpack_state_contract --test hphys0285_spring_soil_storage_retention_contract -- --nocapture` passed, `6 passed`; `cargo test --test clim05_snow_runtime_kernel_contract --test hphys0283_snowmelt_infiltration_partition_contract -- --nocapture` passed, `10 passed`; `cargo clippy --workspace --all-targets -- -D warnings` passed; post-review H1..H39 release runtime rerun passed `39/39` at `/tmp/hphys0285_review_remediation_20260604T203602Z`.

Static:
- Finding: `CLAUDE-0285-002` — depth-to-SWE translation remains unresolved.
- Disposition: accepted as continuation, not closed here.
- Resolution: leave package in `HOLD`; next package should include a snow-column mass trace before assigning remaining spring residual solely to WB18/WB17.

Static:
- Finding: `CLAUDE-0285-003` — scope narrowing from carry/runon overclaim to local-liquid ingress was handled correctly.
- Disposition: accepted.
- Resolution: no code change; maintain MOFE carry/runon storage-ingress as deferred follow-up scope.
