# Review Disposition

Status: `all findings accepted, amended, and independently closed PASS/PASS`.

Evidence class: `Static`.

| Finding | Disposition | Amendment | Closure |
| --- | --- | --- | --- |
| Science 1: first control-volume confound | `accepted` | Package/protocol/lineage now expose effective input fingerprints, membership, active state, projection class, and separate projection/evolution/support estimands. | Pending re-review. |
| Science 2 / Rust 2: nonlinear means | `accepted` | Schema v6 now contains exact duration-tagged substep tuples; hourly summaries are consumer-derived. | Pending re-review. |
| Science 3 / Rust 4: incomplete freeze | `accepted` | Protocol binds fixture/observation/climate hashes, paths, WY windows/censoring, commands, source/binary/trace identities, exact predecessor estimator, and tolerances. | Pending re-review. |
| Science 4: incomplete lineage/naming | `accepted` | Field-level table binds schema names, units, sources, signs, support, roles, all longwave/advected operands, and aerodynamic `z_0,aero`. | Pending re-review. |
| Science 5: underdetermined join/rules | `accepted` | Two-stage join, identity rejection, common-support split, aggregation equations, thresholds, precedence, and term classes are frozen. | Pending re-review. |
| Science 6: N/A/endpoints | `accepted` | Typed reasons/null semantics and per-tuple/hour mass/cold equations and scale-aware tolerances are frozen. | Pending re-review. |
| Rust 1: public API/layout | `accepted` | Four public shapes/calls are protected; companion is boxed/enabled-only and one private solver is required. API, size, allocation, RSS, and parity gates added. | Pending re-review. |
| Rust 3: v5 arithmetic drift | `accepted` | v5 semantics remain unchanged; v6 energy and evaluated-support mean names are distinct. | Pending re-review. |
| Rust 5: real consumer absent | `accepted` | Package-local analyzer/test paths and fail-closed responsibilities are named and scaffolded; implementation follows the contract gate. | Pending re-review. |
| Rust 6: negative proof/tests | `accepted` | Whole-runtime allowlist, failure isolation, WAT/HBP/PASS parity, schema versions, solver vectors, covariance, support, closure, and allocation tests are explicit exit gates. | Pending re-review. |
| Rust 7: governance/write set | `accepted` | Exact exports/runner/analyzer/assurance paths added; line baselines freeze a no-growth rule for 2,923-line `00c`; readiness and reading maps expanded. | Pending re-review. |

## Second-review Residual Disposition

| Finding | Disposition | Amendment | Closure |
| --- | --- | --- | --- |
| Unlike legacy/comparable sign estimands | `accepted` | Frozen exact external + active-conduction legacy bridge; historic value is custody-only; state-evolution class now requires sequential external positive versus same-state external negative; added legacy-estimand class. | Pending third review. |
| Total cold closure omits lower cancellation | `accepted` | Added active and lower cold changes and total equation; nonzero-conduction anti-alias test required. | Pending third review. |
| Wrong albedo fallback | `accepted` | Bound `STAGE3_DEFAULT_SNOW_ALBEDO = 0.82`; absent-state/source-symbol anti-alias test required. | Pending third review. |
| Geometry hash and hourly-bin units | `accepted` | Listed exact forcing/geometry ordered operands and corrected energy/depth bin units. | Pending third review. |
| Omitted-support materiality underdefined | `accepted` | Frozen both-operator/five-term omitted intervals, absolute-value placement, denominator, and zero-denominator behavior. | Pending third review. |
| Turbulent termination branches | `accepted` | Frozen seven-status iteration/length/correction taxonomy and branch-parity tests. | Pending third review. |
| Post-melt after-surface applicability | `accepted` | Added `post_substep_no_resolved_surface`; active after fields become null while total empty-state fields remain applicable. | Pending third review. |
| Frozen-active reference underdefined | `accepted` | Frozen daily reset, first sequential active projection, fixed/varying operands, no-state/no-conduction equations, support, and aggregation. | Pending third review. |
| Nonexistent assurance paths | `accepted` | Replaced with exact typed-v2 report, lock, identity, and transaction path families. | Pending third review. |

## Third-review Residual Disposition

| Finding | Disposition | Amendment | Closure |
| --- | --- | --- | --- |
| Ordered causal crossing underconstrained | `accepted` | Added explicit three-way common support and distinct same-state-to-frozen projection crossing versus frozen-to-sequential evolution crossing predicates; zero-band/nonunique cases are multifactor. | Pending fourth review. |
| Frozen reference omitted albedo | `accepted` | Selected daily albedo and state-versus-`0.82` fallback lineage are fixed and included in reference custody. | Pending fourth review. |
| Ordered delta formulas absent | `accepted` | Defined `S/F/Q`, `delta_projection=F-S`, `delta_evolution=Q-F`, identical three-way support, WY reduction, and exact sign predicates. | Pending fourth review. |
| Status-to-stability mapping absent | `accepted` | Bound exact class, iteration, length, retained-correction semantics for every solver termination; mixed/nonfinite zero-buoyancy correction state fails lineage. | Pending fourth review. |
| Reference fields/options categorical | `accepted` | Enumerated exact fixed, varying-hourly, and identity-only fields and required exact geometry/solver/selector equality before construction. | Pending fourth review. |

## Fourth-review Residual Disposition

