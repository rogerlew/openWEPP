# Independent review A — WGHL-FULL-001F

Status: `HOLD`

Recommendation: `HOLD`

Evidence mode: `Static + Ran`

## Scope and reviewed identity

Static: reviewed the complete current canonical
`SC-LANDSURFACEENERGY-001` at version 14 and SHA-256
`5cae2929a01aa70d7d3ef37f2e030a9a36b552285588960b294077ee75473969`,
but limited this review to the preserved version-13 amendment,
`INV-LANDSURFACEENERGY-139`, and its implementation/evidence. The distinct
version-14 frozen-litter successor is not reviewed or dispositioned here. The
exact version-13 snapshot under review remains SHA-256
`922917e963788ae10faae699ab8c6eb95180748d53a94b15aa484a34eeadfede` as
recorded in `contract_ref.md`.

Static: reviewed the complete current
`crates/openwepp-land-surface-energy/src/solver_covered_solve.rs` at SHA-256
`f9f047632a936a9def0804c7a82890397988bb134a7e277431f6a9d7913d0331`,
the package authority, F-specific contract/test/implementation/gate evidence,
and the named real-consumer disposition.

Ran:

```text
nix develop -c cargo nextest run -p openwepp-land-surface-energy \
  -E 'test(covered_halved_no_update_witness_tests)'
```

Initial result: `3/3 PASS`, run
`be3bee22-9b8c-415a-8c95-a93de6e75710`.

Rereview result after the A-001/A-002 correction: `3/3 PASS`, supplied run
`8cf71b71-1a6f-443a-abca-3144bb14ff4f` and independently repeated as run
`31c8dcdc-871b-461c-a902-6230ef810ad2`.

Second A-002 follow-up execution attempt:

```text
nix develop -c cargo nextest run -p openwepp-land-surface-energy \
  -E 'test(covered_halved_no_update_witness_tests)'
```

Result: `BLOCKED`, exit `101` before Nextest assigned a run ID. The concurrent,
separately owned version-14 `lib.rs` declares missing modules
`litter_phase_closure`, `solver_litter_phase`, `transaction_v3`, and
`v3_state`, producing four `E0583` errors. This is an external shared-worktree
compile-red, not an 001F controller/test failure. The five-test controller
suite required a fresh run after those modules landed.

Block closure Ran: on the exact corrected source, the five controller tests
pass `5/5`, run `baaf9f04-769f-4de0-82bd-f98695c081db`. The complete
`openwepp-land-surface-energy` crate passes `87/87`, run
`dcd3e84b-d3ce-4bae-8960-df2c2a2c1767`. The temporary v14 missing-module
execution block is closed.

Ran: `git diff --check` passed for the F-owned solver, contract, and package
artifact paths. The corrected solver is 1,172 lines (`PASS`, below 2,000). A
source scan found no `eprintln!`, `println!`, or `dbg!` in the solver.

## Findings

### `001F-A-001` — CLOSED (was HIGH) — first-halved ordering corrected

Rereview Static: the corrected implementation now completes the full-trial
witness/refusal classification, then executes a dedicated `b=1..20` loop that
skips domain-invalid candidates and stops after the first domain-valid
candidate is either accepted as a no-update witness or refuses by failed
evaluation/step predicate (`solver_covered_solve.rs:545-616`). Only after that
loop completes does the unchanged `b=0..20` strict-decrease actual-update
search begin (`solver_covered_solve.rs:619-648`).

The former bypass is gone: a decreasing but governed-step-excess `b=0` trial
cannot install before the required first-domain-valid halved witness is
examined. The no-update acceptance still passes current `x` and current
`detail`, while actual updates remain guarded by
`is_strict_residual_decrease`.

Disposition recommendation: `accepted / closed`. The corrected source matches
the v13 algorithm ordering and preserves strict decrease for every installed
update.

### `001F-A-002` — CLOSED (was MEDIUM) — direct controller vectors pass

Rereview Static: the first correction materially improved the guard shape. Full
refusal is now typed as `DomainInvalid` or
`GovernedStepThresholdExceeded`; complete-current-residual acceptance checks
every member for finiteness and `abs <= 1`; the positive helper vector covers
both typed trigger classes; residual-member NaN and prospective-step NaN
poisons refuse; and the source explicitly breaks rather than searching a later
halving after the first domain-valid candidate cannot completely evaluate or
fails its step predicate (`solver_covered_solve.rs:28-59`, `:514-583`, and
`:853-982`). These changes close the collapsed-trigger, scalar-only residual,
and nonfinite-step portions of the original finding.

