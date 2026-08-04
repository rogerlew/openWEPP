# SNOW-WET-COMPACTION-OPERAND-AUTHORITY-AND-DUPLICATE-ALIAS-CLOSURE

Status: `queued`

Date: `2026-08-04`

Package ID: `20260804-snow-wet-compaction-operand-authority-duplicate-alias-closure-001`

Plan class: `Defect-Closure ExecPlan (DC-ExecPlan)`

Defect ID: `SNOW-WETCOMPACT-DUP-001`

This is a living ExecPlan governed by `docs/codex_exec_plans.md` and
`docs/defect_closure_execplans.md`. Keep `Progress`, `Surprises & Discoveries`,
`Decision Log`, and `Outcomes & Retrospective` current during execution.

## Purpose / Big Picture

Close the confirmed active snow-density wet-compaction operand alias before
using density, layer geometry, or cold state to diagnose early melt. The
current real consumer supplies
`snowpack_state_loss + routed_melt` to the density process even though the
compact solid-to-liquid ledger proves
`routed_melt = snowpack_state_loss + rain_released`. That makes the current
driver `2 * snowpack_state_loss + rain_released`.

The package must establish the complete physical input to wet compaction,
amend canonical authority, add a regression that distinguishes every plausible
alias, and correct the real consumer if the seven-gate bar is met. It also
materializes a deterministic, development-only Snowbird precipitation-scaled
CLI for future packages while preserving canonical `p8.cli` byte-for-byte.

## Implementation Intent

- Intent: `science implementation and defect closure`, plus one derived
  development fixture.
- Risk: `Critical`; production kernel operand lineage and a cohort fixture are
  affected.
- Calibration evidence: `NOT_APPLICABLE`; no model parameter is fitted.
- Observation role: Snowbird SNOTEL evidence remains `DIAGNOSTIC_ONLY`.
- Protected next process: early-melt energy attribution is not in scope.
- Pre-implementation base: `d41a67c7a2c8d199f9f05f5f309b9b85915e01e1`.

## Context And Orientation

`crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs`
constructs the private daily `SnowCouplingOutcome`, resolves the routed liquid
handoff, and calls the density process. `09_snow_density.rs` applies the
Anderson/SNOBAL-lineage wet-compaction half-saturation formula to bulk and
multilayer snow. The compact mass-transition ledger independently establishes
that routed liquid is the sum of bounded snowpack SWE loss and released rain.

The primary wet-compaction lineage is PySnobal 0.2.3, whose `mass_bal` sequence
applies H2O compaction after precipitation and melt but before runoff. Its
`h2o_compact` input is generated melt plus rain, divided by current snow mass.
The PyPI source archive is
`pysnobal-0.2.3.tar.gz`, SHA-256
`78f97faf0452816038494b9fde332a2c2a14d92ec2e5960378abd7606d82fda2`.
Anderson 1976 supplies the broader wet-snow and melt-metamorphism authority in
`references/copyrighted/noaa_6392_DS1.md`. The pinned WEPP baseline remains
required negative evidence: it owns CoE melt/rain lineage but does not itself
define the later active multilayer SNOBAL wet-compaction operator.

The complete candidate set must remain distinct until the authority milestone:

- current alias: `snowpack_state_loss + routed_melt`;
- routed-only: `routed_melt`;
- loss plus all rain: `snowpack_state_loss + rain_retained + rain_released`;
- generated positive melt plus all snow-contact rain; and
- retained-liquid store level or change.

The expected authoritative candidate from the primary source is generated
positive melt plus all rain presented while snow cover is active, counted once
before runoff. The package must prove that mapping against current hourly state
rather than assume that a convenient daily alias is equivalent.

## Correction Authority Envelope

### Observed violation

`SNOW-WETCOMPACT-DUP-001`: on active snow days, the real density consumer can
receive bounded snowpack SWE loss twice. Retained reconstruction closes
`density_liquid_input / 1000 = 2 * snowpack_swe_loss + rain_released` within
`2.78e-17 m`; the duplicated loss component totals `73.123 m` over the prior
primary windows. This is confirmed data-flow duplication, but the predecessor
correctly withheld a physical-defect verdict until complete operand authority
was established.

