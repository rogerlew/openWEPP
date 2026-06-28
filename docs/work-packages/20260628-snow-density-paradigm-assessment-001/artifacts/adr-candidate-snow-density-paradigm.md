# ADR Candidate: Climate-Class Snow-Density Specialization First

Status: Superseded by ADR-0029.

Superseded by:
`docs/decisions/0029-commit-paradigm-2-multilayer-snow.md`, ratified by
`docs/work-packages/20260628-adr0029-paradigm-2-ratification-001/`.

Reason: SNOWDENSITY-10.3.22 source-verified and executed the climate-class
candidate, but it closed `HOLD-GATE-FAILURE-NON-PROMOTION`; ADR-0029 therefore
supersedes this WP-local Paradigm-1-first candidate with the staged Paradigm 2
program decision.

Proposed by:
`docs/work-packages/20260628-snow-density-paradigm-assessment-001/`.

## Context

The current no-env snow default, activated bundle plus Harder-Pomeroy hourly
phase partition, scores `15` robust fails / `179` on the cross-SNOTEL
forcing-robust rubric. SNOWDENSITY-10.3.21 decomposes the residual into:

- `9/15` robust fails in seasonal densification trajectory, diffuse and
  split-sign across climates;
- `4/15` robust fails in mountain timing under-persistence;
- `2/15` robust fails in humid-New-England depth-SWE geometry.

SNOWDENSITY-10.3.16 through 10.3.20 did not find a winning additional lever
inside the global bulk SNOBAL/CoE/Anderson family. ADR-0028 permits
observed-data admission when process authority is under-specified, provided the
candidate is physically defensible, improves forcing-robust rubric evidence,
does not fit evaluation fixtures, preserves comparator-as-flag posture, and
conserves mass.

## Decision

Use **climate-class snow-density specialization** as the first post-bulk
density-structure candidate before pursuing a full multilayer snowpack rewrite.

This decision does not activate a new default. It authorizes a later opt-in
candidate package to test class-aware bulk densification coefficients under the
cross-SNOTEL forcing-robust rubric.

## Candidate Shape

The later candidate should:

- add an explicit opt-in density selector, for example
  `physics_bulk_climate_class_density_v1`;
- preserve the current no-env default and explicit rollback selectors;
- assign snow class from independent authority: Sturm 1995 weather-driver
  classification, NSIDC-0768, or a documented equivalent, not site identity;
- use Sturm 2010 as regime-divergent density evidence while translating the
  idea into class-aware Anderson/SNOBAL-style coefficients;
- avoid fitting coefficients to the cross-SNOTEL or `cancov_forest` evaluation
  fixtures;
- preserve current density cap semantics unless a separate contract amendment
  authorizes a change;
- preserve whole-model mass conservation and existing public output schema.

## Consequences

Positive:

- Directly targets the dominant split-sign densification residual.
- Keeps the next experiment small enough to fail cheaply.
- Uses independent climate-class authority instead of site calibration.
- Avoids committing the array-native hot path and winter-column state to
  multilayer structure before evidence requires it.

Negative:

- Climate classes are still an empirical abstraction and may be brittle near
  class boundaries or under climate drift.
- It is unlikely to solve the mountain timing or cancov geometry clusters.
- A later multilayer package may still be necessary if frost attribution or
  cross-SNOTEL evidence requires vertical snow structure.

## Rejected Alternatives

### Immediate Multilayer Rewrite

Rejected for the next step, not permanently. Crocus/SNOWPACK/SNTHERM-style
layer physics is the more complete process answer, but it requires new state,
projection, conservation, and performance work before the existing rubric can
judge it. It is an escalation after a bounded class-aware candidate fails or a
separate frost/canopy requirement makes layer state necessary.

### Accept Floor As Final Snow-Density Answer

Rejected as the density-remediation decision. The current floor is adequate
input for a frost-attribution-threshold process with uncertainty carried
forward, but it does not exploit the independently authoritative, low-cost
regime-divergent candidate.

## Follow-Up

Scaffold `SNOWDENSITY-10.3.22` as a contract-first, opt-in candidate package for
climate-class density specialization. Promotion requires a real cross-SNOTEL
forcing-robust rubric win over the current default and whole-model conservation
closure. Failure closes as non-promotion or hold; no default activation is
implied by this ADR candidate.
