# MOFEFID-D10 - Shock Numerics GAP-005 Closure

Status: **EXECUTED-HOLD-SOURCE-AUTHORITY** (executed 2026-07-05). Campaign:
[MOFEFID](../../planning/mofe-fidelity-campaign-strategy.md) Lane D.
Contract focus: `SC-OFEROUTE-001#GAP-OFEROUTE-005` and the Case-4 residual
of `SC-OFEROUTE-001#INV-OFEROUTE-011`.

## Objective

Close defect `GAP-OFEROUTE-005` end to end. D10 must resolve the Lane D
shock/resolution numerical-method blocker left by D8/D9: Iwagaki Case 4
shock fidelity remains resolution-sensitive, and the real H2637 runtime
shadow vector showed run-level conservation diagnostics that are
resolution-sensitive and non-monotone under the current sampled-handoff
configuration.

The package must either land a contract-first, source-authorized correction
inside the Lane D numerical-method envelope, or close in an explicitly owned
`HOLD` with primary/source evidence proving why the correction cannot be
authorized or implemented inside D10. It must not relay another diagnostic-only
step.

## Rationale

D9 closed the non-numerics D-val surface: Cases 1-3 retain named
dispositions, Zone 1/Zone 2 taxonomy is executed, and Case 4 is the only
remaining `INV-OFEROUTE-011` blocker. `GAP-OFEROUTE-005` is now the first
actionable Lane D production-activation blocker. Active routing cannot use
Case 4 or H2637 conservation diagnostics as acceptance while the current
solver/handoff evidence is known to be resolution-sensitive without a named
numerical authority, convergence criterion, or hold boundary.

This package is therefore a Defect-Closure ExecPlan: it owns the numerical
method defect class, not production activation.

## Correction Authority Envelope

### Defect IDs and Observed Violations

- `GAP-OFEROUTE-005`: Iwagaki shock sampled-hydrograph / resolution
  sensitivity.
- `INV-OFEROUTE-011` Case-4 residual only: D-val shock acceptance remains open.
- H2637 shadow reproduction: the Lane D runtime shadow found run-level cascade
  conservation diagnostics that vary materially with `(sample_dt, max_dt)` and
  are non-monotone on the steep 19-OFE fixture.

### In-Scope Authority

- `SC-OFEROUTE-001` numerical-method, D-val, and `GAP-OFEROUTE-005` rows.
- Papanicolaou 2018 Lane D equations and supplemental-derived Case-4 data.
- Iwagaki 1955 Case-4 shock primary/source evidence.
- TVD / kinematic-wave numerical-method source authority already in the repo
  or acquired under copyright governance.
- Current Lane D solver/cascade implementation surfaces:
  `ofe_routing::kinematic_wave`, `ofe_routing::cascade`,
  `ofe_routing::infiltration`, `ofe_routing::dval`, the D-val example, and
  D-val harness scripts/tests.

### Authorized Edit Classes

Within this package, the executor may:

- Amend `SC-OFEROUTE-001` to record numerical-method authority, convergence
  criteria, Case-4 acceptance/hold text, tolerance governance, and
  `GAP-OFEROUTE-005` disposition.
- Add or modify contract-derived tests and D-val harness checks for Case 4,
  convergence sweeps, and H2637 shadow resolution reproduction.
- Correct the pure Lane D solver/cascade numerical implementation if all
  seven DC gates are met and the correction is source-authorized.
- Add diagnostics needed to measure the defect, provided they do not become
  surrogate acceptance surfaces.

### Protected Boundaries

- No production/default activation and no D14 opt-in flip.
- No `OPENWEPP_LANED_SHADOW` activation semantics change beyond diagnostic
  resolution-sweep instrumentation if needed.
- No D11 friction operand sourcing/default authorization.
- No D12 melt-limb hourly-source implementation.
- No D13 ADR-0036 erosion hourly-shape switch.
- No watershed/channel routing changes.
- No tuned, surrogate, proxy, heuristic, or empirical stand-in physics or
  numerics in production paths.