### In-scope write set

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/09_snow_density.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/infiltration_reconciliation.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs`
- `crates/openwepp-runner/src/hillslope/snowbench_coe_density.rs`
- `crates/openwepp-runner/src/hillslope/snowbench_coe_melt.rs`
- focused tests under `tests/integration/` and owning module tests
- `tests/fixtures/snotel_observed/snotel_snowbird_ut/development/precip_x1p2155576/`
- `tests/fixtures/snotel_observed/snotel_snowbird_ut/manifest.md`
- `tests/fixtures/snotel_observed/README.md`
- this package tree, `docs/work-packages/README.md`, `docs/ROADMAP.md`, and
  `docs/planning/snow-surface-energy-balance-roadmap.md`

The exact terminal diff may use only the subset necessary to close the defect.
Any newly necessary adjacent surface in the same process family must be added
to this envelope before its first edit and recorded in the Decision Log.

### Allowed production edit classes

- Add an exact private daily operand for liquid H2O added to an active pack by
  generated positive melt and rain before runoff.
- Replace the aliased density call input with that authoritative operand once.
- Add typed finite/nonnegative closure guards and behavior-neutral diagnostics
  needed to reconstruct the new operand.
- Keep offline CoE-bound density replay semantically aligned with the real
  production operand when its input surface can prove the same lineage.

No coefficient, cap, selector, default, phase, melt-energy term, canopy,
radiation, frost process, public output schema, or empirical threshold may
change.

### Acceptance criteria

1. Contract authority names the complete driver, units, chronology, aliases,
   and exact-one rule before production edits.
2. A contract-derived test fails on the current formula and distinguishes all
   listed wrong candidates numerically.
3. The real consumer receives generated positive melt plus snow-contact rain
   exactly once before runoff; state-loss and routed-liquid aliases cannot be
   substituted.
4. Snow SWE, routed liquid, compact-ledger closure, density-process closure,
   layer aggregate closure, and Stage-3 mass closure remain within their
   existing tolerances.
5. Canonical four-site runs and canonical-versus-scaled Snowbird lanes remain
   separately labeled. The scaled lane cannot prove the correction.
6. The derived Snowbird CLI changes only daily precipitation, using exact
   decimal factor `1.2155576` and `0.1 mm` half-up rounding; canonical `p8.cli`
   retains its scaffold hash.
7. Direct focused, quick, frost, Critical full, anti-evasion, format, Clippy,
   doctest, and documentation requirements selected by the terminal diff pass.

### Protected boundaries

- Early-melt timing/energy attribution belongs to queued 21L and may not be
  started here.
- Snowbird scaling is development normalization only. It may not alter the
  canonical fixture, prove physics, create forcing truth, or support
  transferability/default claims.
- Do not change observations, PRCPSA, SNOTEL values, phase parameters, melt
  coefficients, energy coefficients, density constants, the `522 kg m^-3`
  cap, public schemas, or default selectors.
- Legacy comparator agreement is A5 investigation evidence, not the target.

## Conversion Rule And Seven-Gate Bar

If reproduction, named mechanism, ownership, authority, safety, testability,
and measurable validation all pass inside this envelope, this package must
amend the contract, add contract-derived tests, record the pre-implementation
gate, implement the real correction, validate it, and finish review and
verification. Further possible study is not a reason to stop.

The seven gates are prospectively bound as follows:

1. Reproduction: retained exact alias identity plus a fresh focused synthetic
   anti-alias case.
2. Mechanism: private density handoff combines overlapping lineage stages.
3. Ownership: the caller, density input, compact ledger, and offline mirror are
   inside this envelope.
4. Authority: canonical `SC-SNOWFREEZE-001`, Anderson 1976, PySnobal 0.2.3,
   and pinned WEPP melt/rain lineage.
5. Safety: no clamp, silent default, fitted coefficient, surrogate physics, or
   weakening of an existing guard.
6. Testability: deliberately non-equal generated melt, retained rain, released
   rain, state loss, routed handoff, and store-change values.
7. Validation: exact operand reconstruction, real-consumer proof, conservation,
   density/layer effects, and canonical/scaled lane separation.

## HOLD Legitimacy

`HOLD` is exceptional. It is permitted only if primary and canonical authority
remain missing or contradictory, the correct mechanism is proven outside this
envelope or in another process family, required evidence cannot be produced in
the environment, or the observed input is invalid and the existing typed
failure is correct. Diagnostic uncertainty, implementation effort, diff size,
or a partial compatibility path are not boundaries.

Before any HOLD, write `artifacts/hold-legitimacy-audit.md` naming the boundary,
citing proof, listing the in-envelope correction route considered, and
explaining why it cannot close now. The first handoff item must be
`close defect SNOW-WETCOMPACT-DUP-001`, never another tracing step.

## Conservation / Output Acceptance

Before production edits, `artifacts/operand-lineage.md` must record every
candidate's units, timing, normalization, source, and authoritative or
diagnostic status. Tests must make each wrong formula produce a different
number. Acceptance reconstructs the authoritative operand from independent
hourly generated-melt and rain components and checks the real downstream
density consumer. Self-consistency, a one-sided bound, or compact-ledger
closure alone is supporting evidence only.

## Contract-First Phase Plan

### Milestone 1 — Freeze authority and derived-fixture custody

Record exact current, prior, primary-source, contract, and pinned-baseline
lineage. Freeze canonical `p8.cli` identity, the deterministic transform, the
derived path, rounding, hashes, and consumer protocol. Materialize the derived
CLI without changing the canonical file.

### Milestone 2 — Amend contract and add the failing regression

Amend `SC-SNOWFREEZE-001` to define the complete wet-compaction input and
exact-one chronology. Add anti-alias tests before changing production. Record
the expected pre-fix failure in `artifacts/pre-implementation-contract-gate.md`.

### Milestone 3 — Correct the real producer-to-consumer path

Add the minimum exact private operand and pass it once to bulk and multilayer
density. Align the diagnostic/offline mirror only where it can consume the same
lineage. Prove the real downstream consumer no longer reads the aliased sum.

### Milestone 4 — Validate materiality and nonregression

Run focused closure first, then canonical four-site and separately labeled
Snowbird development-lane evidence. Reconstruct the operand and mass/density/
layer ledgers independently. Execute all terminal-diff requirements directly.

### Milestone 5 — Review, disposition, verify, and close

Complete two independent reviews, explicit finding disposition, two independent
terminal verifications, line-count governance, exact-diff reconciliation,
security impact, worker handoff, roadmap/catalog updates, and disposition.

## Required Validation

The pre-implementation and terminal artifacts must record exact commands,
working directory, source identity, duration for expensive runs, exit status,
and log/output paths. At minimum select and run:

- focused contract/unit/integration tests for the new operand and derived CLI;
- `cargo fmt --all --check`;
- warnings-denied Clippy for affected crates/tests;
- quick and frost direct profiles;
- Critical full-workspace correctness and doctests;
- `bash tools/release/check_authority_suite_antievasion.sh`;
- `cargo nextest run --test auth11_required_suite_obligation_guards_contract`;
- Markdown lint and link/path checks for touched docs; and
- `git diff --check` plus exact terminal diff/write-set reconciliation.

`cargo deny check` is required only if the terminal diff changes manifest,
lockfile, dependency, license, or source policy surfaces.

## Subagent Authorization And Requirements

Subagent authorization: this package explicitly authorizes spawning/delegating
to bounded read-only authority investigators, two independent scientific/Rust
reviewers, two independent terminal verifiers, and the
`comparator_suite_runner`. Investigator outputs are compact source-line findings
returned to the parent; reviewers and verifiers may write only their named
package artifacts; the comparator runner may write logs and generated `target/`
evidence but no production, contract, test, fixture, roadmap, or package prose.

Subagent requirement: REQUIRED. Spawn `comparator_suite_runner` for all heavy
batch, canonical four-site, Critical full-workspace, doctest, comparator, and
cohort runs. The parent must not execute those heavy runs unless the subagent is
genuinely unavailable, in which case command-level failure evidence is required
before local fallback. Use `rust_code_reviewer` and `rust_qa_reviewer` as the
primary and secondary Rust correctness gates after implementation.

## Progress

- [x] (2026-08-04) User authorized 21K after the defect-first roadmap update.
- [x] (2026-08-04) Initial current-flow and PySnobal 0.2.3 source intake
  identified the exact duplicate and the pre-runoff generated-melt-plus-rain
  candidate.
- [ ] Commit the complete scaffold before implementation edits.
- [ ] Freeze authority, operand lineage, and derived-fixture protocol.
- [ ] Materialize and verify the development-only Snowbird CLI.
- [ ] Amend the canonical contract and add a pre-fix failing regression.
- [ ] Implement the authority-backed correction and real-consumer proof.
- [ ] Run focused and heavy validation.
- [ ] Complete dual review, finding disposition, dual verification, and closure.

## Surprises & Discoveries

- Observation: routed melt is not an independent wet-compaction source; the
  compact ledger defines it as state loss plus released rain.
  Evidence: current `DirectSnowSolidToLiquidLedger` validation and the retained
  reconstruction within `2.78e-17 m`.
- Observation: primary PySnobal calls wet compaction before runoff and bases it
  on generated melt plus rain, so routed-only is also not automatically the
  complete physical operand.
  Evidence: PySnobal 0.2.3 `mass_bal` and `h2o_compact` source intake.

## Decision Log

- Decision: Treat 21K as a DC-ExecPlan, not a diagnostic-only audit.
  Rationale: the duplicate is reproduced and its most plausible correction
  surfaces share one authority and validation domain.
  Date/Author: 2026-08-04 / Codex.
- Decision: Keep Snowbird scaling as a separate derived fixture lane inside
  the same package but prohibit it from carrying physics acceptance.
  Rationale: the user directed future packages to use the normalized lane, and
  materializing it now prevents result-aware transformations in 21L.
  Date/Author: 2026-08-04 / Codex.

## Outcomes & Retrospective

Queued. Replace this text with the terminal correction/non-defect/HOLD outcome,
scientific consequences, and the exact 21L entry condition.

## Idempotence And Recovery

The CLI generator must be deterministic and refuse a source-hash mismatch.
Generated target runs use fresh package-specific directories. If a gate fails,
preserve the failure and correct the owned surface; do not rerun semantic
failures into passes. Canonical `p8.cli` is immutable and recoverable from Git.

Revision note (2026-08-04): initial 21K scaffold authored from the defect-first
roadmap, confirmed alias evidence, and primary wet-compaction source intake.
