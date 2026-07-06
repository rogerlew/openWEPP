# Codex Review (D10B Post-Execution)

Evidence mode: Static + Ran where noted.
Reviewed commit: `1d202b10` (`Execute MOFEFID D10B: resolve GAP-OFEROUTE-005 shock-numerics reconciliation`).
Base scaffold/reference commits: `eaf5e8fb`, `d5a3b469`.

Review inputs:

- Local static read of the D10B Rust, contract, package, and artifact changes.
- Subagent Rust correctness review (`rust_code_reviewer`, Static).
- Subagent QA/test review (`rust_qa_reviewer`, Static + Ran:
  `cargo test -p openwepp-hillslope-orchestrator --release d10b_reconciliation_tests`,
  8 passed).
- Subagent contract/authority review (`explorer`, Static).
- Comparator-suite runner was still pending at artifact authoring time; the
  review-response prompt requires fresh focused gates after fixes.

## Findings

### High 1 — CFL can fail open on non-finite/unsatisfiable celerity

Source: Rust correctness review.

`prepare_step_alpha` computes true celerity without a finite/positive guard in
`crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs`.
If the computed max celerity becomes infinite, `run_with_upstream_integral`
computes `dt_cfl = 0`, then the `dt <= 0` branch breaks the loop and returns
`Ok(RoutingResult)` rather than a typed failure. This violates
`SC-OFEROUTE-001#INV-OFEROUTE-007`'s hard-fail posture for cases where no
finite positive sub-timestep can satisfy CFL.

Evidence:

- `prepare_step_alpha` true-celerity path: `kinematic_wave.rs:589`.
- `dt_cfl` selection from `max_celerity`: `kinematic_wave.rs:961`.
- `dt <= 0` branch breaks and reaches success return: `kinematic_wave.rs:967`,
  `kinematic_wave.rs:1030`.

Failure mode: a corrupt/extreme but finite cell parameter state can produce an
unroutable celerity and the solver can report a partial successful result
instead of `RoutingError::CflViolation` or `RoutingError::NonFiniteState`.

Expected disposition: accepted. Add fail-closed finite/positive celerity guards
and a regression proving non-finite/unsatisfiable CFL cannot return `Ok`.

### High 2 — Case-4 solver and oracle duplicate Iwagaki source authority and diverge at cutoff

Source: Rust correctness review.

The Case-4 D-val solver path redefines Iwagaki slopes, supplies, duration, and
Manning configuration independently from the oracle. The solver forcing uses
`if t > dur` and therefore treats `t == 10.0 s` as still supplied; any solver
step that starts before the cutoff and crosses it applies supply for the whole
step. The oracle uses `t < supply_end_s` and clips steps exactly to
`supply_end_s`. That undermines the D10B "same PDE" acceptance claim.

Evidence:

- Solver-side duplicated Case-4 source setup: `dval.rs:141`.
- Solver cutoff condition: `dval.rs:166`.
- Oracle source authority and cutoff: `iwagaki_oracle.rs:65`,
  `iwagaki_oracle.rs:503`, `iwagaki_oracle.rs:515`.

Failure mode: peak/timing tolerances can still pass while the compared solver
and oracle integrate different lateral-source histories. This is exactly the
kind of source-lineage split D10B was meant to eliminate.

Expected disposition: accepted. Centralize the Iwagaki Case-4 configuration, or
otherwise prove one source of truth. Ensure the solver path clips/splits at the
10 s supply cutoff or uses an equivalent exact source integral, and add a
regression that solver and oracle source totals/duration agree.

### Medium 1 — Terminal negative bin carry can publish a negative outlet bin

Source: Rust correctness review.

`BinRecorder::finish` redistributes negative per-bin flux forward, but if the
final covered bin still carries a deficit it adds the negative carry back into
that same bin. The exported hydrograph then divides that possibly negative bin
by span. Downstream cascade injection will reject negative integrals, but a
terminal single-OFE or terminal cascade outlet can expose negative discharge.

Evidence:

- Negative carry folded into last covered bin: `kinematic_wave.rs:530`.
- Exported hydrograph divides the bin by span: `kinematic_wave.rs:550`.

Failure mode: a terminal outlet can publish negative bin-mean discharge instead
of failing closed or preserving a non-negative exact series.

Expected disposition: accepted. Preserve exact total only when a non-negative
redistribution exists. If a terminal deficit remains, fail closed with a typed
error or add a contract-authorized bounded handling rule plus regression.

### Medium 2 — `cascade_seam_ledger` no longer measures the quadrature terms it names

Source: Rust correctness review.

The diagnostic example still describes `seam_sampling_m3` and
`terminal_quadrature_m3` as sampled-hydrograph quadrature terms, but after D10B
it reads fields that are now solver-ledger outflow. The post-fix all-zero terms
are therefore not evidence about sampled quadrature anymore.

Evidence:

- Example reads `per_ofe_outlet_m3`: `examples/cascade_seam_ledger.rs:129`.
- D10B changed that field to solver-ledger outflow: `cascade.rs:270`.
- Terminal outlet is also ledger mass: `cascade.rs:293`.

Failure mode: the diagnostic artifact can overstate what the seam ledger proves.

Expected disposition: accepted. Either update the example labels/artifacts to
say these are booked-ledger terms, or restore an explicit sampled-quadrature
diagnostic if that evidence is still claimed.