## Scope

### Included

- Primary/source authority audit for the TVD-MacCormack / shock-capture method
  used by Lane D, including limiter form, CFL/substep constraints, and any
  relevant Case-4-specific expectations.
- Iwagaki Case-4 evidence with named tolerances for `NS_trace`, peak ratio,
  sampled `t_peak`, rise behavior, and resolution sweep behavior.
- Convergence criteria over cell count, sample interval, and max sub-step.
- H2637 real-hillslope resolution-sensitivity reproduction carried into the
  same numerical-method verdict.
- Contract-first amendment and contract-derived tests when a correction or
  hold disposition is determined.
- Dual review, disposition, dual verification, line-count governance, and
  final package status.

### Excluded

- Case 1, Case 2, Case 3, and Zone 1/Zone 2 taxonomy adjudication; D9 closed
  those surfaces.
- D11 friction operand authority (`k_o`, `I`, roughness elements,
  vegetation/residue operands).
- D12 melt-limb source coverage.
- D13 erosion hourly-shape consumer switch.
- D14 active routed-water publication and DC01-disable wiring.
- D15 default-promotion policy.

## Dependencies

- `SC-OFEROUTE-001` rev 17.
- D9 handoff:
  `docs/work-packages/20260705-mofefid-d9-dval-disposition-001/artifacts/case4-d10-handoff.md`.
- D8 execution report:
  `docs/work-packages/20260702-mofefid-d8-routing-fidelity-defect-closure-001/artifacts/execution-report.md`.
- Lane D activation increment:
  `docs/work-packages/20260705-mofefid-laned-activation-increment-001/`.
- MOFEFID strategy §6.1 D10-D15 sequence.
- Copyright-governed references already in repo:
  `references/copyrighted/Iwagaki1955_runoff_characteristics_DPRI10.pdf`,
  `references/copyrighted/Papanicolaou2018.md`,
  `references/copyrighted/Papanicolaou2018-supplemental/`, and
  TVD/kinematic-wave references such as `references/copyrighted/mingham2001.pdf`.

## Intended Write Set

Primary:

- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/work-packages/20260705-mofefid-d10-shock-numerics-gap005-001/`
- `docs/work-packages/README.md`
- `docs/planning/mofe-fidelity-campaign-strategy.md` only if D10 status or
  downstream sequencing text changes.

Conditional, only if the D10 evidence requires implementation or harness
changes:

- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/infiltration.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/dval.rs`
- `crates/openwepp-hillslope-orchestrator/examples/dval_case.rs`
- `tools/dval/compare_dval.py`
- D-val, Lane D, or H2637 tests/fixtures directly needed to prove the
  correction or hold.

Protected:

- Production routed-water activation files except read-only inspection or
  resolution diagnostics that do not change default/shadow behavior.
- HBP/pass/watershed output schemas.
- Raw copyrighted workbook/source additions beyond existing approved fixture
  governance.

## Phase Plan

1. **D10-S0 - Intake and baseline.** Read core/conditional authority,
   reproduce the D8 Case-4 baseline and the H2637 resolution-sensitivity
   reproduction, and record exact pre-change metrics.
2. **D10-S1 - Source authority.** Audit primary/source authority for the
   shock-capture numerical method: limiter, CFL/substep, interpolation,
   boundary/source handling, and expected convergence behavior. Decide whether
   D10 has authority to correct the solver/cascade in production.
3. **D10-S2 - Contract-first decision.** Amend or confirm `SC-OFEROUTE-001`
   before production edits. Record either the source-backed correction rule and
   tolerances, or a hold-for-authority boundary with a named blocker.
4. **D10-S3 - Contract-derived tests and pre-implementation gate.** Add tests
   and harness checks that fail under the observed defect or make the hold
   executable. Record the pre-implementation gate.
5. **D10-S4 - Correction or legitimate HOLD.** If the seven DC gates are met,
   implement the numerical correction in the pure Lane D solver/cascade and
   validate Case 4 plus H2637. If not, stop only after a hold-legitimacy audit
   proves the boundary.
