# SNOW-SURFACE-EB-04B Coupled-Dynamics Characterization

Status: `COMPLETE / PASS`

Date: `2026-07-31`

Campaign: `SNOW-SURFACE-EB`

This ExecPlan is a living document maintained under
`docs/codex_exec_plans.md`.

## Purpose / Big Picture

Explain how the three exact EB-04A failure families develop, identify the
proximate mechanism and the remaining authority question for each family, and
give the correction packages a falsifiable evidence base. A reader should be
able to distinguish an invalid upstream thermal state from a conductivity
defect and a layer-bookkeeping defect without rerunning the frozen EB-04
experiment.

## Implementation Intent

Intent: `diagnostic characterization` and `independent reconstruction` only.
This package does not implement process science, perform calibration, score
observations, change runtime behavior, or amend canonical authority.

## Objective

- Reconstruct the complete daily chronology preceding every EB-04A failure.
- Quantify active mass, cold content, diagnosed temperature, sublimation,
  latent cooling, radiation, conduction, refreeze, routing, density, and layer
  geometry as each lane approaches rejection.
- Compare early, middle, and late failures and retain contrary B/L/S/LS cases.
- Test prospectively frozen causal signatures rather than selecting a story
  after plotting results.
- Assign every exact family to EB-04C, EB-04D, an authority predecessor, or a
  defensible model-limitation boundary.
- Publish accessible plot-only SVG figures with complete Markdown sidecars.

## Authority And Protected Boundaries

The governing authority is `SC-SNOWENERGY-001`, especially invariants 017,
019, 021, 023, 024, and 025 and producer/consumer obligations 004, 005, 007,
008, 009, and 011. EB-04 and EB-04A are immutable evidence inputs.

Protected boundaries:

- no equation, coefficient, constant, threshold, tolerance, cadence, forcing,
  fixture, observation, selector, default, parser, or user-schema change;
- no production Rust or canonical-contract edit;
- no rerun, replacement, or rescoring of the frozen EB-04 factorial;
- no clamp, guard relaxation, energy limiter, layer deletion rule, fallback,
  or corrective implementation;
- no calibration, promotion, or causal claim stronger than the retained
  diagnostic evidence supports.

## Prospective Causal Tests

The characterization must test these signatures before disposition:

1. `VANISHING_MASS_RETAINED_COLD_CONTENT`: rejected temperature is explained
   by positive cold content divided by a vanishing active heat capacity.
2. `SUBLIMATION_AMPLIFIED_THIN_PACK_COOLING`: S/LS failures show simultaneous
   mass export and negative latent energy that increase cold content per unit
   remaining mass; a non-sublimation contrary case prevents claiming
   sublimation is necessary.
3. `LOWER_LAYER_DECOUPLED_AT_FAILURE`: the extreme-cold boundary is reached in
   a complete-pack active volume with no lower reservoir able to supply `G_0`,
   or contrary evidence rejects this signature.
4. `NOT_CONDUCTIVITY_CONSTITUTIVE`: the invalid temperature or vapor-pressure
   state occurs before a valid conductivity value can be evaluated.
5. `GEOMETRY_ROUNDOFF_SCALE`: the two aggregate failures conserve SWE and have
   signed depth residuals near the declared `1e-9 m` boundary rather than a
   material mass or density discontinuity.

## Intended Write Set

- this package tree;
- `docs/ROADMAP.md`;
- `docs/planning/snow-surface-energy-balance-roadmap.md`;
- `docs/work-packages/README.md`.

No production or test source edit is intended. Any discovered need for one is
a package-plan amendment and requires reclassification before the edit.

## Inputs And Evidence Identity

- EB-04 frozen `factorial-results.json` and retained 48 trace files;
- EB-04A `diagnostic-replay.json`, 24 current diagnostic traces, typed failure
  snapshots, and independent conservation evidence;
- current executable and executable-source-diff hashes bound by EB-04A;
- canonical `SC-SNOWENERGY-001` authority and static source-line ordering.

The tool must fail closed if any retained input hash, target count, rejection
day, classification, or semantic snapshot differs from EB-04A.

## Deliverables

- deterministic package-local analysis tool and machine-readable result;
- population chronology and boundary-distance tables;
- causal taxonomy and root-cause/authority ledger;
- independent calculation of temperature from mass and cold content;
- early, middle, late, and contrary-case plots with Markdown sidecars;
- exact input/source identity, security, assurance, review, verification, and
  final-disposition evidence.

## Phase Plan

1. Freeze intent, authority, causal signatures, operands, and validation.
2. Implement a deterministic read-only analysis over retained EB-04/04A data.
3. Execute population chronology, independent reconstruction, contrary-case
   analysis, and figure generation.
