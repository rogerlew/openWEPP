# SNOWDENSITY-10.3.2 Canopy-Stratum Correspondence

Status: complete.

Package type: evidence-only canopy observation/model binding gate.

Primary authority: `docs/planning/snow-frost-fidelity-strategy.md` §10.3,
item 2 "Canopy-Stratum Correspondence"; `SC-SNOWFREEZE-001`
`INV-SNOWFREEZE-050` as the downstream rubric discipline.

Closure target: `COMPLETE-10-3-2-CANOPY-STRATUM-BINDING-DISPOSITIONED`.

Objective: decide whether the current Harvard and Marcell mixed-forest fixtures
can be bound to canopy-stratified observations before those sites carry canopy
melt or density verdicts.

## Scope

- Read the §10.3.2 tuning-sequence authority.
- Read Harvard and Marcell `cancov_forest` fixture manifests and management
  files.
- Consume SNOWDENSITY-10.3.1 canopy-provenance evidence for current runtime
  `cancov`.
- Map observed strata to current modeled hillslope surfaces.
- Decide whether one representative hillslope is defensible, or whether paired
  open/under-canopy model variants must be generated.

## Non-Scope

- No production physics changes.
- No fixture edits.
- No observation ingestion, downloading, or new comparator harness.
- No melt, albedo, density, frost, radiation, coefficient, or canopy tuning.
- No default activation, parser/runfile/user CLI selector, or output schema
  change.

## Acceptance Gates

- Harvard and Marcell observed strata are enumerated from current fixture
  metadata.
- Current modeled surfaces are enumerated from `.man` and 10.3.1 runtime
  evidence.
- Each observed stratum receives an explicit binding disposition.
- The package decides whether one representative hillslope is defensible for
  canopy verdicts.
- No code, fixture input, science contract, or production output schema changes.
- Focused checks pass: `git diff --check`, package evidence presence, and a
  no-diff check for `tests/fixtures/cancov_forest`, `crates`, and
  `docs/specifications/science-contracts`.

## Phase Plan

1. Scaffold package and prompt.
2. Read 10.3.2, fixture manifests, fixture `.man` files, and 10.3.1 evidence.
3. Write observation-to-model stratum correspondence evidence.
4. Write binding decision and downstream constraints.
5. Record review, verification, line-count governance, owned files, handoff,
   and disposition.

## Subagent Authorization

Subagent authorization: this package does not explicitly authorize spawning or
delegating to subagents. Reviews and verification are local unless a later
operator request adds explicit delegation authorization.

## Completion Summary

Closure: `COMPLETE-10-3-2-CANOPY-STRATUM-BINDING-DISPOSITIONED`.

Current Harvard and Marcell fixtures are single mixed-forest hillslopes with
static runtime `cancov = 0.55`. They cannot be bound directly to observed
open, hardwood/deciduous, conifer/hemlock strata for canopy verdicts.

One representative mixed hillslope is defensible only for mixed-site diagnostic
context and runtime plumbing checks. It is not defensible as a canopy-stratum
comparison surface because each site's observed strata intentionally separate
open and under-canopy regimes that the current single modeled hillslope
collapses.

Downstream closure condition before Harvard/Marcell carry canopy verdicts:

- Generate paired model variants for the observed strata, or
- author a documented observation aggregation rule with explicit stratum
  weights and bind the current model only to that aggregate.

## Revision 2026-06-26 — Paired Strata Built

The first closure option (generate paired model variants) was executed: six
per-stratum within-watershed hillslopes were added to
`tests/fixtures/cancov_forest/` (see the disposition and
`stratum-correspondence-evidence.md` revisions for the full binding table).

- **Marcell: spatial binding RESOLVED** — conifer (`marcell_conifer_mn`),
  deciduous (`marcell_deciduous_mn`), and open (`marcell_open_mn`) now bind to the
  three observed strata.
- **Harvard: spatial binding PARTIAL** — hardwood (`harvard_deciduous_ma`) and
  open (`harvard_open_ma`) bind, but the Harvard delineation has no pure conifer
  hillslope, so the **hemlock** stratum is unbound (mixed proxy only).

Canopy-stratum verdicts remain gated on two unchanged preconditions: per-day
winter `cancov` routed into the diagnostic (SNOWDENSITY-10.3.1) and ingest of the
HF237 / USDA RDA stratified observation tables.

No code, fixtures, science contracts, production physics, defaults, output
schemas, selectors, or observations changed.

## Closeout Artifacts

- `artifacts/stratum-correspondence-evidence.md`
- `artifacts/binding-decision.md`
- `artifacts/gate-results.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/review-disposition.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`
- `artifacts/line-count-governance-checklist.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/worker-handoff.md`
- `artifacts/disposition.md`