6. **D10-S5 - Evidence and closure.** Run required gates, update artifacts,
   complete dual review and verification, disposition findings, and set final
   package status.

## Exit Criteria

- `GAP-OFEROUTE-005` is either closed by a contract-backed correction with
  evidence, or held with primary/source evidence and a first actionable
  blocker that is not another vague diagnostic step.
- `INV-OFEROUTE-011` Case 4 no longer has ambiguous status: the contract states
  whether Case 4 satisfies D-val, remains held by a named numerical authority
  boundary, or is superseded by a revised acceptance criterion.
- Iwagaki Case-4 evidence records `NS_trace`, peak ratio, sampled `t_peak`,
  rise behavior, and resolution sweep behavior.
- H2637 resolution-sensitivity reproduction is adjudicated under the same
  numerical-method verdict.
- Any production correction preserves fail-closed guards, mass/conservation
  accounting, and typed numerical-domain validation.
- No production/default activation, D11-D13 scope, or surrogate numerics are
  added.
- Dual review findings are dispositioned as `accepted`, `rejected`,
  `deferred`, or `follow-up`; accepted findings are fixed before closure.
- Line-count governance is recorded for every touched `.rs` file.

## Required Gates

Selection follows `docs/standards/local-ci-gate-selection.md` where relevant,
but D10 cannot close without recording:

