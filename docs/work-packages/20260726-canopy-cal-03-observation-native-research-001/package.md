# CANOPY-CAL-03 Observation Corpus, Native Fixtures, And Research Outputs

Package ID: `20260726-canopy-cal-03-observation-native-research-001`

Status: `COMPLETE / CAL-04 AND AFFECTED CAL-05 AUTHORITY-BLOCKED`

Date opened: `2026-07-26`

Execution mode: `package-end-to-end`

Package type: pre-calibration scientific evidence and observability.

This ExecPlan is a living document maintained under
`docs/codex_exec_plans.md`. Progress, discoveries, decisions, and outcomes must
remain current during execution.

## Purpose

Turn the canopy campaign's retained field authority, native management inputs,
real production consumers, and Bill Elliot's stock-flow methodology into a
deterministically rebuildable pre-calibration evidence system. A fresh agent
must be able to rerun the selected Marcell, Harvard, and Hubbard Brook lanes,
rebuild daily records and annual diagnostics, and trace every value without
conversation history.

This package changes no calibrated value, process equation, or public output
schema.

## Progress

- [x] Read the roadmap, CAL-01 authority ledger, CAL-02 handoff, controlling
  science contracts, fixture guidance, and production canopy consumer path.
- [x] Freeze the prospective write set, operand lineage, fixture inventory,
  observation roles, output boundary, and gate plan before implementation.
- [x] Install the provenance-bound observation corpus and immutable role ledger.
- [x] Generate native YAML/runfile counterparts and prove protected bindings.
- [x] Implement the campaign-confined production research trace and tests.
- [x] Execute retained lanes and generate deterministic daily/annual/cohort
  evidence.
- [x] Complete selected gates and dual independent reviews with finding
  disposition.
- [x] Complete dual
  verification, and final disposition.

## Objective

Deliver every item in roadmap Section 6.1: an authority-classified observation
corpus, frozen calibration/holdout assignments, native YAML counterparts for
all source-supplied Marcell/Harvard/Hubbard canopy-gradient lanes, a stable
campaign-confined daily research trace read from real production consumers,
Bill-method annual and shadow-cohort diagnostics, and the frozen CAL-04/CAL-05
protocol.

## Claim Boundary

- `OBSERVATION` values can carry calibration or holdout authority only when
  their retained source and applicability support that use.
- `FITTED_OPERAND`, `DERIVED_DIAGNOSTIC`, `LEGACY_COMPARISON`, and
  `MODEL_OUTPUT` never become independent field observations.
- CAL-02's results characterize Bill's mechanism and continuity only.
- Existing snow observations remain downstream holdouts; they do not select
  canopy parameters.
- No parameter fitting occurs here. Absence of an independent measurement is a
  recorded gap, never permission to promote context or model output.

## Declared Write Set

