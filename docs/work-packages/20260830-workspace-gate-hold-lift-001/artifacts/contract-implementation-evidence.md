# Contract implementation evidence

Status: `V31 CONTRACT/TEST AUTHORED — PRODUCTION NOT STARTED`

Evidence mode: `Static + Ran`

`WGHL-FULL-001D` now carries canonical candidate authority
`SC-SNOWENERGY-001@31`, SHA-256
`02ee1fb365626db5c77e601a46f56a4f8e88dea99d7d5834ad7add029f4c740c`.
`INV-SNOWENERGY-055` and `OBL-SNOWENERGY-C-023` admit only one exact-floor
terminal-one-volume `w=0.5` unpublished midpoint. It is reconstructed in
canonical `W=ice+liquid` and `H=Lf*liquid-cold_content` coordinates from a
common immutable beginning plus one coordinated complete-support operand
vector, then projected through the existing version-22 phase equilibrium.

Static: v31 explicitly rejects independent interpolation of ice, liquid, cold
content, melt, refreeze, or cumulative fields. It retains exact
schema/model/lane/cursor/layer/density/settling/initial/input/support/topology/
custody/receipt identities, raw authentic history, component and cumulative
closure, and fresh-authentic-only finalization/replay/acceptance/publication.
The exact 60-second floor, 96-iteration cap, convergence and ledger tolerances,
equations, event chronology, rollback, and public API remain unchanged.

Static: rejected v30 remains historical evidence only. Its independent
persistent-component/cumulative interpolation helper, consumer seam, tests,
index row, and impact-map bindings were removed before v31 was authored. V31
does not restore the 64-weight v30 search or any relaxed publication route.

Ran: contract-derived tests precede production. An isolated unchanged-production
sentinel at commit `6fa804082273c1c4340614ffc208a74a8b48e408` fails exactly
with `E0425` for missing
`CoveredExactFloorTerminalPhaseSupportImageV1` and
`covered_exact_floor_terminal_phase_iterate_v1` (exit 101). Production
`fixed_point.rs` and `open_snow.rs` remain unedited by v31. Implementation,
real DFF-WS2 execution, A0/canonical gates, review, and promotion remain
pending; no implementation or activation claim is made.

## WGHL-FULL-001F covered no-update witness

Status: `IMPLEMENTED — FOCUSED PASS`

Evidence mode: `Static + Ran`

Static: `SC-LANDSURFACEENERGY-001@13` adds
`INV-LANDSURFACEENERGY-139`. `solver_covered_solve.rs` retains the existing
domain-valid full-trial no-update witness. Only when that witness cannot accept
because the full trial is domain-invalid or a governed full prospective step
exceeds its unchanged threshold, the solver identifies the first domain-valid
halved `b>=1` trial. A passing current complete residual vector and passing
hydraulic, beta, temperature and humidity prospective norms accept the current
iterate. The trial is neither installed nor projected; the accepted solution,
evaluation, branches, ledgers and owner candidates remain the current values.

Static: every actual update still requires strict residual decrease. The
implementation does not skip a failed first domain-valid halving to obtain a
smaller witness. A failed prospective evaluation, residual poison, governed
step poison, or exhausted strict-decrease search retains the prior typed
failure and rollback behavior. No bound, threshold, equation, pivot,
iteration/backtracking limit, floor, event, ledger, receipt, custody, topology,
publication, public API, or serialized state changed. No runtime diagnostic
print remains.

Ran: full LSE crate tests pass `84/84`, run
`9a5aaf67-de5a-4c85-b149-225c52196c66`; warnings-denied all-target/all-feature
LSE Clippy passes. Both real consumers clear the exact prior `LSEB-E-034` and
reach a later Stage-3 qualification chronology guard; terminal cross-owner
consumer disposition remains with that owning correction.

Review correction: the dedicated first-domain-valid `b>=1` witness search now
precedes the unchanged ordinary `b=0..20` strict-decrease search, so a
strictly-decreasing full step cannot bypass the required witness. Trigger
classes are private typed values for domain invalidity versus governed-step
threshold excess. The current complete residual vector is checked member by
member for finiteness and exact normalized passage, preventing a scalar
`f64::max` fold from masking a NaN. Terminal predicate vectors pass 3/3, run
`8cf71b71-1a6f-443a-abca-3144bb14ff4f`.

Terminal review correction: production and direct vectors share one private
ordered controller that returns only the exact examined exponent and
prospective step metadata. It cannot carry or install a trial. The exact
exponent is added to the existing cumulative backtracking-count diagnostic as
clarified prospectively in canonical v14; no separate public or persisted
field was added. Fresh focused vectors pass 5/5, run
`baaf9f04-769f-4de0-82bd-f98695c081db`, and the complete crate passes 87/87,
run `dcd3e84b-d3ce-4bae-8960-df2c2a2c1767`.