Second-follow-up Static: production now routes the complete `b>=1` preflight
through private controller
`covered_first_domain_valid_halved_no_update_witness`
(`solver_covered_solve.rs:68-95`, call site `:577-616`). Its probe result is an
enumerated `DomainInvalid`, `EvaluationIncomplete`, or
`Complete(CoveredStepNorms)`. The controller alone owns ordered exponents,
skips only domain-invalid trials, returns immediately on the first
domain-valid incomplete or step-refusing trial, and returns only
`(exponent, step metadata)` on acceptance. Its type cannot return or install a
trial. The production acceptance call consumes current `x` and current
`detail`; the ordinary strict-decrease loop remains subsequent and begins at
`b=0`.

Direct controller tests now prove domain-invalid `b=1/b=2` followed by the
first complete `b=3`, and prove that an evaluation-incomplete or step-failing
first domain-valid trial stops after `b=1` rather than skipping to a later
passing candidate. A no-trigger vector proves the probe is never called.
Together with the typed-trigger, full residual-member, per-coordinate,
nonfinite, already-passing-full, and later-candidate predicate vectors, these
tests cover the remaining controller-level ordering/refusal obligations
(`solver_covered_solve.rs:867-981`). No-install is enforced structurally by
the controller return type and current-value acceptance call.

Disposition recommendation: `accepted / closed`. The implementation and direct
controller tests satisfy the A-002 vector gap, and fresh exact-source execution
passes both the five-test controller selection and the complete 87-test LSE
crate.


### `001F-A-003` — HIGH — mandatory real-consumer completion is not yet available

Ran/evidence review: run `ec067bbd-443d-45ce-ba76-5c4fdd2e252b` demonstrates
that both named consumers no longer return the former iteration-4
`LSEB-E-034`. Both nevertheless stop at the later
`qualification terminal snow-free successor chronology` guard
(`contract-test-implementation-evidence.md:63-68`). The v13 contract requires
both paths to complete with unchanged owner closure and no trial installation
(`SC-LANDSURFACEENERGY-001.md:1237-1241`). Those terminal facts have not yet
been observed.

Impact: this is a truthful cross-owner evidence block, not evidence that the
001F LSE correction caused the chronology failure. It still prevents the
current contract cycle and package from claiming real-consumer closure.

Proposed disposition: `follow-up` to the already inventoried chronology owner,
then rerun both unchanged consumers. Accept 001F consumer closure only when
both complete and directly prove unchanged owner/ledger closure and no trial
installation.

## Protected-boundary assessment

Static: after closure of `001F-A-001`, the no-update acceptance call supplies
the current `x` and current `detail`; no prospective trial is installed,
projected, published, or used as an owner candidate. The prospective step
norms and examined exponent are carried only through the pre-existing
numerical diagnostic fields required by v13. No new production print or
persistent microstepping trace was introduced.

Static: the diff changes no numeric literal for the hydraulic, beta,
temperature, humidity, residual, pivot, iteration, backtracking, or 60-second
support limits; no constitutive equation, closed bound, phase/event rule,
ledger, receipt, custody, topology, rollback path, public item, serialized
surface, or model-definition identity changed. Existing actual updates still
call `is_strict_residual_decrease`; natural failure and rollback evidence is
recorded as passing.

Static: the 001F-local Clippy `needless_continue` was corrected by replacing
the domain-invalid match arm's explicit `continue` with an empty arm; loop
ordering and behavior are identical. Remaining LSE Clippy diagnostics are in
separately owned v14 source and do not reopen an 001F finding.

## Final recommendation

`HOLD`. The production-code fidelity finding `001F-A-001`, test-design finding
`001F-A-002`, and its temporary execution block are closed. Only the external
chronology-dependent real-consumer evidence in `001F-A-003` remains. Rerun the
two unchanged real consumers after that chronology correction. No current code
finding indicates a threshold, bound, floor, trial-installation,
strict-decrease, rollback, public-API, or diagnostic-persistence regression.
