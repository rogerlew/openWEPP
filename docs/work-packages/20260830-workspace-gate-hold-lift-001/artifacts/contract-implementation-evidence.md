# Contract implementation evidence

Status: `SUSPENDED — V56/V57 UNRESOLVED`

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

### SC-SNOWENERGY-001 v33 corrective implementation

Status: `IMPLEMENTED — FOCUSED PASS — CANONICAL DEFAULT-STACK HOLD`

Evidence mode: `Static + Ran`

Static: current canonical source is `SC-SNOWENERGY-001@33`, SHA-256
`a3a4de78b02af73de9a336ece5e6569d2871ea58c6f55512d75100f68a1e016c`.
Production removes the v31/v32 synthetic controller from control flow and
implements the corrective private reduced residual solve in canonical
`W/H/E/T` coordinates with one shared 96 physical-evaluation budget. The
transition-reset detector, dense safeguarded solve, physical residual/side
gates, and `CoupledAuthentic` admission retain fresh physical replay/reseal and
all prior owner, receipt, event, conservation, rollback, and publication
requirements.

Static: soil state is consumed natively through typed V1/V2 read views and
unpublished candidates. V2 exact carry, operand ordering, source identity, and
predecessor state remain in the native trial; no V1 projection/cache or trial
installation exists. The private CN operand has no digest or publication
eligibility. Only the existing final accepted segment creates the sealed V2
candidate/receipt/install.

Ran: v33 focused vectors pass 7/7, the canonical terminal numerics contract
passes 10/10, native V2 resident vectors pass 4/4, and exact-carry refusal and
rollback vectors pass 6/6. Terminal batch and exact successor-partition parity
tests pass. The canonical one-day fixture remains unavailable for contract
closure because its default test thread overflows before returning a physical
result; the retained latest log and mechanical frame evidence are recorded in
`implementation-test-evidence.md` and `gate-results.md`. No canonical runtime,
step-count, width-distribution, or ledger-closure claim is made yet.

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

Final hardening requires all four governed full-trial norms finite before the
typed threshold-excess refusal can be produced. NaN and either infinity now
leave the halved no-update witness unavailable and retain the ordinary strict-
decrease/fail-closed path. No threshold, contract, API, or diagnostic changed.
Focused 6/6 and full LSE 103/103 pass, runs
`6efcec2e-2666-4a58-b911-80a2267bf0dd` and
`86f824a0-4486-4b9d-80ff-fe8fe0e8fbfd`.

## SC-SNOWENERGY-001 v56/v57 terminal disposition

Status: `IMPLEMENTED — FOCUSED PASS — CANONICAL FAIL — SUSPENDED`

Evidence mode: `Static + Ran`

Static: v56 supplies the frozen temperature-primary specialization. V57 adds
only the contract-bounded external-liquid eligibility and zero-charge
post-root transition; the exact operand remains unchanged and no production
diagnostic is retained. The corrected eligibility includes the intended
terminal one-volume modeled domain while still refusing actual event,
model-change, melt-change, and liquid-change cases outside the contract.

Ran: v57 contract-derived/source-binding tests pass, and the retained v56
focused tests pass. Canonical r151 nevertheless fails at `1800..1860 s` inside
the V56 safeguarded physical solve, after V57 dispatch. It exits `101` after
`5:09.55` wall time; retained log SHA-256 is
`d4a26e0194a769c1303cc7500ea254d2a9dbcdaa08e05f65188e4ba07ea27252`.
Focused passage is not canonical convergence evidence. Neither version has a
completed one-day result, accepted/rejected microstep totals, final width
distribution, runtime qualification, or final ledger closure. Both remain
unresolved and no package-completion claim is made.