- `docs/work-packages/README.md`
- `docs/planning/canopy-phenology-assurance-roadmap.md`
- `docs/work-packages/20260726-canopy-cal-03-observation-native-research-001/**`
- `tests/fixtures/cancov_forest/**`
- `tools/canopy_phenology/**`
- `crates/openwepp-runner/src/hillslope/03_tests.rs`
- `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs`
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00c_day_input_builder_impl.rs`

Science contracts, plant/residue physics, management schema, parameter
defaults, Cargo manifests, public output schemas, CAL-01/CAL-02 evidence, and
source fixture forcing/soil/slope/initial-state files are read-only. Amend this
plan prospectively before crossing that boundary.

## Required Deliverables

- `artifacts/required-reading-map.md`
- `artifacts/intent-plan.md`
- `artifacts/operand-lineage.md`
- `artifacts/observation-inventory.csv`
- `artifacts/calibration-holdout-ledger.csv`
- `artifacts/fixture-pair-manifest.json`
- `artifacts/research-output-schema.md`
- `artifacts/consumer-lineage.md`
- `artifacts/pre-calibration-protocol.md`
- retained tidy daily output or a checksum-bound partition manifest
- `artifacts/annual-diagnostics.csv`
- `artifacts/cohort-ledger-summary.csv`
- `artifacts/run-manifest.json`
- `artifacts/rebuild-evidence.md`
- `artifacts/gate-evidence.md`
- two independent review artifacts
- `artifacts/finding-disposition.md`
- two independent verification artifacts
- `artifacts/final-disposition.md`

Generated run directories and large daily traces remain outside Git. The
manifest, compact representative records, summaries, commands, identities, and
checksums retained here must be sufficient to reproduce them.

## Implementation Plan

1. Normalize exact retained source observations and metadata without inventing
   precision; freeze calibration, process-holdout, downstream-holdout, context,
   and excluded roles.
2. Use `openwepp-landuse-migrate` to generate native YAML from each selected
   legacy management. Derive TOML runfiles by changing only the management
   binding. Machine-compare all protected bindings.
3. Add an opt-in, campaign-confined JSONL research trace selected by
   `OPENWEPP_CANOPY_RESEARCH_TRACE_PATH`. Capture the full native daily GSI and
   canopy realization at the producer and the exact values consumed by
   interception, ET, snow, frost, decomposition, runoff, and erosion. The
   default path remains unchanged.
4. Add deterministic tooling to validate/normalize the trace, compute annual
   leaf-on/off, net, amplitude, churn, residue equilibrium/range/drift, and
   analysis-only current/previous/old cohorts from the exact litter source and
   declared decay.
5. Run all native forest lanes and open controls. Retain compact evidence and
   checksum-bound generated-object identities.
6. Reconcile the terminal diff, run selected gates, then obtain two independent
   reviews and two independent verifications.

## Gates

- migration and fixture protected-binding checks;
- focused runner tests proving default-off behavior, stable schema, error
  handling, and real producer-to-consumer values;
- deterministic analysis-tool tests including rejected aliases and mass/cohort
  closure;
- exact selected-lane CLI execution and deterministic rebuild;
- `cargo nextest run -p openwepp-runner` focused to the native canopy research
  tests plus directly affected management/runner tests;
- authority anti-evasion guards because observation bindings are added;
- documentation lint, `git diff --check`, write-set reconciliation, generated
  debris/credential scans, and line-count disposition;
- dual independent scientific/code reviews, explicit finding disposition, and
  dual terminal verification.

Broad full-workspace correctness, coverage, CRAP, and release qualification are
not selected: this is an opt-in diagnostic plus fixtures/tooling, not a
critical physics or public-schema change. Any terminal diff that changes those
facts requires prospective gate escalation.

## Review And Delegation Authorization

Subagent authorization: this package explicitly authorizes spawning/delegating
the required independent reviews, terminal verifications, and bounded heavy
fixture execution. Expected outputs are two review verdicts, two verification
verdicts, and one matrix execution receipt. Write access is package artifacts
only when the primary executor expressly assigns an owned artifact; reviewers,
verifiers, and the matrix executor otherwise have read-only repository access.

The package requires two independent reviews and two independent terminal
verifications. The executing agent is explicitly authorized to spawn and
delegate to subagents for those bounded review, verification, and heavy
fixture-execution tasks. Reviewers must inspect authority/role legitimacy,
protected fixture bindings, real production-consumer lineage, mass/cohort
closure, default-off behavior, and the no-calibration/no-new-physics boundary.

## Decision Log

- `2026-07-26`: Selected a campaign-confined JSONL surface rather than a public
  schema. It exposes observability without creating durable product API.
- `2026-07-26`: Native YAML is generated by the repository migrator; manual
  translation is not acceptable.
- `2026-07-26`: Open controls receive paired native runfiles but emit null
  canopy-process values only conceptually; because no native forest process
  exists, the campaign trace is absent and classified `NOT_APPLICABLE`.
  Missing source strata are not manufactured.
- `2026-07-26`: CAL-02 values retain `LEGACY_COMPARISON`; existing snow
  observations retain downstream-holdout roles and cannot fit canopy operands.

## Outcomes And Retrospective

The observation corpus, paired fixtures, trace, diagnostics, and protocol are
implemented and executed. All nine lanes pass. Exact daily foliar and
equivalent cohort/aggregate ledgers close for all seven forest lanes.

The package retains three authority limits: no independent quantitative
phenology-timing holdout, no evidence-derived probability priors, and no
site-matched litter material composition. These do not invalidate CAL-03's
evidence/observability delivery, but they block CAL-04 fitting and the affected
CAL-05 adequacy/fitting claims until prospectively admitted evidence exists.
