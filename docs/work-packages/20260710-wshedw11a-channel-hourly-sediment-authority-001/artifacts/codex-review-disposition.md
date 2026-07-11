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

## Re-confirmation cycle disposition (2026-07-10, `codex_reconfirmation.md`)

Codex re-confirmation verdict: `REOPEN` (`WSHED-W11A-RECONFIRM-001`) —
M1/M2/H2/M3 closed; H1 residual + L1 residuals.

| finding | decision | action_taken | rationale |
|---|---|---|---|
| H1 residual (total vs per-unit-length `qlat` conflation) | accepted | `SC-ROUTE-001` v53: distinct canonical symbols — `qlat(it)` = published wave-routing **total** (`m^3 s^-1`, `RoutedChannelWaveState::qlat_m3_s`), partition-only into `leff(it)`; the solve's lateral operand is the derived per-unit-length `qlat_eff(it) := qe(it)/leff(it)` (Eq. [13.5.11]; the baseline `chnrt.for:233-242` local-`qlat`/migrated `qlat_cfs_per_ft` quantity); raw-total and total/`lc` substitution declared invalid; storage expression corrected to all-total form; the pre-existing grouped Variables row split (`qlat_eff` now `ft^3 s^-1 ft^-1`); two new Variables rows; vectors 1/11 re-pinned (anti-alias distinguishes both wrong-unit aliases with `qlat/lc != qe/leff` by construction); unit-bridge derived-normalization note. Verified the three cut-points before binding (`Ran`: `chnrt.for:230-242` "effective lateral inflow (ft^3/sec/ft)" comment; `network_frame.rs:269` `qlat_m3_s`; `02_ws20_segment_routing.rs:89` `qlat_cfs_per_ft = (q_cfs - qu_top_cfs)/leff_ft`). | Codex is right: v52's own storage expression (`qlat*lc`) betrayed the conflation. Symbol continuity kept by reusing the Ch-13 canonical `qlat_eff` name rather than inventing a new one. |
| L1 residuals (record inconsistencies) | accepted | `final-disposition.md` exit-criteria vector count → eleven; verification-notes wording aligned to the note-4 deferral in both the exit table and promotion-gate item 5; `w11-handoff.md` header re-pinned to v53 with the two Codex cycles named. | Record truthfulness. |

Post-fix `Ran`: binding-exposure lint PASS (7 rows). Second
re-confirmation prompt: `prompts/active/20260710_wshedw11a_codex_reconfirmation2_prompt.md`.
