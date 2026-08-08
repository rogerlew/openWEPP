# Paradise WY2015 Support Resolution

Status: `complete / support cause localized / non-blocking diagnostic`

Date: `2026-08-07`

Package ID: `20260807-snow-stage3-paradise-wy2015-support-resolution-001`

Plan class: `Critical evidence characterization; no production change`

This ExecPlan is governed by `docs/codex_exec_plans.md` and
`docs/work-packages/AGENTS.md`.

## Purpose

Resolve the exact dates, terms, operator statuses, and snow-state transitions
behind Paradise WY2015's frozen `183` unmatched hours, `19` partial-support
hours, and `98.0757 MJ m^-2` omitted magnitude. Preserve the historical
`0.0621730192 > 0.05` support failure. Determine whether direct trace evidence
supports a narrower causal classification; never call the row noise, retune the
threshold, or let this one censored diagnostic carry a physical pass/fail claim.

## Progress

- [x] (2026-08-07) Scaffolded from campaign row 31 and predecessor evidence.
- [x] (2026-08-07) Froze exact inputs, algorithms, claims, and write set.
- [x] (2026-08-07) Implemented and tested the independent localization consumer.
- [x] (2026-08-07) Executed custody-complete immutable attempt 004 and reconciled all totals.
- [x] (2026-08-07) Completed gates, dual review, dual verification, and disposition.

## Implementation Intent

Intent is `diagnostic characterization`. This package does not implement
science, calibrate, validate physical magnitude, change a support threshold, or
alter production behavior. Existing schema-v6 traces are the only result input.

## Frozen Intake

- Source head at scaffold: `624d169ea6b860f5d6c0972ddc9ef023c1e6a98c`.
- Parent result:
  `target/snow_stage3_evolving_state_carrier_plausibility_reconciliation/attempt-004/results/evolving-carrier-plausibility-results.json`, SHA-256
  `7bd19a24b63375dba9f61e8d522afcc43b42b9f9a8dd8d6156cbe9fad1fbbbff`.
- Contextual parent analysis receipt (not consumed by this analyzer):
  `target/snow_stage3_evolving_state_carrier_plausibility_reconciliation/attempt-004/execution-receipt.json`.
- Consumed retained operator receipt:
  `target/snow_stage3_operator_reconciliation_v3/execution-receipt.json`,
  SHA-256 `61564035575b165722213abe0657a4dc70b04a1d72c1200b1bb5e35d435fdc9e`.
- Exact Paradise paired/sequential schema-v6 traces and their hashes are bound
  by the parent protocol freeze and operator-reconciliation receipt.
- Target row: `snotel_paradise_wa`, water year `2015`, observation-window end
  selected by the unchanged parent cohort rule.
- Frozen support threshold: `0.05` from `TOL-SNOWFREEZE-019`.

## Included Scope

- Verify parent result, receipt, climate, observation, and trace custody.
- Reuse the parent consumer's exact date, water-year, observation-window,
  tuple validation, hourly grouping, common-prefix, and term reconstruction.
- Emit every unmatched and partial hour with date/hour, S/Q support, operator
  statuses, tuple counts, before/after snow state, and omitted term magnitude.
- Reconcile hour rows exactly to `183`, `19`, and the parent omitted magnitude.
- Distinguish direct causal evidence from descriptive association and unknowns.
- Update package, roadmaps/catalog, and canonical contracts/tests only if a new
  binding authority statement is necessary.

## Excluded Scope

- Production Rust, fixtures, observations, schemas, climate, defaults, or
  WAT/HBP/PASS outputs.
- Threshold tuning, record deletion, support imputation, or calling omissions
  noise without direct cause evidence.
- Physical wind, canopy, geometry, magnitude, persistence, promotion, CoE
  ownership, or cutover claims.

## Intended Write Set

- This package tree.
- `docs/ROADMAP.md`, the snow campaign roadmap, and work-package catalog.
- Existing science contracts/index and contract-derived tests only if the
  result requires a binding clarification.
- DRAFT assurance files only if an identified report dependency changes.

`target/` receives immutable analysis output only and remains untracked.

## Execution Plan

1. Freeze input identities and result-blind classification rules.
2. Author an independent package-local consumer with synthetic adversarial
   tests and no production imports.
3. Execute against retained paired/sequential traces into a new immutable
   attempt directory.
4. Reconstruct parent counts and magnitudes; classify dates/status/transitions.
5. Record claim-limited disposition and update campaign routing.
6. Run selected gates, dual independent review, finding disposition, and dual
   independent verification.

## Exit Criteria

- Exact custody and parent-total reconciliation pass.
- Every one of the `202` affected hours is inventoried without overlap.
- Omitted magnitude reconciles by term and support class.
- Any causal label is backed by direct tuple/state evidence; otherwise the
  result remains descriptive or unknown.
- The `0.05` threshold and historical failure remain unchanged.
- Protected boundaries, line-count governance, reviews, verification, and
  terminal diff reconciliation pass.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes spawning/delegating
to science-review and independent-verification subagents for result review,
terminal review, and verification; expected outputs are package review and
verification artifacts; write access is read-only.

## Outcomes & Retrospective

Paradise WY2015 contains exactly 19 daily thin-pack termination episodes. Each
episode begins with one partial sequential hour (`300--3300 s`) whose final
tuple reports `post_substep_no_resolved_surface` with `0.9560--0.9999 kg m^-2`
ice remaining, followed by 183 same-state-only hours for which sequential
status is directly `thin_pack_boundary_reached`. Thus the support topology has
a direct evaluator-state cause: within-day sequential evaluation reaches its
thin-pack/no-resolved-surface boundary while the immutable same-state arm
continues. This explains the missing support, not whether the boundary or
underlying physics is correct. The historical `0.0621730192 > 0.05` failure
remains, as do all physical-authority and persistence holds.
