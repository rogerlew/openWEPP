# Annual Sediment Adequacy Metric Authority

Status: `EXECUTED-COMPLETE-METRIC-AUTHORITY`
Evidence mode: Ran.
Date: 2026-07-08

## Objective

Resolve the mesh-policy annual pass-sediment adequacy metric authority question
raised by `wa_cascades_forest_h1` day 1126 before any renewed `dx5`
production mesh-policy promotion.

The immediate predecessor attributed the WA `tdep:4` annual sediment
fine-reference miss to annual sediment response to a sub-threshold
routed-hydrograph shape perturbation, not active-router numerics or daily
water-magnitude drift. This package decides the metric contract-first and
replays selected real-cohort evidence under the decided rule.

## Scope

In scope:

- Scaffold package-local execution artifacts and prompt.
- Decide whether annual pass-sediment mesh-policy adequacy remains strict
  per-member-year relative-only or gains a predeclared low-contribution
  annual-vector rule.
- Amend `SC-OFEROUTE-001` before evidence replay if the metric changes.
- Replay the selected real-cohort annual pass-sediment comparisons from the
  prior coupled space-time package under the decided metric.
- Record review, verification, gates, disposition, and worker handoff.

Out of scope:

- No `dx5` production default flip.
- No active mesh default change.
- No routed-shape, routed-outlet, storage, tail-fold, or closure threshold
  changes.
- No sediment process-physics change.
- No rerun of the heavy coupled ladder unless the existing raw outputs are
  missing or inconsistent.

## Required Reading

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/standards/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/work-packages/20260708-laned-router-wa-sediment-reference-adequacy-attribution-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260708-laned-router-wa-sediment-reference-adequacy-attribution-001/artifacts/wa-sediment-attribution.md`
- `docs/work-packages/20260708-laned-router-wa-sediment-reference-adequacy-attribution-001/artifacts/classification.md`

Conditional:

- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` rev 43
  active mesh-policy judged surfaces and change log.
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
  `hourly_runoff_fraction` / `INV-SED-013` routed-hydrograph shape consumer.
- `docs/work-packages/20260708-laned-router-tier2-dx5-coupled-spacetime-ratification-001/artifacts/coupled-spacetime-summary.json`
  and raw pass parquets for selected-cohort evidence replay.

On demand:

- D13 routed-hydrograph erosion-shape consumer evidence.
- D15A active-owner erosion water-magnitude follow-on note.

## Phase Plan

### Phase A - Scaffold and Authority Map

- Create package-local `package.md`, `artifacts/`, `prompts/active/`,
  `prompts/archived/`, and catalog/roadmap pointers.
- Record required-reading and evidence provenance.

### Phase B - Metric Authority Decision

- Evaluate strict per-member-year relative annual sediment against the
  predecessor's low-denominator evidence.
- Evaluate a predeclared low-contribution annual-vector rule that is not fitted
  to the WA observed value:
  - material member-years keep the current per-year relative threshold;
  - low-contribution member-years are controlled by a per-column annual-vector
    L1 relative threshold across all years.
- Amend `SC-OFEROUTE-001` before replay if the metric changes.

### Phase C - Evidence Replay

- Replay all selected real-cohort annual pass-sediment comparisons from the
  coupled space-time package under the decided metric.
- Record current strict-relative values and revised metric values side by side.
- Confirm whether the WA `tdep:4` annual sediment blocker closes under the new
  rule and whether any new annual sediment blocker appears.

### Phase D - Disposition

- If the metric remains strict relative-only, hold with evidence and first
  follow-on.
- If the metric changes and replay closes annual sediment adequacy, complete
  metric authority and hand off renewed `dx5` mesh-policy ratification/flip.
- Do not flip production defaults in this package.

## Subagent Authorization

This package explicitly authorizes subagent spawning/delegation for read-only
review and verification. Authorized roles:

- review: inspect contract authority, metric decision, and no-tolerance-fitting
  posture.
- verification: independently check the analyzer replay, contract delta, and
  gate claims.

Expected outputs are package-local `artifacts/review-*.md` and
`artifacts/verification-*.md`. Write access is bounded to this package's
artifact directory unless the operator explicitly expands scope.

## Required Artifacts

- `artifacts/README.md`
- `artifacts/required-reading-map.md`
- `artifacts/metric-authority-decision.md`
- `artifacts/annual-sediment-metric-replay.json`
- `artifacts/annual-sediment-metric-replay.md`
- `artifacts/contract-disposition.md`
- `artifacts/implementation.md`
- `artifacts/gate-results.md`
- `artifacts/review-*.md`
- `artifacts/verification-*.md`
- `artifacts/disposition.md`
- `artifacts/final-disposition.md`
- `artifacts/worker-handoff.md`

## Gates

Required:

- `python -m py_compile artifacts/analyze_annual_sediment_metric.py`
- analyzer replay command recorded in `artifacts/gate-results.md`
- `git diff --check`
- Markdown/doc lint for touched docs
- Contract/profile/BEI checks required by touched `SC-*` contracts

Conditionally required:

- Focused Lane D / `ofe_routing` tests if Rust code changes.
- Full Rust gate suite only if production Rust code or contract bindings are
  changed:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo nextest run --workspace --profile full`
  - `cargo deny check`

## Exit Criteria

`EXECUTED-COMPLETE-METRIC-AUTHORITY`:

- Metric authority is decided in `SC-OFEROUTE-001` if changed.
- Selected real-cohort annual pass-sediment evidence is replayed under the
  decided metric.
- Review and verification findings are dispositioned.
- Production `dx5` remains unflipped in this package.
- Worker handoff names the next production ratification/default-promotion
  package or any remaining blocker.

`EXECUTED-HOLD-*`:

- Metric authority cannot be safely decided in this envelope, or replay reveals
  a new annual sediment blocker under the decided rule.
- Hold audit names exact blocker, evidence, considered correction routes, and
  first actionable follow-on.

## Final Outcome

`EXECUTED-COMPLETE-METRIC-AUTHORITY`.

`SC-OFEROUTE-001` rev 44 replaces the annual pass-sediment mesh-policy
strict relative-only gate with a material-year plus annual-vector rule. The
package replayed all 21 selected real-cohort annual pass-sediment comparisons
from the coupled space-time package. The pre-rev44 strict rule has one blocker
(`wa_cascades_forest_h1` refined-75 `tdep:4`, `0.022131684` >
`0.0066666667`); the rev-44 rule has zero blockers. No `dx5` production
default flip, active mesh default change, routed-water change, routed-shape
threshold change, or sediment process-physics change is made here.
