# MOFEFID-D8 — Routing Fidelity Defect Closure

Status: **EXECUTED — APPROVED / MERGE-READY** (2026-07-02)
Campaign: [MOFEFID](../../planning/mofe-fidelity-campaign-strategy.md) Lane D.
Contract: `SC-OFEROUTE-001` rev 8 baseline (`INV-OFEROUTE-002`,
`INV-OFEROUTE-003`, `INV-OFEROUTE-004`, `INV-OFEROUTE-010`,
`INV-OFEROUTE-011`, `GAP-OFEROUTE-002`, withdrawn `GAP-OFEROUTE-004`).
Owner: Codex. Package shape: Defect-Closure ExecPlan (`DC`).
Activation: **shadow-first only**; no production phase wiring.

## Objective

Close the four implementation discrepancies surfaced by MOFEFID-D7 D-val:

- **D8-1** — skin `I`/`nu` unit-convention audit
  (`INV-OFEROUTE-002`, `GAP-OFEROUTE-002`).
- **D8-2** — Iwagaki peak noise and internal-vs-sampled `t_peak`
  disagreement.
- **D8-3** — Cases 2-3 peak under-prediction.
- **D8-4** — Case 1 rising-limb lag.

Each item closes in this package as exactly one of:
`corrected`, `operand-limited (no defect)`, or `declared boundary`. No item may
end as a vague gap or a request for another trace.

## Correction Authority Envelope

Defects / symptoms:

| ID | Observed D7 symptom | Current status |
|---|---|---|
| D8-1 | `f_s = (3393 I^0.407 + k_o)/Re` convention unconfirmed; Cases 1-3 are `k_o` dominated and D7 mis-exercised `I` in Iwagaki. | Open audit |
| D8-2 | Iwagaki peak non-monotonic in `k_o`; `RoutingResult::time_to_peak_s` differs from sampled hydrograph peak by about 9 s. | Open metric/numerics diagnosis |
| D8-3 | Case 2 peak about 25% low; Case 3 peak about 45% low with S0 magnitude caveat. | Open operand/friction diagnosis |
| D8-4 | Case 1 steady magnitude reproduces, but 10-90% rise is about 5000 s vs enhanced about 3580 s. | Open transient attribution |

In-scope write set:

- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`.
- `docs/work-packages/20260702-mofefid-d8-routing-fidelity-defect-closure-001/`.
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/friction.rs`.
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs`.
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/infiltration.rs`.
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs`.
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/dval.rs`.
- `crates/openwepp-hillslope-orchestrator/examples/dval_case.rs`.
- `tools/dval/compare_dval.py`.
- Focused tests in the same modules.

Allowed edit classes:

- Contract-first clarifications and gap dispositions in `SC-OFEROUTE-001`.
- Kernel corrections that implement existing R-63 / SC-OFEROUTE-001 authority
  without surrogate physics.
- D-val harness extensions that expose diagnostics or compare derived scalar
  metrics without vendoring copyrighted series.
- Contract-derived regression tests using cited scalars, equation-level
  operands, or synthetic low-`k_o` vectors.

Protected boundaries:

- No production runtime activation or direct-runtime wiring; default hillslope
  path must remain byte-flat by construction (`INV-OFEROUTE-010`).
- No forced reproduction against enhanced-WEPP; ADR-0017 applies. Enhanced-WEPP
  is an investigation flag / method-fidelity trace, not an oracle.
- No vendoring of copyrighted workbook rows or hydrograph series.
- No primary-source claim for Shen & Li, Hirsch, or Woolhiser unless the
  primary is actually read and cited in this package. R-63 secondary authority
  may close the local implementation convention only to the extent it states
  the equation and units.

## Mandatory Diagnosis Rule

Every item starts with a forcing/operand/unit audit before solver or kernel
attribution. A suspiciously clean, suspiciously round, or regime-specific
discrepancy is treated first as a forcing/unit smell. A solver/friction defect
may be declared only after the audited experiment confirms:

