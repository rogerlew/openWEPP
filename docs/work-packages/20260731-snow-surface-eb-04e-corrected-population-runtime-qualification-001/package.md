# SNOW-SURFACE-EB-04E Corrected-Population Runtime Qualification

Status: `complete / pass`

Date: `2026-07-31`

Campaign: `SNOW-SURFACE-EB`

Plan class: `Campaign checkpoint / characterization-only ExecPlan`

This living ExecPlan follows `docs/codex_exec_plans.md`.

## Purpose / Big Picture

Qualify the EB-04C/04D corrected snow-energy candidate across the complete
population that exposed the first-round failures. EB-04E establishes runtime
and physical-ledger admissibility before any new empirical experiment. It is
not a second factorial, calibration round, or promotion adjudication.

## Objective

Execute all 48 prescribed B/L/S/LS cells once using the exact current runner,
the complete EB-04A research trace, unchanged fixtures/selectors, and a
package-local independent consumer. Prove trace identity, daily mass and energy
closure, signed latent/mass identity, thermal-domain safety, complete
layer-vector reconciliation, and real WAT/trace consumption. Characterize
branch use and mechanism magnitudes without observation scoring.

## Implementation Intent

Intent: `runtime qualification and physical characterization`.

Calibration intent: `NOT_APPLICABLE`. No observations are read for scoring, no
coefficient is fitted, and no interaction or promotion rule is evaluated.

## Frozen Qualification Boundary

The pre-result protocol is
[`artifacts/prospective-qualification-protocol.md`](artifacts/prospective-qualification-protocol.md).
It must be complete and hash-bound before the first result-bearing run.

Included:

- the immutable 12-lane, four-cell EB-04 population;
- existing B/L/S/LS selectors and non-target environment;
- package-local runner, audits, tables, SVG plots, and Markdown sidecars;
- exact current debug `openwepp-cli-hill` binary built before execution;
- current trace fields admitted by EB-04A–04D;
- roadmap, catalog, and package evidence.

Excluded:

- observations, rubrics, scores, factorial effects, interactions, calibration,
  coefficient search, defaults, promotion, or EB-04 reinterpretation;
- process-physics, contract, fixture, forcing, selector, threshold, schema, or
  runtime-code changes;
- retries of semantic/scientific failures.

## Dependencies

- frozen EB-04 population and selector protocol;
- EB-04A complete diagnostic operands;
- EB-04C thermal-domain closure;
- EB-04D layer-lifecycle closure;
- `SC-SNOWENERGY-001` v5.

## Intended Write Set

- this package tree;
- `docs/ROADMAP.md`;
- `docs/planning/snow-surface-energy-balance-roadmap.md`;
- `docs/work-packages/README.md`.

Production Rust, tests, canonical contracts, fixtures, observations, and prior
package evidence are read-only. Any executable-source need forces a prospective
package amendment before an edit and changes this package's qualification-only
posture.

## Conservation / Publication Acceptance

The package-local consumer must independently reconstruct from serialized
operands:

- daily snow mass balance to `<=1e-9 m` SWE;
- `shortwave + longwave + latent - applied - unused` to `<=1e-6 J m^-2`;
- 24-hour latent flux versus signed mass and effective latent heat using a
  roundoff-aware allowance no weaker than `1e-6 J m^-2`;
- daily/hourly shortwave, longwave, latent-energy, and vapor-mass totals;
- signed vapor mass versus sublimated-ice debit to `<=1e-6 kg m^-2`;
- cold-content change from published surface, conduction, refreeze, and export
  operands to `<=1e-6 J m^-2`;
- every after-layer SWE and depth aggregate from the complete eight-field layer
  vectors, including represented subnanometer-SWE fragments.

Anti-alias self-checks must reject wrong latent sign, wrong latent-heat pairing,
omitted unused energy, wrong layer unit, and aggregate-only substitution.
Exact producer residuals are corroborating evidence only.

## Phase Plan

1. Scaffold, map authority, freeze identities, population, gates, and claim
   limits before result execution.
2. Implement and self-check the package-local qualification/figure tool.
3. Build the exact runner and execute every prescribed cell once.
4. Audit complete inventory, trace/WAT identity, physical ledgers, thermal
   state, layer vectors, and process-boundary chronology.
5. Run selected validation, dual review, finding disposition, dual terminal
   verification, roadmap/catalog updates, and final disposition.

