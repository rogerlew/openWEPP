# Codex Post-Hoc Review Disposition

Status: `EXECUTED`

Evidence mode: `Static` (disposition + amendment authoring) + `Ran`
(independent re-verification of both H2 terminal claims against pinned
`dcap.for:160-262` before binding them into contract text; post-fix
binding-exposure lint: `PASS ... 7 binding exposure row(s) fully
consolidated`).

Review dispositioned: `codex_posthoc_review.md` (Codex, 2026-07-10,
recommendation `REOPEN` for `WSHED-W11A-POSTHOC-001`). All amendments below
are in `SC-ROUTE-001` v52 (revision row records the mapping).

| finding_id | severity | decision | action_taken | rationale |
|---|---|---|---|---|
| H1 (hydraulic operands under-determined) | High | accepted | Unique hydraulic-profile operand map bound in `INV-ROUTE-016` and the addendum operand table: `qe(it) := q1(it)` (outlet anchor — the legacy `qe = peakot` slot interval-ized), `qt(it) := qin(it)`, `qlat(it) :=` the wave-routing lateral series (all three are already-published WS11 state symbols); event-peak fractions and event-duration rates declared invalid on the lane; routed storage change deliberately unreconciled (outlet-anchored steadiness per interval — the exact legacy event-scale posture and the quasi-unsteady class posture), with the reasoning stated in-invariant. Anti-alias **vector 11** added with nonzero storage change so the four candidate constructions diverge. Vector 1 pins the profile operands. | Codex is right that the omission forced an executor choice. The ratified map is the legacy operand map evaluated per interval with no new physics: legacy anchors the steady profile at the routed outlet (`peakot`) while inflow partitions set the split; the interval-ization substitutes the grid-native routed series for each slot. |
| H2 (migrated terminals diverge from pinned baseline) | High | accepted | `INV-ROUTE-018` now binds **pinned `dcap.for` behavior as the realization** with the migrated lanes as implementation target, names both divergent terminals (capped-widening geometry-from-capped-erosion, `dcap.for:238-261`; subcritical-boundary-shear `timsh = timpot` re-entry with `depmid` decrement, `dcap.for:210-215, 173-190`), and mandates their correction before interval reuse. New `GAP-ROUTE-014` (open) records the divergence, the locked-in lane tests, the W11 Phase B correction obligation, and the event-scalar-lane Investigation-tier parity flag. Vector 10 extended with (b) capped and (c) subcritical terminals, each with independent geometry-mass reconstruction. Both terminal claims re-verified against the pinned source before binding (`Ran`). | The contract cannot simultaneously mandate the migrated lanes and the constructive geometry-mass rule while the lanes diverge from the rule's basis. Pinned baseline is the physics authority (science-contracts AGENTS.md); the lanes are the migration target, not the authority. |
| M1 (erosion clock normalization) | Medium | accepted | The surrogate's two legacy operand roles split into named interval operands: `t_exp(it)` (exposure; fills every legacy `timsh` slot; `timpot`/`timex`-partitioned) and `t_norm(it) := dtchr` (fills every legacy `tb` denominator slot — `dct = d_i*t_exp*werod/(t_norm*wflow)`, `dct = eros*rho_soil/(t_norm*wflow)`, capped-eros reconstruction); the triangular factor 2 retires wholesale; constructive closure equation stated (interval mass = flux integrated over `t_norm`; handoff fluxes = mass/`dtchr`). Variables rows + operand-table rows added; vector 1 pins both operands. | Passing `event_duration = dtchr` into the untouched core would silently preserve the banned `2*dtchr` path — exactly the alias the named operands prevent. |
| M2 (`d_i` undefined; density convention) | Medium | accepted | `d_i` defined in Variables and in the `INV-ROUTE-018` text: baseline `di = excess * Kch * (tau - taucr)` (`dcap.for:166`), the CREAMS `e_m` realization, lbm ft^-2 s^-1. `rho_soil` re-pinned as in-place soil **bulk mass density** (CREAMS "mass density of the soil in place"; baseline `wtdsoi` numeric provenance; lbm convention; the variable name's "weight density" noted as a legacy misnomer) in the Variables row and INV-ROUTE-019. | Removes the last guessable symbol in the layer-contact rule. |
| M3 (anchor overclaims) | Medium | accepted | `REF-ROUTE-HECRAS-QUS` narrowed to bed-change **state advancement/carry** with the threshold-gated cross-section refresh named (manual PDF p. 178) and geometry-update authority explicitly re-pointed to Ch-13/lineage; `REF-ROUTE-ARS77-SAMEGRID` narrowed to the zero-upper-boundary-Tc deposition-mode claim with the whole-reach dry-interval rule marked as contract inference; `REF-ROUTE-CREAMS-CH3-QS` mixed-graded for the interpretive "not a physics claim" characterization. Authority-matrix carries a third post-review correction note. | Source fidelity; none of the three narrowings weakens the model-class corroboration, as the review itself states. |
| L1 (stale record claims) | Low | accepted | `contract-disposition.md`: anchor count corrected to eight, vector count to eleven, storage-decision row scoped to the fallback lane; `gate-results.md`: vector count corrected, verification-notes wording now states note 4 (pre-existing v47 lettering artifact) was explicitly deferred rather than addressed; `final-disposition.md`: reopen-cycle record appended, "per-increment bed carry" wording aligned to state-carry; `authority-matrix.md`: HEC-RAS per-increment bullet corrected. | Record truthfulness; the reviewer is right that post-fix reconciliation lagged the contract itself. |

## Adjudications Codex ratified (no action)

- The `INV-ROUTE-015` activation biconditional and `INV-ROUTE-005(a)`
  dependency-authority repair.
- The A-8 rejection (register vocabulary precedent).
- The widening-law primitive adjudication (linear rate, `1.0176`
  exponential, fitted `f(x_b)`, `timpot`/`timex`).
- The quasi-steady-sequence model class corroboration.

## Re-closure

All five required closure amendments from the review's final section are
applied in v52. Re-confirmation is requested from Codex via
`prompts/active/20260710_wshedw11a_codex_reconfirmation_prompt.md`; the
package status notes the pending re-confirmation.
