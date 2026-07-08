# WA Day-1122 High-Resolution Active Router Numerics Investigation

Status: EXECUTED-HOLD-ACTIVE-ROUTER-CLAMP-NUMERICS

## Objective

Diagnose the `wa_cascades_forest_h1` high-resolution active-router failure
that blocked the Tier-2 target-`dx` mesh-policy adjudication. The package must
separate the day-1122 fine-rung active day cascade residual from the larger
`dx10/dx5` clamp/storage/outlet amplification, classify the condition, and
state whether any production-default Lane D active behavior is affected.

If the blocker cannot be closed without a production numerics change, the
package must stop at an evidence-backed hold and name the first actionable
follow-on. It must not change the production mesh policy.

## Required Reading

Core:
- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-rescope-001/package.md`
- `docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-rescope-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-rescope-001/artifacts/mesh-ladder-summary.md`
- `docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-rescope-001/artifacts/mesh-ladder-summary.json`
- `docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-rescope-001/artifacts/mesh-fidelity-adjudication.md`

Code and data surfaces:
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/laned_active.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs`
- `crates/openwepp-runner/src/hillslope/laned_active.rs`
- `docs/work-packages/20260707-laned-router-d16-rowcrop-canhgt-active-runtime-publication-001/artifacts/selected-cohort-materialization.json`
- Prior WA ladder run directories under
  `docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-rescope-001/artifacts/mesh-ladder-runs/wa_cascades_forest_h1/`

Conditional:
- `crates/AGENTS.md` before any Rust edit under `crates/`.
- `tests/AGENTS.md` before any test edit under `tests/`.
- `docs/specifications/science-contract-authoring-procedure.md` before
  changing canonical `SC-*` text.
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
  before changing contract binding exposure, invariant text, or BEI rows.

## Scope

Included:
- Scaffold package-local documentation, prompts, and artifacts.
- Re-run or consume the WA active-plain ladder evidence for
  `baseline_fixed10`, `dx20`, `dx10`, `dx5`, `dx2p5`, and `dx1p25` as needed.
- Record exact failing closure operands for day 1122:
  `injected`, `clamp`, `terminal outlet`, `mesh storage`, absolute residual,
  relative residual, timing, and binary provenance.
- Use trace evidence from completed rungs to identify which day/lane drives
  the huge `dx10/dx5` clamp/storage/outlet totals.
- Compare active hydrology/source rows across completed rungs to distinguish
  source-producer defects from router-internal numerics.
- Classify the issue as one or more of:
  - bounded diagnostic target-`dx` regime limitation,
  - active router numerics defect,
  - route-coefficient/geometry stressor,
  - production-default blocker.
- Produce review, verification, disposition, and follow-on handoff artifacts.

Excluded:
- Production mesh-policy changes.
- Any hybrid implicit-stepper revival or reference to H2637 as fleet-general
  performance evidence.
- Retuning physics, coefficients, or tolerances to make the WA fine rungs pass.
- WEPPpy management-generation edits.
- Broad Tier-2 mesh adjudication. This package only investigates the WA
  blocker exposed by that adjudication.

## Subagent Authorization

This package explicitly authorizes spawning/delegating to review,
verification, and diagnostic/comparator subagents. Expected outputs are
package-local `review-*.md` and `verification-*.md` artifacts. Subagent write
access is bounded to package artifacts unless the package owner explicitly
assigns an implementation fix with a disjoint write set.

## Phase Plan

### WA-A: Scaffold and Provenance

Create package-local structure, prompt files, a required-reading map, and
catalog updates. Record whether evidence is current-rerun, prior committed
artifact reuse, or both. If prior artifacts are reused, record their paths and
hashes.

### WA-B: Day-1122 Closure Reproduction

Re-run the WA high-resolution rungs in the package or consume the committed
Tier-2 run logs with checksum provenance. Record:
- `dx2p5` day-1122 failure operands and timing.
- `dx1p25` day-1122 failure operands and timing.
- relative error against injected source and absolute error in litres.
- whether the failure is the day cascade guard only, or also seam, identity,
  source reconstruction, non-finite, CFL, or output publication failure.

### WA-C: Magnitude Attribution

Use completed rung traces to identify the top clamp/storage/outlet lane-days.
Separate the day-1122 failure from the day/lane that dominates `dx10/dx5`
totals. Check whether active hydrology inputs (`Q`, `QOFE`, `RM`, area,
`latqcc`) change across rungs.

### WA-D: Numerics Adjudication

Decide whether the observed behavior is:
- a production default problem at fixed `10 cells/OFE`,
- a target-`dx` diagnostic limitation only,
- a solver numerics defect that is outside this package envelope, or
- a coefficient/geometry stressor requiring source-authority follow-up.

Do not relax `SC-OFEROUTE-001` closure tolerances in this package. Any future
tolerance or compensated-ledger proposal must be contract-first.

### WA-E: Review, Verification, and Closure

Run package-local analysis checks, doc lint, `git diff --check`, and focused
static/runtime checks appropriate to the write set. Obtain independent review
and verification artifacts, disposition their findings, and close with either
`EXECUTED-COMPLETE-*` or `EXECUTED-HOLD-*`.

## Required Artifacts

- `artifacts/required-reading-map.md`
- `artifacts/day1122-reproduction.md`
- `artifacts/magnitude-attribution.md`
- `artifacts/numerics-adjudication.md`
- `artifacts/implementation.md` or `artifacts/hold-legitimacy-audit.md`
- `artifacts/gate-results.md`
- `artifacts/review-*.md`
- `artifacts/verification-*.md`
- `artifacts/disposition.md`
- `artifacts/final-disposition.md`
- `artifacts/worker-handoff.md`

## Gates

Required:
- `git diff --check`
- Markdown/doc lint for touched docs.
- Package-local WA analysis script rerun or checksum-backed static analysis.
- Current WA active-plain rerun or explicit reuse audit of the Tier-2 ladder
  artifacts.
- Active-mode closure evidence for `dx2p5` and `dx1p25`.
- Completed-rung trace evidence for `baseline_fixed10`, `dx20`, `dx10`, and
  `dx5`.
- Static check that production default remains fixed `10 cells/OFE`.

Conditional:
- `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo nextest run --workspace --profile full`, and `cargo deny check` if
  Rust or contract-derived implementation files are touched.
- Contract/profile/BEI checks if `SC-*` contracts are touched.
- Authority anti-evasion guards if required-case bindings, cohort fixtures, or
  external-authority suite posture are touched.

## Completion Criteria

Complete only if the package proves one of:
- The WA blocker is bounded to rejected diagnostic target-`dx` rungs and does
  not affect the retained production default.
- A narrow code or contract fix safely closes the blocker and passes the
  required gates.
- The blocker is legitimate and cannot be safely fixed in-envelope, with exact
  evidence and the first follow-on action recorded.