| Finding | Disposition | Amendment | Closure |
| --- | --- | --- | --- |
| Projection class can fire without real sequential reversal | `accepted` | Added `Q > +tol`, exact legacy-bridge reproduction, reconstruction/delta closure, and invariant-shortwave requirements. | Pending fifth review. |
| Albedo source lineage opaque | `accepted` | Added explicit source ID, nullable model ID, and nullable accumulated-positive-temperature state field; all are fixed in reference custody and covered by anti-alias tests. | Pending fifth review. |

Fifth independent admission review closed every row above at exact clean
`42a32a297383eb4e624abc0038a073238a5d0a92`: science `PASS`, Rust/custody
`PASS`. No result-bearing evidence was inspected.

No finding was rejected, deferred, or moved to follow-up. The result-blind
admission gate is complete. Contract-first implementation may begin; result
execution remains blocked until the later implementation and consumer review
gates pass.

## Implementation Re-Admission Disposition

The first implementation review at `f5202ee65f8b40d2c0244d92fd5c2843077e9997`
and its follow-up reviews remained result-blind. All findings were accepted.
Rust reconstruction, applicability, ordering, mutation, protected-shape, and
solver-taxonomy findings were amended at `ee86ac6a872ee00c925bc391175df712880db734`.
The final Rust residual—missing exact contract enumeration of
`longwave_model_id` and `sublimation_model_id`—was amended and independently
closed PASS at `80b653ce9d343ce37b5949c3a9e853bc08315ac0`.

Science and consumer review then found accepting malformed evidence,
producer-total aggregation, incorrect support reduction, incomplete causal
gates, incomplete status/tail/state custody, incomplete inventories, and weak
retained-source custody. All are accepted and amended in the package-local
consumer with adversarial tests. No cohort result was run or inspected.
Science and consumer closure remain pending exact-commit result-blind
re-review; result execution remains blocked.

Follow-up re-review also found a real row/receipt boundary mismatch plus
permissive requested-duration, sublimation-selector, and zero-wind geometry/
options custody. All are accepted: `site_id` is now explicitly receipt/path-
bound rather than fabricated as a snow-row field, and the consumer validates
the exact runtime duration, selector, and pre-solver domains, including vapor
pressure below atmospheric pressure. Adversarial coverage is `34/34`; result
execution remains blocked pending re-review.

The last result-blind residual—independent longwave reconstruction accepting
producer-impossible daylight/radiation/canopy domains—is accepted and amended
with exact selected-solver gates and adversarial coverage. The model-free
consumer suite is now `40/40`; cohort execution remains blocked until the
exact-commit science and consumer verdicts pass.

## Inactive-Lifecycle Re-Admission Disposition

The first corrected-lifecycle re-review at exact clean
`4b0960426c694dfe4a84c58b4d4ba4c9433fe96b` remained result-blind and returned
HOLD from science, Rust, and consumer review. Every finding is accepted:

- the package write set and artifact routing named only the rejected v1 target
  namespace even though the freeze and analyzer correctly selected v2;
- the analyzer accepted empty records with nonzero/mixed/missing sentinel
  identities or noncanonical hourly reasons and did not consume the serialized
  inactive support fields;
- the Rust test exercised the inactive constructor but not the repaired control
  flow before typed hourly-forcing acquisition; and
- no reduction-level vector proved that a paired inactive day cannot enter the
  operator estimands.

The package and artifact routing now distinguish read-only v1 custody from the
authorized prospective v2 namespace. The consumer enforces the exact four-zero
sentinel form, 24 `operator_not_selected` statuses, daily/hourly support, and
carrier-evaluated flags with adversarial aliases. The real site reducer proves
the inactive day changes only its typed inventory entries, and a source-order
contract seam binds the inactive return ahead of forcing acquisition for every
operator/default. Independent science/Rust/consumer re-review closed PASS at
exact clean `e591d89c219d69f619e68f9aa7194f88d20f9a1c` before v2 execution.
Corrected v2 subsequently failed before results on the separate sequential
transition-continuity defect recorded below.

## Sequential-Continuity Re-Admission Disposition

The first v3 Rust review at exact clean
`3fbcdfddeff1b25f3e9384120b04857af66d96da` found two validation seams while
science and consumer review passed: the positive continuity vector crossed
hours but did not reproduce v2's same-hour dynamic-substep failure or the real
JSON formatter, and resolved status was computed before a preparation whose
floating-point redistribution supplies the serialized endpoint. Both findings
are accepted. The producer now recomputes resolved status after preparation,
and the runner test forces same-hour substeps 0/1 through actual schema-v6 JSON
serialization and bit-exact consumer fields. Renewed exact-head review remains
required before v3 execution.

## Post-Result Finding Disposition

| Finding | Disposition | Correction | Result impact |
| --- | --- | --- | --- |
| Legacy-sign class incorrectly required predecessor reproduction | `accepted` | Removed `predecessor_ok` only from the class-3 predicate and added ordered coexistence coverage for predecessor failure plus legacy-positive/external-nonpositive energy. | None. Retained Snowbird external `Q_all` is positive, so the predicate remains false and the two retained classes are unchanged. |

The corrected consumer suite passes `51/51`. Independent science and consumer
re-review both passed at exact clean `f70fb0cb258af0daffc08f49024d46c4076ae749`.
No finding was rejected or deferred, and the retained result did not change.