- forcing channel (`rainfall_excess`, `rainfall_intensity`, upstream inflow);
- geometry and width/length basis;
- units and conversion helper;
- soil / friction operands and uncertainty class;
- comparison window and metric definition.

## Conversion Rule

If this package establishes a reproducible root cause inside the declared
envelope, and the expected behavior is supported by `SC-OFEROUTE-001`, R-63 as
contract authority, or a contract-authorized physical invariant, then this
package must amend the contract, add or update tests, implement the correction,
and validate. It may not close as `HOLD` because more investigation is possible.

## Seven-Gate Bar

For each D8 item:

1. **Reproduction** — reproduce the D7 symptom or statically tie it to a named
   D-val fixture.
2. **Mechanism** — reduce it to a named mechanism, not another variable to
   inspect.
3. **Ownership** — classify whether the mechanism is inside this envelope.
4. **Authority** — identify the `SC-OFEROUTE-001` / R-63 / physical invariant
   authority for the expected behavior, or declare the missing authority.
5. **Safety** — no silent clamp, invalid-input normalization, surrogate
   physics, or production wiring.
6. **Testability** — add a regression or cited-scalar check if the mechanism is
   in-envelope.
7. **Validation** — rerun the D-val harness or focused diagnostic that measures
   the symptom after correction/disposition.

## Milestones

- **D8-S0 — Scaffold and baseline reproduction.** Create this package, record
  baseline D7 scalar metrics, confirm local copyrighted source hashes, and
  verify shadow-first call boundaries.
- **D8-S1 — D8-1 skin audit.** Exercise the Shen & Li `I` term in a low-`k_o`
  rain-driven regime, cross-check against R-63's stated equation and units, and
  update `INV-OFEROUTE-002` / `GAP-OFEROUTE-002` to the true status.
- **D8-S2 — D8-2 Iwagaki metrics.** Reconcile internal and sampled `t_peak`,
  characterize peak `k_o` noise, and either correct the metric or document the
  intended metric and bounded shock-capture property.
- **D8-S3 — D8-3 Case 2/3 under-prediction.** Audit operands/forcing first;
  then check form, wave, and vegetation formulas against R-63. Classify as
  corrected, operand-limited, or boundary.
- **D8-S4 — D8-4 Case 1 rising limb.** Attribute the transient lag among
  Green-Ampt ponding, routing celerity, and infiltration->routing coupling.
  Correct or declare a boundary with evidence.
- **D8-S5 — Contract/code/tests.** Apply necessary contract amendments and
  kernel/harness corrections with focused tests.
- **D8-S6 — Validation and closure.** Rerun D-val diagnostics, required gates,
  shadow-first grep, BEI/authority guards, full workspace nextest, and cargo
  deny; complete artifacts and review-ready disposition.

## Per-Item Closure Verdicts

| Item | Verdict | Evidence |
|---|---|---|
| D8-1 | corrected | Low-`k_o` SI `I` regression against R-63; negative `I` no longer silently clamped. |
| D8-2 | corrected metric + declared boundary | Sampler interpolates within solver steps; Case 4 shock peak/timing remains resolution-sensitive (`GAP-OFEROUTE-005`). |
| D8-3 | Case 2 operand-limited; Case 3 declared boundary | Case 2 closes under `Ks` sensitivity; Case 3 enhanced trace exceeds recorded rainfall-length ceiling. |
| D8-4 | operand-limited | Routing-only rise is fast; slow limb is Green-Ampt operand sensitivity. |

## Required Artifacts

- `artifacts/baseline-diagnostics.md` — baseline D7 metrics and source hashes.
- `artifacts/forcing-operand-audit.md` — per-item forcing, operand, and unit
  audit before attribution.
- `artifacts/execution-report.md` — per-item mechanism and closure verdicts.
- `artifacts/gate-log.md` — commands run and results.
- `artifacts/review-codex.md` — executor self-review.
- `artifacts/review-claude.md` — independent review, approved with two
  non-blocking traceability findings.
