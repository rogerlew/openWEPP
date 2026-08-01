# EB-04U Terminal Verification — Agent A

Evidence class: `Static + Reused Ran + Ran review-only checks`.

Verdict: `PASS` for the narrowed
`PROSPECTIVE_DIAGNOSTIC_DESIGN_COMPLETE` disposition.

## Finding Verification

### `EB04U-A-001` — PASS

The generated seasonal protocol now computes the primary evaluation frame once
from observations by water year and observable. Baseline and every candidate
must use the same observed dates. The protocol deterministically takes the
earliest accepted maximum, reports all ties, makes missing or invalid observed
boundaries inconclusive, and prohibits a modeled date from substituting for an
observed boundary. Candidate-derived dry/wet/model-peak labels are explicitly
secondary diagnostics and cannot alter primary efficacy membership.

This removes the earlier observed/model-earlier-peak contradiction and prevents
a candidate from changing the dates on which it is scored. Unresolved window
or regime thresholds are now mandatory authority-backed pre-result successor
seals rather than EB-04U efficacy operators.

### `EB04U-A-002` — PASS

The operand registry now separates:

- `stage3_cold_content_vapor_energy`, directly available with shortwave,
  longwave, latent, conduction, before/after/exported cold content, refreeze,
  unused-positive-energy, and closure-residual fields; and
- `coe_melt_energy_drivers` plus `rain_and_sensible_heat`, both explicitly
  `MISSING_REQUIRED` and assigned to EB-04W.

The anti-alias text states that Stage 3 unused positive energy cannot stand in
for CoE melt authority. The seasonal and efficacy protocols likewise prohibit
inferring CoE melt causality from Stage 3 cold-content energy. This agrees with
`SC-SNOWENERGY-001`: Stage 3 applies bounded energy to cold content without
energy-balance melt, while `INV-SNOWENERGY-019` requires cold-content closure
from applied energy, conduction, refreeze, and exported cold content.

### `EB04U-A-003` — PASS

The decision protocol now states that directional movement and a one-cell band
crossing are diagnostic only. It admits no result-bearing efficacy study.
Before such a study, a successor must prospectively seal an authority-backed
materiality threshold, replication unit, minimum improved fraction/count,
minimum independent-site count, missing-record rule, and stratification rule.
EB-04V must preserve the two density-bias directions separately, EB-04W must
resolve forcing/process ownership first, and EB-04X must protect open-lane
geometry. The admission matrix repeats these prerequisites for all three
successors.

This is a valid narrowing rather than a deferral of a current result-bearing
gate: EB-04U claims only completion of a diagnostic population/operator/
observability design, and its package, synthesis, roadmap, catalog, and
machine-readable disposition consistently deny a result-bearing efficacy or
promotion study.

## Independent Reconstruction

A separate read of EB-04U and EB-04T artifacts confirms:

- 16 failure rows and 16 unique `(lane_id, cell_id)` identities, exactly equal
  to the EB-04T source population;
- cohorts of nine density/structure, two Harvard geometry/interception, and
  five mountain under-persistence rows;
- successor assignment of EB-04V `9`, EB-04W `5`, and EB-04X `2`;
- all 16 failure rows `DIAGNOSTIC_ONLY` and ten distinct observation lanes;
- 40 retained trace/WAT cells;
- 26 operand rows: 16 `AVAILABLE_DIRECT`, one `AVAILABLE_DERIVED`, one
  `AVAILABLE_SEMANTIC_PROOF_REQUIRED`, one `PARTIAL_AMBIGUOUS`, and seven
  `MISSING_REQUIRED`; and
- zero model subprocesses, zero new candidate results, and no promotion in the
  manifest summary.

The fifth EB-04W row remains the Niwot peak-depth timing failure with explicit
density coupling; it is not presented as a uniquely identified mass-loss
cause. The openWEPP wind-redistribution process and wepppy/forcing-provider
precipitation/phase responsibilities are also separated in the readiness
matrix.

## Checks Run

- `.venv/bin/python .../tools/build_partition.py --self-check` — PASS,
  including frozen inventory and rejected-alias/overlap controls.
- `markdown-doc lint --path <EB-04U package>` — 22 files, zero errors and zero
  warnings before this verification artifact.
- Scoped Markdown lint for `docs/ROADMAP.md`, the campaign roadmap, and the
  work-package catalog — zero errors and zero warnings.
- `git diff --check` — PASS.
- Protected-path diff inspection — empty under source, tests, contracts,
  science contracts, EB-04S, and EB-04T.

No closure-blocking science, mechanism, evidence-role, or scope finding remains
for the narrowed diagnostic-design claim.

## Exact-Terminal Addendum — 2026-08-01

Verdict: `PASS`.

After closure artifacts and prompt archival, I rechecked the exact terminal
state. `package.md` is `complete / PROSPECTIVE_DIAGNOSTIC_DESIGN_COMPLETE` and
its progress/outcome claims agree with `final-disposition.md`,
`exact-diff-reconciliation.md`, `worker-handoff.md`, and `gate-results.md`.
The handoff preserves EB-04V as bounded diagnostic work and does not admit a
result-bearing efficacy or promotion study.

The active prompt directory is empty and `prompts/archived/execute.md` contains
the completed kickoff. Terminal Markdown lint passes for all 27 package files
and each of the three shared roadmap/catalog files; `git diff --check` passes.
`git status` contains only the new EB-04U package tree and modifications to the
three declared shared documentation paths. Protected source, tests, contracts,
science contracts, retained evidence, EB-04S, and EB-04T remain unchanged. No
terminal closure inconsistency or new blocking finding was identified.
