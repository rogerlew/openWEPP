# SNOW-SURFACE-EB-04R Fresh Factorial Execution And Adjudication

Status: `executed / HOLD / terminally verified`

Date: `2026-08-01`

Campaign: `SNOW-SURFACE-EB`

Plan class: `Campaign checkpoint / independent-validation ExecPlan`

This living ExecPlan follows `docs/codex_exec_plans.md`.

## Purpose / Big Picture

Run the corrected snow-energy candidate as one fresh, prospectively frozen
scientific experiment after EB-04C/04D closed the runtime defects and EB-04E
qualified the complete population. EB-04R determines whether combined
sub-canopy longwave and sublimation earns promotion assessment under the
original observation rubric and protected-group rule. It does not tune the
model or reinterpret the failed EB-04 round.

## Objective

Execute exactly 12 lanes by four B/L/S/LS cells through the production direct
hillslope consumer. Before every subprocess, remove all inherited
`OPENWEPP_*` variables, install only the frozen model selectors and trace path,
and serialize the effective selector environment into result-bearing
provenance. Independently verify WAT/trace/layer and physical ledgers before
scoring the unchanged observation rubric, then apply the frozen promotion rule
once and publish accessible figures with Markdown sidecars.

## Implementation Intent

Intent: `independent-validation` plus deterministic factorial adjudication.

Calibration intent: `NOT_APPLICABLE`. No coefficient, forcing, observation,
rubric, threshold, fixture, process equation, or default is fitted or changed.

## Prospective Boundary

The complete pre-result protocol is
[`artifacts/prospective-decision-protocol.md`](artifacts/prospective-decision-protocol.md).
It must be hash-bound with the package tool, source commit, executable, frozen
EB-04 authority, and EB-04E qualification before the first result-bearing run.

Included:

- the immutable EB-04 12-lane population and B/L/S/LS meanings;
- unchanged observation roles and `INV-SNOWFREEZE-050` rubric;
- corrected `SC-SNOWENERGY-001` v5 runtime at committed source;
- package-local sanitized launcher, provenance records, independent consumer,
  scoring, tables, SVG figures, and Markdown sidecars;
- roadmap, catalog, and package evidence.

Excluded:

- production Rust, tests, canonical contracts, fixtures, observations,
  forcing, coefficients, selectors, thresholds, rubrics, or defaults;
- empirical fitting, rescaling, imputation, observation leakage, post-result
  operator changes, retries of scientific failures, or a third experiment;
- warm-maritime conifer transfer claims without paired observations.

## Dependencies

- immutable EB-04 decision protocol and observation-role assignment;
- EB-04C thermal-domain correction;
- EB-04D represented-layer correction;
- EB-04E 48-cell runtime qualification and provenance prerequisite;
- `SC-SNOWFREEZE-001` invariant 050;
- `SC-SNOWENERGY-001` v5.

## Intended Write Set

- this package tree;
- `docs/ROADMAP.md`;
- `docs/planning/snow-surface-energy-balance-roadmap.md`;
- `docs/work-packages/README.md`.

Production Rust, tests, contracts, fixtures, observations, and prior package
evidence are read-only. Any need outside this write set forces a prospective
amendment before editing.

## Environment And Provenance Acceptance

Each cell must prove that the subprocess inherited no ambient `OPENWEPP_*`
state. The launcher copies non-OpenWEPP ambient variables, removes every key
whose name begins `OPENWEPP_`, then installs exactly:

- four frozen non-target model selectors;
- the cell's longwave selector;
- the cell's sublimation selector; and
- the cell-specific opt-in research trace path.

Per-cell provenance must bind the removed key names, exact effective
`OPENWEPP_*` mapping, argv, source commit, executable hash, tool/protocol hash,
fixture/runfile, timestamps, return code, runtime manifest, WAT, trace, stdout,
and stderr. Values of removed ambient variables are never recorded.

## Conservation / Publication Acceptance

Before observation scoring, the package-local independent consumer must prove:

- exact 12x4 inventory and sequential trace/WAT chronology;
- WAT SWE and depth against runtime aggregates and complete layer vectors to
  `<=1e-9 m`;
- daily snow mass closure to `<=1e-9 m`;
- surface and cold-content energy closure to `<=1e-6 J m^-2`;
- daily/hourly shortwave, longwave, latent, vapor, and latent/mass identities;
- every numeric trace operand finite and every hourly vector exactly length 24;
- layer count, density, temperature, cold-content, and aggregate coupling;
- real longwave and sublimation reach in all 24 enabled cells and zero target
  mechanism reach in disabled cells; and
- rejection of fragment deletion, aggregate-only substitution, wrong selector,
  wrong sign, and omitted-energy aliases.

Producer summaries are corroborating evidence only. A failed physical or
provenance gate excludes scoring and forces the package to `HOLD`.

## Phase Plan

1. Scaffold, map authority, freeze population, identities, result operators,
   environment sanitizer, decision thresholds, gates, and claim limits.
2. Implement and self-check the package-local launcher/consumer/figure tool.
3. Build the exact runner and execute every prescribed cell exactly once.
4. Independently audit retained outputs, score unchanged observations, compute
   effects/interactions, and apply the frozen decision rule.
5. Run selected validation, dual review, finding disposition, dual terminal
   verification, roadmap/catalog updates, and final disposition.

## Acceptance Criteria