- `git diff --check`
- Markdown lint for touched docs.
- Contract/profile/BEI checks for changed `SC-OFEROUTE-001` surfaces.
- D-val Case-4 commands and resolution sweeps.
- H2637 shadow resolution reproduction or an equivalent targeted fixture gate.
- Focused Rust tests for touched `ofe_routing` / D-val surfaces.
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace --profile full`
- `cargo deny check`
- Source-level anti-evasion guards if required-case bindings, D-val fixtures,
  or authority-suite posture are touched.

If heavy gates are delegated, record subagent output and log paths in
`artifacts/gate-results.md`.

## Conservation / Output Acceptance

D10 changes or adjudicates a conservation-sensitive water-routing numerical
method. Before any production solver/cascade edit, record an operand-lineage
table covering source terms, boundary inflows, outlet hydrographs, storage,
sample interval, substep controls, units, and authoritative vs diagnostic
status. Acceptance cannot rest only on exact self-consistency or one-sided
bounds: it must include independent mass/closure reconstruction, rejected alias
formulas, Case-4 trace metrics, and H2637 resolution evidence.

## DC Conversion Rule

If D10 establishes a reproducible root cause inside the declared envelope, and
the expected behavior is supported by `SC-OFEROUTE-001`, primary/source
numerics authority, pinned-baseline provenance where applicable, or a
contract-authorized physical/numerical invariant, then D10 must proceed through
contract amendment, contract-derived tests, pre-implementation gate,
production correction, validation, review, and disposition. It may not close as
`HOLD` because more investigation is possible.

The seven gates are:

1. reproduction,
2. named mechanism,
3. ownership,
4. authority,
5. safety,
6. testability,
7. validation.

## HOLD Legitimacy

D10 may close in `HOLD` only if it proves one of these boundaries:

- missing or contradictory primary/source numerical authority,
- an operand-bound issue that belongs to D11 rather than the numerical method,
- a melt/source-shape issue that belongs to D12,
- an erosion-shape issue that belongs to D13,
- a production-activation wiring issue that belongs to D14.

A hold must name the boundary, cite evidence, list the in-envelope correction
route considered, and explain why that route cannot close `GAP-OFEROUTE-005`
inside D10.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes
spawning/delegating to `rust_code_reviewer`, `rust_qa_reviewer`, `explorer`,
and `comparator_suite_runner` subagents for read-only review, verification,
source/harness inspection, primary/source-evidence review, and heavy
D-val/full-gate execution. Expected outputs are compact findings, gate metrics,
log paths, and package-local review or verification artifact text. Write
access is read-only unless a later operator explicitly assigns a bounded write
set.

Subagent requirement: `comparator_suite_runner` is REQUIRED for heavy
Case-4/H2637 sweeps, full workspace nextest, and other heavy closure gates
when available. Do not run those heavy batches on the parent model unless the
subagent is unavailable; if unavailable, record command-level evidence and run
locally only when package governance permits substitution.

## Required Artifacts

- `artifacts/required-reading-map.md`
- `artifacts/correction-authority-envelope.md`
- `artifacts/source-authority-evidence.md`
- `artifacts/contract-implementation-evidence.md`
- `artifacts/contract-test-implementation-evidence.md`
- `artifacts/pre-implementation-contract-gate.md`
- `artifacts/iwagaki-case4-evidence.md`
- `artifacts/h2637-resolution-evidence.md`
- `artifacts/numerics-convergence-evidence.md`
- `artifacts/conservation-output-lineage.md`
- `artifacts/implementation-test-evidence.md`
- `artifacts/hold-legitimacy-audit.md`
- `artifacts/kernel-profile-compliance-checklist.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/gate-results.md`
- `artifacts/line-count-governance-checklist.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`
- `artifacts/worker-handoff.md`
- `artifacts/disposition.md`

## Progress

- [x] 2026-07-05: Package scaffolded from D9 handoff and MOFEFID §6.1 D10 row.
- [x] D10-S0 intake and baseline.
- [x] D10-S1 source authority.
- [x] D10-S2 contract-first decision.
- [x] D10-S3 contract-derived tests and pre-implementation gate.
- [x] D10-S4 correction or legitimate HOLD.
- [x] D10-S5 evidence, review, verification, and closure.

## Surprises & Discoveries

- Garcia-Navarro 1992 and Mingham 2001 are now primary-in-hand, so
  `GAP-OFEROUTE-001`'s acquisition premise was stale, but the primaries do not
  close the reduced-KWE/OFE-handoff production-correction authority gap.
- Iwagaki 1955 primary names Manning `n=0.009`; D-val Case 4 uses `k_o`.
  D10 rejected converting this into a `k_o` tuning/default route because D11
  owns friction operand authority.
- A source-shaped limiter-branch correction trial worsened Case 4 and failed
  focused `ofe_routing` tests; it was reverted and not carried into the final
  diff.
- H2637 reproduces the production-shaped resolution sensitivity, but the
  shadow block is diagnostics-only and has no D10 parameterized resolution
  sweep CLI.

## Decision Log

- Decision: D10 is a Defect-Closure package for `GAP-OFEROUTE-005`, not a
  general Lane D activation package.
  Rationale: D9 narrowed `INV-OFEROUTE-011` to Case 4 shock numerics, while
  D11-D14 own distinct activation blockers.
  Date/Author: 2026-07-05 / Codex.
- Decision: Close D10 as `EXECUTED-HOLD-SOURCE-AUTHORITY`, not as a production
  solver/cascade correction.
  Rationale: reproduction, ownership, and testability were established, but
  the authority and safety gates failed: the available primaries do not bind
  the reduced Papanicolaou KWE limiter/handoff/Iwagaki operand mapping, and a
  source-shaped limiter trial regressed tests.
  Date/Author: 2026-07-05 / Codex.
- Decision: Add Case-4-only resolution controls to the D-val comparator harness.
  Rationale: D10 needs executable cell/sample/substep diagnostics while keeping
  Cases 1-3 out of scope.
  Date/Author: 2026-07-05 / Codex.

## Outcomes & Retrospective

D10 executed the Case-4 and H2637 evidence surfaces, amended
`SC-OFEROUTE-001` to revision 18, added Case-4-only D-val resolution controls,
and recorded a legitimate source-authority HOLD. Case 4 remains the only open
`INV-OFEROUTE-011` D-val surface, and it remains non-acceptance evidence until
a follow-on authority reconciliation binds limiter/CFL/dissipation,
lateral-source/boundary handoff, and Iwagaki friction mapping into named
tolerances. No production/default activation or D11-D13 work was performed.
