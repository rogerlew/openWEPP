# Final Disposition

Status: `EXECUTED-COMPLETE-AUTHORITY` — **final**. Codex cycle-2
re-confirmation `RATIFIED` (2026-07-10, `codex_reconfirmation2.md`): both
residuals closed; `WSHED-W11-HOLD-001` stands lifted; W11 may resume at
Phase B under `SC-ROUTE-001` v53. (The two reopen-cycle records below are
the audit trail.)

Evidence mode: `Static` + `Ran`, per-artifact labels throughout the package.

Executor: Claude Code, operator-directed package-end-to-end execution
(2026-07-10), under the standing operator-authorized exception for
Claude-executed contract authoring with Codex review to follow.

## Exit-criteria adjudication

| Exit criterion | Met by |
|---|---|
| Supported water branch/time grid | Inherited W11 record (`ipeak >= 3` routed `q1(it)` on the normalized `dtchr` grid); restated in INV-ROUTE-015 |
| Temporal sediment quantum and state-carry order | `INV-ROUTE-015` (dtchr grid, biconditional mandatory activation), `INV-ROUTE-017` (monotonic geometry carry, tillage-only reseed), sequencing steps in the W11A addendum |
| Per-class ingress/egress and detachment/deposition/storage closure | `INV-ROUTE-016` (per-interval quasi-steady sequence on the WSHEDIMPL18-41 lanes), `INV-ROUTE-018` (lineage-realization widening clock with `timpot`/`timex` budget partition), `INV-ROUTE-019` (per-interval/day class mass closure, projection exactness, constructive geometry-mass derivation) |
| Typed failure behavior and tolerances | Guard-map rows 015-020 (`WKERNEL-WS10-CHANNEL-E-001..003` family), `TOL-ROUTE-006..008` with the zero-mass carve-out, invalid-state rows |
| Contract-derived test vectors | Eleven vector obligations in the W11A addendum as of v52/v53 (equivalence with pinned operands, closure, carry, widening clock, zero-flow, storage, cross-day, tillage reseed, fail-closed family, layer contact + divergent terminals, hydraulic-profile anti-alias) |
| Explicit W11 resume instructions | `artifacts/w11-handoff.md` + updated W11 package `worker-handoff.md` |
| Dual review and verification, no undispositioned findings | 21 findings dispositioned (20 accepted+fixed, 1 rejected with validated rationale); both verifiers PASS-WITH-NOTES with all in-scope notes addressed in-cycle (verification-B note 4, a pre-existing v47 lettering artifact, explicitly deferred — see Residual items); see `gate-results.md` |

## Authority summary

The ratified process is the **per-`dtchr`-interval quasi-steady sequence**
with monotonic geometry carry — convergent from the WEPP/CREAMS lineage
(Ch. 13 §13.5.1 geometry carry; CREAMS [I-56] compute-cost rationale for the
event-scalar collapse; Ch. 14 internal per-time-step precedent), USDA-ARS
KINEROS ARS-77 (same-grid sediment/water coupling; the Bennett 1974 unsteady
parent recorded as the fallback form), and USACE HEC-RAS quasi-unsteady
(the sequence-of-steady-profiles class with per-increment bed-change
**state carry** — cross-section refresh is threshold-gated in HEC;
class-corroboration grade). The two labeled refinements beyond legacy
source-intent (per-interval solve form; per-interval re-anchored widening
clock on the WEPP-adapted lineage realization) are explicitly labeled with
recorded fallbacks. No surrogate physics was ratified; no HBP schema change
was required (the package exclusion held); SC-SED-001 and SC-SYSTEM-001
required no amendment (rationale in `contract-disposition.md`).

## Promotion-gate check (authoring procedure)

