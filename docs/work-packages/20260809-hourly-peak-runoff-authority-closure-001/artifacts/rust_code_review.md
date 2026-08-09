# Rust Code Review

Status: `complete`

Review target: exact commit diff `a65cc3973..c7dbfefe7`.

Evidence:

- Ran: `git diff --check a65cc3973..c7dbfefe7` -- PASS.
- Static: reviewed the exact committed Rust, tests, package, and controlling
  science contracts with `git diff`/`git show`. The shared checkout contained
  later concurrent source edits, so current-worktree cargo execution would not
  have tested the requested commit and was not run.

## Findings

### Critical -- gross routed melt is counted again after WB14 partitioning

The production builder adds the snow ledger's `liquid_handoff_m` to the WB14
hyetograph and to daily liquid input while also carrying its matching 24-bin
`hourly_routed_melt_m`
(`crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00c_day_input_builder_impl.rs:348-385`).
The storage guard proves that the hourly array sums to that same handoff
(`crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs:906-938`).
WB14 therefore partitions melt as part of its liquid supply before producing
`wb14_hourly_excess_m`, but the new peak shape adds the full gross routed-melt
array to that post-partition excess a second time
(`crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs:1409-1415`,
`:1430-1458`). This does not increase the accepted daily `Q`, because the raw
shape is normalized; it silently double-counts melt in the temporal weights and
can classify melt that infiltrated as runoff timing. The resulting peak hour and
rate are not derived solely from modeled hourly surface runoff.

This violates the source-complete runoff meaning in
`SC-WATBAL-001:127-130`, the positive-hourly-volume rules at `:1254-1259`, and
the package's independent operand requirement at `package.md:96-107`. Carry a
source-tagged post-infiltration hourly runoff limb, or treat WB14 excess as the
sole partitioned local-liquid limb; do not add gross melt alongside its own
partition result. Add end-to-end melt-only fixtures in which melt is fully and
partially infiltrated, using the real runner builder rather than independent
arrays.

### Critical -- missing runon timing still becomes a silent uniform series

`dc01_distribute_runon_supply` is a production admission function that divides
positive surface or lateral runon uniformly over 24 hours when its corresponding
shape total is zero
(`crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs:640-695`).
It then injects that synthesized array into WB14 at `:658-660`; the later
positive-source check at `:1442-1451` sees nonzero WB14 excess and cannot tell
that upstream timing was invented. The committed test explicitly ratifies this
behavior
(`crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_dc01.rs:27-44`).

That is the forbidden uniform fallback in `SC-WATBAL-001#INV-WATBAL-102` and
contradicts the package's fail-closed rule (`package.md:80-85`). Change the
distribution seam to return a typed error whenever a positive runon component
lacks its own authoritative hourly shape. Reconcile the older dry-runon
time-base requirement with the newer WB16 authority explicitly; an infallible
uniform default cannot carry the production peak claim.

### Medium -- the declared WB16 error taxonomy is not reachable from WB16

The contract assigns `HKERNEL-WB16-PEAK-E-001..003` to missing, non-finite, and
domain/closure failures (`SC-WATBAL-001:1277-1281`), and the hydrology guard
family already maps `HydrologyPeakRunoff` to those codes
(`crates/openwepp-hillslope-orchestrator/src/hydrology/02_guard_errors.rs:632-681`).
The new operator instead returns generic `DirectRuntimeError` variants
(`crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs:1436-1450`,
`:1494-1537`), whose display path emits no WB16 code
(`crates/openwepp-hillslope-orchestrator/src/direct_runtime/04_audit_error_helpers.rs:254-279`,
`:586-629`). Thus the specified typed codes are not produced at the actual
runtime seam. Route WB16 validation through the phase-specific guard type (or
an equivalent typed wrapper) and assert code plus boundary class from the real
span.

### Medium -- retired rainfall-envelope operands remain production inputs

`DirectPeakRunoffInputs` still exposes the old hyetograph, irrigation rate,
`efflen`, `ealpha`, and exponent
(`crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs:2396-2403`).
The runner still computes the WB16 `ealpha` authority, constructs these inputs,
and copies them into every day frame even though the new peak compute reads none
of them
(`crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs:1065-1081`,
`crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00d_authority_runtime_impl.rs:624-633`,
`crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00c_day_input_builder_impl.rs:401`).
This leaves obsolete rainfall-envelope configuration in the production runtime
seam and permits unused legacy derivation to block or drift an hourly-authority
run. It conflicts with `SC-WATBAL-001` rule 8 (`:1268-1271`). Remove the peak
input surface and its obsolete derivations; retain any independently needed
geometry under its real erosion/publication owner, and isolate historical fields
behind an explicitly diagnostic schema.

## Residual Risk And Missing Tests

- The committed behavioral tests exercise isolated arrays. They do not execute
  the real runner snowmelt-to-WB14-to-peak chain, so they cannot detect the melt
  overlap above. The melt fixture even accepts `0.002 m` event runoff from a
  `0.004 m` raw melt shape
  (`direct_runtime_dc01.rs:47-75`).
- There is no production-path negative test proving that positive surface or
  lateral runon with missing timing returns the required typed WB16 failure.
- `tests/integration/peak_hourly_authority_contract.rs:12-86` is a source-string
  marker test, not downstream-consumer evidence. Add an independent HBP/public
  reconstruction test covering 24 hourly volumes, event volume, peak hour,
  rectangular duration, and two areas.
- The public area conversion at
  `direct_runtime/01_publication.rs:582-619` applies area once on the inspected
  normal path, but negative and inconsistent shadow/publication-basis states are
  not validated directly. Add cross-basis zero/nonzero and malformed-shadow
  tests so future producer drift fails closed.
- The `1e-12 m` source-informed zero canonicalization at
  `direct_runtime/runoff.rs:1362-1375` matches the contract's bounded rule; no
  arithmetic blocker was found there. The exact-commit full workspace and full
  1,088-case Topanga evidence remained pending and was not independently rerun
  in this review.

## Verdict

`HOLD` -- the two Critical findings can silently change hourly peak timing and
therefore erosion/public runoff despite daily runoff closure. The package is not
acceptable at `c7dbfefe7` until they are corrected and independently exercised.