## Acceptance Criteria

1. Exactly 12 lanes and 48 cells are attempted; every cell completes with
   return code zero and complete WAT/trace output.
2. Every within-lane fixture and non-target selector identity check passes.
3. Every mass, energy, latent/mass, hourly aggregation, thermal, and layer
   reconstruction gate passes for every trace row.
4. Day indexes are exact sequential `0..N-1`; trace and WAT row counts agree.
5. EB-04's 24 formerly failing lane/cells all complete and are explicitly
   identified in the result.
6. B/L/S/LS mechanisms reach their intended real consumer and relevant
   mechanism/branch diagnostics are nonzero where physically active.
7. No observation file is loaded for scoring; no empirical, factorial-effect,
   interaction, calibration, or promotion result is emitted.
8. Tool regeneration is deterministic for retained artifacts, with SVG and
   sidecar one-to-one inventory.
9. Selected gates, dual review, finding disposition, dual verification,
   exact-diff, security, and final handoff pass.

Any unmet current-scope criterion forces `HOLD`; EB-04R is not admitted.

## Validation Requirements

- package tool syntax/self-check and deterministic analysis regeneration;
- exact release of one hash-bound 48-cell population run;
- independent physical and layer reconstruction;
- focused EB-04C/04D contract/runtime tests and authority guards;
- `cargo fmt --all -- --check` and strict workspace Clippy;
- workspace quick and frost profiles;
- Critical full workspace because EB-04E is a campaign checkpoint;
- scoped Markdown, SVG parse/inventory, exact-diff, security, and line-count
  checks.

## Security Impact Gate

No network, secret, authentication, unsafe Rust, dependency, external write,
or public schema change is authorized. Model subprocesses use explicit argument
arrays. Transient output is confined to `target/snow_surface_eb04e_qualification/`.

## Subagent Authorization And Requirement

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to one `comparator_suite_runner` for the 48-cell population
batch and Critical full-workspace gate, two independent read-only reviewers,
and two read-only terminal verification agents. Expected outputs are compact
metrics/log paths and package-recorded review/verification artifacts. No
subagent may edit production, tests, contracts, fixtures, or prior packages.

Subagent requirement: REQUIRED for the population batch and Critical full
profile. The parent must not run those heavy commands while the authorized
runner is available.

## Progress

- [x] (2026-07-31) User authorized scaffold and end-to-end execution.
- [x] (2026-07-31) Protocol and package-local tool frozen before result execution.
- [x] (2026-07-31) Exact 48-cell population executed and independently audited.
- [x] (2026-07-31) Selected validation and dual review/verification complete.
- [x] (2026-07-31) Roadmap/catalog and final disposition complete.

## Surprises & Discoveries

- All 48 corrected cells complete. This includes the 24 exact EB-04 failures,
  so the corrected paths qualify population-wide rather than only in the two
  targeted closure cohorts.
- Every cell exercises EB-04C thermal-domain suspension and lower-volume
  collapse somewhere in its trajectory. Four cells also serialize represented
  subnanometer-SWE layers, totaling 30 occurrences; EB-04D is therefore a real
  population path rather than an isolated synthetic boundary.
- The population minimum serialized layer temperature is `-38.556 deg C`,
  safely inside the canonical thermal domain. This is a domain result, not an
  empirical temperature-validation claim.
- The runner manifests bind WAT and executable provenance but not the opt-in
  research trace or complete runtime environment. EB-04E closes trace identity
  with an explicitly forensic timestamp/hash seal. EB-04R must strengthen this
  to promotion-grade selector/environment serialization before execution.

## Decision Log

- Decision: EB-04E reads no observations and computes no factorial contrasts.
  Rationale: corrected-population physical admissibility must precede a new
  prospectively frozen empirical experiment.
  Date/Author: 2026-07-31 / Codex.

## Outcomes & Retrospective

The one-time matrix executed 761,212 daily WAT/trace rows across 48 complete
cells. All physical, chronology, output-identity, former-failure, and mechanism
reach gates pass. Deterministic analysis regeneration is byte-identical.
Dual review and terminal verification pass. Focused gates, authority guards,
formatting, exact-evidence-reused quick/frost/Clippy, scoped checks, and the
fresh 2,177-test Critical full profile all pass. EB-04R is admitted for fresh
prospective scaffolding after its runtime-provenance prerequisite is built into
the protocol.