1. Protocol and tool are frozen before execution.
2. Exactly 48 unique cells are attempted once; no semantic/scientific retry
   occurs.
3. Every cell has complete promotion-grade environment and file provenance.
4. All physical and independent consumer gates pass before any score is used.
5. All observation scores, effects, interactions, protected groups, contrary
   evidence, uncertainty, and unavailable claims are retained.
6. The decision follows the frozen eight-part rule without post-result change.
7. Every SVG has one readable Markdown sidecar.
8. Selected gates, dual review, finding disposition, dual verification,
   exact-diff, security, and final handoff pass.

Any unmet current-scope criterion forces `HOLD`; empirical failure with valid
evidence closes as `CLOSE_NONPROMOTION`, not as an implementation defect.

## Validation Requirements

- package-tool syntax, sanitizer/provenance self-check, anti-alias controls,
  and deterministic analysis regeneration;
- exact one-time 48-cell production-direct execution;
- independent physical, WAT, layer, and scoring reconstruction;
- focused snow-energy contract/runtime tests and authority guards;
- `cargo fmt --all -- --check` and strict workspace Clippy;
- workspace quick and frost profiles, with exact unchanged evidence reusable
  under the canonical testing strategy;
- fresh Critical full workspace because this is a campaign checkpoint;
- scoped Markdown, SVG, inventory, exact-diff, security, and line-count checks.

## Security Impact Gate

No network, secret, authentication, unsafe Rust, dependency, external write,
or public schema change is authorized. Removed environment values are not
serialized. Subprocesses use explicit argument arrays and write only beneath
`target/snow_surface_eb04r_factorial/`.

## Subagent Authorization And Requirement

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to one `comparator_suite_runner` for the 48-cell population
and fresh Critical full gate, two independent read-only science/code reviewers,
and two read-only terminal verification agents. Expected outputs are compact
execution metrics/log paths plus package-recorded review and verification
artifacts. No subagent may edit production, tests, contracts, fixtures,
observations, or prior packages.

Subagent requirement: REQUIRED for the population batch and Critical full
profile while the authorized runner is available.

## Progress

- [x] (2026-08-01) User authorized environment sanitization, scaffolding, and
  end-to-end EB-04R execution.
- [x] (2026-08-01) Protocol, source, executable, population, and tool frozen
  after two independent `PASS_TO_FREEZE` reviews.
- [x] (2026-08-01) Exact 48-cell experiment executed once: 48/48 subprocesses
  and all environment/file provenance gates passed.
- [x] (2026-08-01) Terminal review found 12 cells fail the frozen
  vapor-to-sublimation tolerance because the tool used a looser contradictory
  threshold. Observation scoring is inadmissible; outcome is
  `HOLD_PHYSICAL_OR_PROVENANCE_GATE`.
- [x] (2026-08-01) Validation, dual terminal review, dual verification,
  roadmap/catalog, and final disposition complete. Fresh Critical full passed
  2,177/2,177; frozen science gate remains authoritative HOLD.

## Surprises & Discoveries

- The first `--execute` invocation stopped before creating an attempt or
  running a cell because JSON serialized selector tuples as arrays while the
  in-memory freeze comparison retained tuples. The same representation defect
  would have rejected later attempt analysis. The tool now uses one explicit
  JSON-native cell mapping for freeze, attempt, and validation. At that first
  rejection no attempt existed and the run inventory was empty; two focused
  reviews passed before the later single result-bearing execution.
- Terminal review found the frozen protocol transcribed the vapor-to-
  sublimation tolerance as `1e-9 kg m^-2`, while the imported consumer retained
  `1e-6 kg m^-2` (dimensionally corresponding to `1e-9 m` SWE). Twelve cells
  fall between those thresholds. The explicit frozen text controls despite the
  likely unit error, so the observation phase is inadmissible and the package
  holds without a rerun.

## Decision Log

- Decision: preserve the original EB-04 observation rubric, timing operators,
  protected groups, and eight-part promotion rule.
  Rationale: EB-04C/04D corrected runtime defects without fitting to observation
  scores; changing the decision rule would create result-aware drift.
  Date/Author: 2026-08-01 / Codex.
- Decision: strip all inherited `OPENWEPP_*` state and serialize only key names
  for removed variables.
  Rationale: this closes provenance without leaking ambient values or requiring
  users to supply new coefficients.
  Date/Author: 2026-08-01 / Codex.

## Outcomes & Retrospective

The corrected runtime made the entire execution design available: 48/48 cells
completed, both mechanisms reached all 24 enabled cells, disabled paths stayed
clean, and independent source, selector, WAT/trace/layer, conservation,
finiteness, signed-mechanism, and decision reconstructions completed under the
implemented consumer thresholds.

Terminal review found that the frozen protocol requires vapor-to-sublimation
closure at `1e-9 kg m^-2`, while the consumer used `1e-6 kg m^-2`. Twelve cells
exceed the frozen bound, with a maximum `8.109983287707401e-8 kg m^-2`.
Therefore the physical gate failed before observation access should have been
allowed. The generated ordinal scores and nonpromotion decision remain only
diagnostic post-gate-leakage evidence. The truthful outcome is
`HOLD_PHYSICAL_OR_PROVENANCE_GATE`; no rerun is authorized under EB-04R and
EB-05 remains deferred pending separate authority/process adjudication.