- `artifacts/review-disposition.md` — accepted/closed CL-D8-1 and CL-D8-2 with
  doc-only package amendments.

Subagent authorization: this package explicitly authorizes spawning/delegating
to review and verification subagents for read-only review and bounded
package-artifact review outputs. Expected outputs are package-local review and
verification artifacts; production/test write access remains with the executor.

## Exit Criteria

- Each D8 item has a closure verdict: `corrected`, `operand-limited (no
  defect)`, or `declared boundary`, with `Static:` and/or `Ran:` evidence.
- Any in-envelope correction has contract text, regression tests, and D-val
  validation evidence.
- `SC-OFEROUTE-001` has revision entries for every contract edit and no stale
  active text contradicts the dispositions.
- Copyright governance preserved: no workbook rows or full hydrograph series
  committed; offline metrics only cite local source paths and sha256.
- Shadow-first preserved: no `ofe_routing` calls from `direct_runtime/` or
  `openwepp-runner/` production paths.
- Gates run and recorded:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo nextest run -p openwepp-hillslope-orchestrator`
  - `cargo nextest run --workspace --profile full`
  - `cargo deny check`
  - BEI lint (PASS-DEFERRED acceptable for unpromoted routing bindings)
  - `bash tools/release/check_authority_suite_antievasion.sh`
  - `cargo nextest run --test auth11_required_suite_obligation_guards_contract`
  - `git diff --check`

## HOLD Legitimacy Conditions

A declared boundary is legitimate only if the package records:

- the missing or contradictory authority;
- the source evidence proving the boundary;
- the in-envelope correction route considered;
- why that route would be surrogate, tuned, or unsupported here;
- the next defect-shaped closure target.

## Progress

- 2026-07-02: Package scaffolded from D7 rev 8 baseline after local `main` was
  fast-forwarded to `origin/worktree-mofefid-d7` because `origin/main` had not
  yet advanced past D6 in this checkout.
- 2026-07-02: D8 code/contract corrections landed in the working tree; focused
  `ofe_routing` tests passed 43/43.
- 2026-07-02: Required gates passed, including full workspace nextest
  1249/1249 (1 skipped).
- 2026-07-02: Independent review approved D8 as merge-ready with two
  non-blocking traceability findings; both were dispositioned by doc-only
  package updates.

## Surprises & Discoveries

- D8-2 changed the D7 Case 4 story materially: after correcting sample-time
  attribution, Case 4 no longer has near-reference timing/rise at `k_o=200`.
  The previous "timing/rise reproduce" status was a sampling artifact layered
  on top of the earlier forcing bug.
- The sampler correction is broader than Case 4: every shadow routing sampled
  outlet hydrograph now uses sample-time interpolation, including D4/D5/D6
  validation surfaces and cascade handoff interpolation. Conservation and CFL
  evidence remain solver-internal and unaffected.
- A simple increase of the implicit `alpha` fixed-point iteration count was
  considered for Case 4 but rejected: it changed steady/cascade/conservation
  behavior and broke existing tests, so it is not a safe contract-backed D8
  correction.

## Decision Log

- D8-1 closes only the local SI `I` convention against R-63 and regression
  tests. It does not claim a primary Shen & Li / Hirsch / Woolhiser audit.
- D8-2 opens `GAP-OFEROUTE-005` instead of reviving D7's withdrawn
  `GAP-OFEROUTE-004`; the D7 forcing-bug attribution remains withdrawn.
- D8-3 Case 2 is operand-limited rather than a friction defect because plausible
  `Ks` moves `NS_trace` to 0.961 without kernel changes.
- D8-4 is operand-limited because routing-only response is fast; the slow limb
  enters through Green-Ampt operand uncertainty.

## Outcomes & Retrospective

- D8 executed end-to-end and is approved / merge-ready. All four discrepancies
  have closure verdicts, `SC-OFEROUTE-001` rev 9 records the authority changes,
  and the routing subsystem remains shadow-first with no production wiring.