### Medium 3 — `SC-OFEROUTE-001` rev 26 still contains stale GAP-005 blocker authority

Source: contract/authority review.

The contract says the D10 hold is lifted and `GAP-OFEROUTE-005` is resolved,
but other authority rows still say activation is gated on GAP-005 numerics or
that D10 holds Case 4/GAP-005.

Evidence:

- Resolved/lifted note: `SC-OFEROUTE-001.md:252`.
- Stale producer obligation text: `SC-OFEROUTE-001.md:296`.
- Stale BEI text: `SC-OFEROUTE-001.md:423`.

Failure mode: canonical authority is internally inconsistent about whether D15
is blocked by GAP-005.

Expected disposition: accepted. Reconcile every stale GAP-005 blocker statement
to the rev-26 state: GAP-005 resolved; next precondition is D14 endpoint-timing
refresh plus D15's own activation gates.

### Medium 4 — D15 blocker language is stale in planning and roadmap docs

Source: contract/authority review.

Strategy and roadmap text records D10B as complete in one place while still
saying D15 is held by rev-23/GAP-005 in others.

Evidence:

- D10B complete / D14 refresh next: `mofe-fidelity-campaign-strategy.md:365`.
- Stale D15 row: `mofe-fidelity-campaign-strategy.md:370`.
- Stale progress/stop-condition rows: `mofe-fidelity-campaign-strategy.md:401`,
  `mofe-fidelity-campaign-strategy.md:405`.
- Same contradiction in `ROADMAP.md:277`.

Failure mode: queue state is ambiguous and may send the next agent to the wrong
package.

Expected disposition: accepted. State the queue as D14 endpoint-timing refresh,
then D15 rerun; GAP-005 is no longer the blocker after D10B.

### Low 1 — Oracle reanchoring artifact retains superseded acceptance wording

Source: contract/authority review.

`oracle-reanchoring-evidence.md` still says solver error must decrease
monotonically and requires total-variation non-increase. Rev 26 ratifies
non-divergence plus a bounded TV transient.

Evidence:

- Stale wording: `oracle-reanchoring-evidence.md:58`.
- Ratified forms: `SC-OFEROUTE-001.md:370`, `SC-OFEROUTE-001.md:403`.

Expected disposition: accepted. Mark the S2 text as a superseded proposal or
rewrite it to the rev-26 accepted form.

### Low 2 — Bibliography rights status is stale

Source: contract/authority review.

R-102/R-103 still say first-pass rights classification is pending while the
rights log classifies both under `copyrighted/`.

Evidence:

- Bibliography pending text: `references/annotated_bibliography.md:1188`,
  `references/annotated_bibliography.md:1198`.
- Rights log classification: `rights_classification_first_pass_2026-05-11.md:93`.

Expected disposition: accepted. Sync the bibliography rows to the rights log.

### Low 3 — Test/comment wording still overclaims strict TVD

Source: QA review.

The test header says "TVD property: no homogeneous-step total-variation
increase" and the test name says `tv_diminishing`, but the assertion and
contract accept a bounded TV transient.

Evidence:

- Header: `d10b_reconciliation_tests.rs:16`.
- Test name: `d10b_reconciliation_tests.rs:94`.

Expected disposition: accepted. Rename/reword to bounded TV transient.

### Low 4 — Durable regression checks only three of six H2637-class sweep points

Source: QA review.

The package evidence records six H2637-class sweep points, but the checked-in
regression only covers a diagonal subset of three. The one-time gate logs cover
the full grid, so this is not a closure blocker, but the durable guard is
weaker than the evidence surface.

Evidence:

- Regression subset: `d10b_reconciliation_tests.rs:241`.
- Six-point package evidence: `h2637-resolution-evidence.md:32`.

Expected disposition: follow-up or accepted. Prefer expanding the durable test
if runtime cost is acceptable; otherwise record why the full-grid guard remains
artifact-level evidence only.

### Low 5 — Stale comments and artifact counts after final review fixes

Source: local pass + QA review.

Several comments/artifacts still describe pre-D10B or pre-review state.

Evidence:

- `kinematic_wave.rs:9` says TVD primaries are un-acquired and the scheme is
  implemented as stated in R-63.
- `ofe_routing.rs:1` says no phase-span wiring exists, while opt-in shadow
  wiring exists in the runner.
- `line-count-governance-checklist.md:8` has pre-review line counts
  (`kinematic_wave.rs` now 1521 lines, `d10b_reconciliation_tests.rs` now 370).
- `behavior-pinned-test-audit.md:22` cites 61 focused tests; final gate records
  64/64.

Expected disposition: accepted. Update stale text/counts during the revision
pass.

## Readiness Verdict

Design/science direction remains sound: the D10B re-anchor to Iwagaki-primary
physics plus TVD-family authority is a valid way to resolve the clean-room
implementation-parity hold, and no reviewer found the oracle posture itself
unsound.

Implementation readiness is **revision required before re-check**. The CFL
fail-open path and duplicated/divergent Case-4 source authority are blockers
for treating D10B as final closure. Medium/low findings should be cleaned up in
the same response so the package, contract, planning queue, and artifacts all
tell one story before the D14 refresh/D15 rerun proceeds.
