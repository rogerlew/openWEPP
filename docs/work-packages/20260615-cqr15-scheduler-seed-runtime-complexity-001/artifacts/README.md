# CQR15 Artifacts

Artifact set for CQR15. Evidence entries must label `Static:` versus `Ran:`.

Status: complete-with-warnings.

Static: package target is
`crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs`
function `seed_wb11_runtime_surface_inputs`.

Ran: before LCOV and CRAP artifacts are stored as `lcov_before.info` and
`crap_before.json`.

Ran: after LCOV and CRAP artifacts are stored as `lcov_after.info` and
`crap_after.json`.

Static: supporting reports in this directory record quality plan, public API
parity, behavior equivalence, line-count governance, kernel-profile compliance,
dual review, dual verification, disposition, and worker handoff.

Ran: final required gates passed. See `gate-results.md`.