4. Assign each family to its correction/authority boundary.
5. Complete documentation gates, dual review and finding disposition, dual
   verification, exact-diff reconciliation, and roadmap/catalog disposition.

## Validation Requirements

- analysis-tool compile/import and built-in anti-alias self-check;
- exact input, executable, source-diff, target, day, and classification hashes;
- independent `T = -Q_cc/(rho_w SWE c_i)` reconstruction from typed snapshots;
- all 22 thermal-family and two geometry-family failures classified;
- source-line ordering audit for energy application, mass removal, and guard;
- plot/sidecar one-to-one inventory, SVG parse, and visual inspection;
- scoped Markdown lint, reference checks, placeholder scan, security check, and
  `git diff --check`;
- exact terminal diff proving no executable or contract source changed;
- valid reuse of EB-04A Rust validation only if its executable-source identity
  remains unchanged.

## Exit Criteria

1. All 24 failures have a complete chronology and assigned causal family.
2. Every prospective signature is `SUPPORTED`, `REJECTED`, or `INCONCLUSIVE`
   with quantitative evidence and at least one contrary-case check.
3. The temperature reconstruction independently matches every thermal typed
   snapshot within declared floating-point roundoff.
4. Root-cause claims distinguish proven proximate mechanics, strong
   associations, and unresolved corrective authority.
5. EB-04C and EB-04D receive concrete first actions and authority questions.
6. No protected boundary changes and all current-scope gates pass.
7. Figures are plot-only and each has a complete Markdown sidecar.
8. Dual reviews, finding disposition, and dual verification pass.

Any unmet criterion forces `HOLD`.

## Security Impact Gate

The analysis is local and read-only outside the package output tree. It uses no
network, secret, authentication, unsafe Rust, fixture mutation, or shell-built
subprocess command.

## Calibration Readiness

Science implementation: `NOT_IMPLEMENTED` in this package because no correction
is authorized. Calibration evidence: `NOT_APPLICABLE`. Identifiability:
`NOT_APPLICABLE`. Retained observations remain `DIAGNOSTIC_ONLY` and are not
scored.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent science/code reviewers and two terminal
verification agents for read-only review of source, analysis, artifacts,
figures, gate legitimacy, and disposition. Expected outputs are
`artifacts/review_agent_a.md`, `review_agent_b.md`,
`verification_agent_a.md`, and `verification_agent_b.md`. Write access is
read-only; the primary agent records returned verdicts.

Subagent requirement: no heavy batch/comparator runner is selected because the
package changes documentation and package-local analysis only and does not
execute a population cohort or Rust closure suite.

## Progress

- [x] (2026-07-31) User authorized scaffolding and full execution.
- [x] (2026-07-31) Characterization scope and protected boundaries frozen.
- [x] (2026-07-31) Deterministic analysis and five figures/sidecars complete.
- [x] (2026-07-31) Causal taxonomy and EB-04C/04D authority handoff complete.
- [x] (2026-07-31) Validation, dual review, dual verification, and final
  disposition complete.

## Surprises & Discoveries

- EB-04A already publishes the required daily and hourly operands. No new
  production diagnostic is needed for the declared daily chronology.
- All 22 thermal failures are rejected before the current substep's carrier is
  evaluated. Seventeen temperatures are impossible; five remain above `0 K`
  but underflow inside the SNOBAL vapor-pressure conductivity dependency. The
  exact preceding boundary-crossing driver is not retained.
- Both geometry residuals exactly equal one fragment excluded by an SWE
  threshold whose physical depth exceeds the separately equal-valued depth
  tolerance.

## Decision Log

- Decision: consume retained EB-04 and EB-04A traces without rerunning the
  factorial.
  Rationale: this preserves the one-round stop-loss while allowing a read-only
  successor analysis bound to exact input hashes.
  Date/Author: 2026-07-31 / Codex.
- Decision: keep EB-04B characterization-only.
  Rationale: selecting correction physics requires the causal and authority
  assignment this package is intended to produce.
  Date/Author: 2026-07-31 / Codex.

## Outcomes & Retrospective

The retained population supports one common thermal state signature and one
separate geometry mechanism. Sublimation has a strong prior association and a
mechanistic contribution in 20 thermal cases but is not necessary; terminal
amplification is inconclusive. Seventeen invalid thermal states and five
valid-Kelvin vapor-pressure underflows are assigned to EB-04C; the two
dimensionally inconsistent fragment-filter failures are assigned to EB-04D.
Both successors are independently admitted, with EB-04C recommended first.
Dual review, dual verification, and every selected lifecycle gate pass.