1. Two independent reviews completed — YES (parallel, B not primed with A).
2. Disposition complete, no missing rows — YES (21/21).
3. Accepted findings fixed and verified — YES (verification A: all 20 closed).
4. Rejected findings carry authority-backed rationale — YES (verification B
   validated A-8's rejection with corpus-wide evidence).
5. Both verifiers PASS or PASS-WITH-NOTES — YES (both PASS-WITH-NOTES; all
   in-scope notes cosmetic and addressed in-cycle; note 4 deferred as a
   pre-existing artifact).
6. Remaining open items gap-registered — YES (GAP-ROUTE-012/013 closed with
   retained-limitation labels; per-class-hourly interchange remains
   `SC-SED-001#GAP-SED-008`, open, cross-referenced).
7. No invariant without guard mapping — YES (rows 015-020 mapped).
8. Baseline provenance mapping — YES (REF-ROUTE-GULLY-STATE pinned commit;
   WSHEDIMPL18-41 lineage named).
9. BEI checks pass — YES (`Ran`: lint PASS, 7 rows, re-run after fixes).

## Residual items (none blocking)

- The peak-form/event-scalar lane remains production for all non-activated
  configurations; the interval lane activates only under the INV-ROUTE-015
  biconditional. Production behavior is unchanged until W11 implements.
- Verification-B note 4 (pre-existing v47 revision-row clause-lettering
  artifact) predates this package and is left to a future editorial pass.
- Codex post-hoc review of this operator-directed Claude-executed cycle is
  requested via the dispatch prompt in `prompts/active/`.

`WSHED-W11-HOLD-001` is lifted. W11 resumes at its contract-first phase
(Phase B) per `w11-handoff.md`.

## Codex post-hoc reopen cycle (2026-07-10)

The operator-dispatched Codex post-hoc review (`codex_posthoc_review.md`)
returned `REOPEN` (`WSHED-W11A-POSTHOC-001`): 2 High (H1 hydraulic-profile
operand under-determination; H2 migrated-lane terminals diverging from
pinned `dcap.for`), 3 Medium (M1 erosion-clock normalization operand; M2
undefined `d_i` + density convention; M3 anchor overclaims), 1 Low (L1
stale record claims). It simultaneously ratified the activation
biconditional, the A-8 rejection, the widening-law primitive adjudication,
and the model-class corroboration.

All six findings were accepted and closed in `SC-ROUTE-001` v52
(`codex-review-disposition.md` carries the row-by-row disposition; both H2
terminal claims were independently re-verified against pinned
`dcap.for:160-262` before being bound into contract text). The v52
additions: the unique hydraulic-profile operand map
(`qe/qt/qlat := q1(it)/qin(it)/qlat(it)`), the `t_exp`/`t_norm` operand
split, `d_i` and the `rho_soil` mass-density convention, the
pinned-`dcap.for`-as-realization binding with `GAP-ROUTE-014` (open: the
two migrated terminals must be corrected in W11 Phase B before interval
reuse), vectors 10(b)/(c) and 11, and the three anchor narrowings.
Post-fix BEI lint: PASS (7 rows).

The exit-criteria adjudication above stands with these substitutions: the
vector count is eleven; the "no executor science choice" claim now rests on
the v52 operand maps; and `GAP-ROUTE-014` is an open, W11-assigned
correction obligation (a code-parity defect flag, not missing authority).
Final closure of this cycle awaits Codex re-confirmation
(`prompts/active/20260710_wshedw11a_codex_reconfirmation_prompt.md`).

## Codex re-confirmation cycle (2026-07-10, second pass)

`codex_reconfirmation.md` returned `REOPEN` (`WSHED-W11A-RECONFIRM-001`):
M1/M2/H2/M3 verified closed; H1 residual — v52 still aliased the
wave-routing **total** lateral series (`m^3 s^-1`) and the Chapter-13
per-unit-length erosion operand (`ft^3 s^-1 ft^-1`) under one `qlat(it)`
symbol (the v52 storage expression's `qlat*lc` betrayed the conflation) —
plus L1 record residuals. Both accepted and closed in `SC-ROUTE-001` v53:
distinct `qlat(it)` (wave total, partition-only) and derived
`qlat_eff(it) := qe(it)/leff(it)` (the solve operand) with unit-distinct
Variables rows, invalid-substitution rules, corrected all-total storage
expression, re-pinned vectors 1/11, and the unit-bridge
derived-normalization note; record reconciliations applied
(`codex-review-disposition.md` re-confirmation section). Post-fix BEI
lint: PASS. Final closure awaits cycle-2 re-confirmation
(`prompts/active/20260710_wshedw11a_codex_reconfirmation2_prompt.md`).
