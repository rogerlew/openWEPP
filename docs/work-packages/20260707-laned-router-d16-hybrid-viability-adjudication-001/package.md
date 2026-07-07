# LANED Router D16 Hybrid Viability Adjudication

Status: EXECUTED-HOLD-HYBRID-VIABILITY

## Objective

Adjudicate whether the current hybrid implicit-explicit Lane-D stepper is
viable for active-path default promotion after the selected-cohort active suite
became executable, and if not, name the exact hold conditions and the levers
most likely to make the hybrid viable.

This package is a promotion/viability decision package. It does not flip any
selector, amend physics, or implement optimizer code.

## Rationale

The earlier D16 default-promotion package held on missing fidelity/tolerance
authority using H2637-only evidence. The selected-cohort suite later held on
`mn_corn_h4` active canhgt publication, and the row-crop canhgt package closed
that blocker. The selected cohort now runs end-to-end in active plain and
explicit hybrid modes, so the promotion question can be re-adjudicated with
broader timing and fidelity evidence.

The evidence shows the hybrid has a real algorithmic win on H2637, but the
current selector is not no-harm across the selected cohort.

## Required Reading

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md`
- `docs/backlog/20260706-laned-router-numerics-performance-tiers.md`
- `docs/work-packages/20260707-laned-router-d16-hybrid-default-promotion-001/package.md`
- `docs/work-packages/20260707-laned-router-d16-hybrid-default-promotion-001/artifacts/timing-and-fidelity.md`
- `docs/work-packages/20260707-laned-router-d16-hybrid-default-promotion-001/artifacts/hold-legitimacy-audit.md`
- `docs/work-packages/20260707-laned-router-d16-selected-cohort-active-suite-001/package.md`
- `docs/work-packages/20260707-laned-router-d16-rowcrop-canhgt-active-runtime-publication-001/package.md`
- `docs/work-packages/20260707-laned-router-d16-rowcrop-canhgt-active-runtime-publication-001/artifacts/timing-and-deltas.md`
- `docs/work-packages/20260707-laned-router-d16-rowcrop-canhgt-active-runtime-publication-001/artifacts/active-suite-command-log.json`
- `docs/work-packages/20260707-laned-router-d16-rowcrop-canhgt-active-runtime-publication-001/artifacts/active-suite-summary.json`
- `docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/timing-and-fidelity.md`
- `docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/worker-handoff.md`

## Scope

Included:

- Re-adjudicate `SC-OFEROUTE-002#INV-OFEHYB-008` promotion posture using the
  now-executable selected cohort.
- Summarize selected-cohort wall/user timing, active Lane-D profile counters,
  closure surfaces, output deltas, and pass-sediment deltas.
- Decide whether default promotion is allowed at current mesh.
- If held, identify the hold conditions and rank the next optimization /
  selector levers.
- Produce a worker handoff for a hold-lift package.

Excluded:

- Selector default flip.
- Rust implementation changes.
- Contract amendments.
- New timing runs beyond the already-current selected-cohort evidence.
- Tier-2 mesh-policy changes.
- Broad no-env `OPENWEPP_LANED_ACTIVE` production activation.

## Adjudication Result

Current hybrid is **NO-GO for default promotion**.

Hold conditions:

- `INV-OFEHYB-008` still lacks ratified production-facing fidelity tolerances
  for the H2637 publication deltas.
- The selected cohort fails the timing no-harm expectation: aggregate selected
  user time is `57.34 s` plain vs `59.95 s` hybrid (`+4.55 %` hybrid), driven
  by the WA Cascades forest case (`15.65 s` plain vs `24.50 s` hybrid,
  `+56.55 %`).
- The current selector lacks a preflight/adaptive no-harm rule to avoid
  generic non-bare implicit solves when their map-iteration cost exceeds the
  explicit-step savings.

Positive signal:

- H2637 remains a real win: `40.17 s` active plain vs `33.62 s` active hybrid
  user time (`-16.31 %`), with zero implicit equilibrium-map evaluations
  because the exact bare-skin evaluator applies.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes
spawning/delegation to comparator/timing, science-authority review, package QA,
and verification subagents for selected-cohort timing review, delta review,
gate review, and package disposition review. Expected outputs are
package-local `artifacts/review-*.md`, `artifacts/verification-*.md`, and
compact comparator/timing evidence. Write access is read-only unless a worker
is explicitly assigned a bounded package-artifact fix.

No subagent was required for this execution because the package is docs-only
and uses already-current package-local evidence.

## Required Artifacts

- `artifacts/required-reading-map.md`
- `artifacts/timing-profile-adjudication.md`
- `artifacts/promotion-readiness-audit.md`
- `artifacts/viability-levers.md`
- `artifacts/hold-legitimacy-audit.md`
- `artifacts/gate-results.md`
- `artifacts/review-codex.md`
- `artifacts/verification-codex.md`
- `artifacts/disposition.md`
- `artifacts/final-disposition.md`
- `artifacts/worker-handoff.md`

## Gates

- `git diff --check`.
- Markdown/doc lint for touched docs.
- Evidence audit of selected-cohort active run logs and summary JSON.
- `SC-OFEROUTE-002#INV-OFEHYB-008` promotion posture audit.
- Line-count governance statement.
- Rust closure gates: not applicable unless this package changes Rust code.
- Contract/profile/BEI checks: not applicable unless this package changes
  `SC-*` contracts.

## Closure

`EXECUTED-HOLD-HYBRID-VIABILITY`: no selector flip, no contract amendment, and
no production promotion. The next actionable package is a hold lift for an
adaptive no-harm selector and non-bare implicit solve-cost reduction, followed
by renewed fidelity/tolerance adjudication.
