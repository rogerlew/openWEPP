# SNOWDENSITY-04 Offline Adjudication Loop

Status: complete.

Package type: offline implementation / adjudication.

Primary gap: `GAP-SNOWFREEZE-002`.

Objective: adjudicate the offline `physics_bulk` candidate against the v74/v75
SNOTEL rubric and H openWEPP/legacy/PySnobal comparator profiles. Iterate only
inside the ratified `INV-SNOWFREEZE-051` envelope using global named candidate
variants; close either with a non-site-tuned candidate that beats legacy on
forcing-robust cells or with a documented non-promotion route and next physics
escalation.

This package follows `AGENTS.md`, `docs/work-packages/AGENTS.md`,
`crates/AGENTS.md`, `tests/AGENTS.md`,
`docs/specifications/science-contracts/AGENTS.md`,
`docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
(`INV-SNOWFREEZE-050`, `INV-SNOWFREEZE-051`, `OBL-SNOWFREEZE-P-026`,
`TOL-SNOWFREEZE-011`), ADR-0017, ADR-0027,
`docs/planning/snow-frost-fidelity-strategy.md`, SNOWDENSITY-03 evidence, and
SNOWFROST-FIDELITY-H evidence.

Subagent authorization: none. Execute locally and record review/disposition in
package artifacts.

## Scope

In scope:

- Offline-only named `physics_bulk` candidate variants with global constants.
- A SNOTEL adjudication tool that runs/scored those variants and compares them
  against the existing H openWEPP/legacy/PySnobal profiles.
- Explicit profile-level comparison using forcing-robust rubric cells.
- Focused tests proving variants remain offline, no-site, and production
  confined.
- Package evidence, review, verification, line-count governance, and handoff.

Out of scope:

- No production runtime coupling, parser/config activation, output schema
  changes, default activation, or compatibility deletion.
- No per-site constants, SSD fitting, residual minimization by site, or
  SNOTEL-trained parameter selection.
- No PySnobal runtime dependency or hardening.
- No frost heat-flow, frozen-K/SFCC, impedance, or migration/fringe work.
- No claim that observation disagreement alone creates an
  `OPENWEPP-DEFECTIVE` verdict.

## Acceptance Criteria

- Required reading is recorded.
- Named variants and their constants are documented as global, candidate-only,
  and inside the `INV-SNOWFREEZE-051` process envelope.
- Offline `openwepp-snowbench physics-bulk` can run at least the baseline and
  named variants without production runtime coupling.
- The adjudication runner emits JSON/Markdown comparing every candidate variant
  to H openWEPP/legacy/PySnobal profiles by forcing-robust rubric counts,
  robust ordinal score, and per-cell wins/losses.
- Closure disposition is one of:
  - `COMPLETE-PROMOTION-CANDIDATE`: a global variant beats legacy/openWEPP on
    forcing-robust cells without site constants; or
  - `COMPLETE-NON-PROMOTION`: no global bulk variant beats the comparator
    profile; handoff names the smallest next physics escalation.
- Production confinement, no-site-tuning, review, verification, line-count, and
  package gate evidence are complete.

## HOLD Boundaries

Close as `HOLD` only if the adjudication runner cannot produce finite rubric
profiles for all five SNOTEL fixtures, the H comparator profile is unavailable,
or the only apparent path forward requires per-site constants or production
runtime coupling.

## Execution Plan

1. Scaffold package and active kickoff prompt.
2. Read required authority, SNOWDENSITY-03 handoff, H profile evidence, and the
   `physics_bulk` implementation.
3. Add offline-only named variants and tests.
4. Add the adjudication runner and run baseline + named variants.
5. Compare candidate profiles against H openWEPP/legacy/PySnobal profiles.
6. Decide promotion-candidate vs non-promotion from current evidence.
7. Run focused and closure gates.
8. Record evidence, reviews, verification, line-count governance, and handoff.

## Closeout

Disposition: `COMPLETE-PROMOTION-CANDIDATE`.

This package added global named offline variants for `physics_bulk`, an
adjudication runner, and a compact SNOTEL profile comparison against the H
openWEPP/legacy/PySnobal report. The best variant is
`dense_slow_melt_v1`:

- openWEPP as-built robust profile: `fail=9`, robust ordinal score `84`.
- legacy as-built robust profile: `fail=9`, robust ordinal score `84`.
- `dense_slow_melt_v1`: `fail=6`, robust ordinal score `102`.

This satisfies the SNOWDENSITY-04 promotion-candidate rule: lower
forcing-robust failure count with no loss of robust ordinal score against both
openWEPP and legacy as-built profiles. It does not activate runtime production
behavior, change defaults, unblock frost attribution, or delete legacy snow.
SNOWDENSITY-05 owns runtime opt-in coupling and production gate evidence.
