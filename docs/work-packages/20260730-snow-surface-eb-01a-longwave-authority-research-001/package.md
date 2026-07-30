# SNOW-SURFACE-EB-01A Longwave Authority Research

Status: `complete`

Package ID:
`20260730-snow-surface-eb-01a-longwave-authority-research-001`

Campaign: `SNOW-SURFACE-EB`

Owner: Codex

Execution mode: `package-end-to-end`

Science intent: `equation-level authority resolution`; no production physics,
canonical contract, calibration, or selector change is authorized.

## Purpose

Resolve the authority gaps that held `SNOW-SURFACE-EB-02`: atmospheric
downwelling longwave, the sky/canopy radiative view partition, canopy
emissivity, and canopy radiometric temperature. Translate the literature into
an implementation-facing formulation and identify, without inventing a
surrogate, which openWEPP operands are ready and which remain blocked.

## Progress

- [x] (2026-07-30) Reconciled the EB-01 authority gaps and current runtime
  operands.
- [x] (2026-07-30) Acquired and reviewed equation-level primary sources.
- [x] (2026-07-30) Classified the two-component, three-component, canopy
  temperature, atmospheric forcing, and outgoing-snow formulations.
- [x] (2026-07-30) Produced source, equation, operand, uncertainty, and
  acquisition ledgers.
- [x] (2026-07-30) Produced two accessible figures with standalone Markdown
  sidecars.
- [x] (2026-07-30) Completed validation, dual review, finding disposition,
  dual terminal verification, and final disposition.
- [x] (2026-07-30) Applied the user-directed derived-sky-view amendment: no
  new user coefficient or remote-sensing input may be required; EB-02 must
  derive effective sky view from existing canopy state.
- [x] (2026-07-30) Reopened the amended tree for dual review, corrected the
  equation ledger, operand lineage, roadmap hold, and stop-loss, then repeated
  terminal validation and verification before restoring complete status.

## Governing Authority

- `docs/planning/snow-surface-energy-balance-roadmap.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- ADR-0011, ADR-0017, ADR-0042, and ADR-0043
- `SNOW-SURFACE-EB-01`
- Essery et al. (2008), Sicart et al. (2004), Flerchinger et al. (2009),
  Lundquist et al. (2013), Webster et al. (2016), Musselman and Pomeroy
  (2017), Rutter et al. (2023), and Essery et al. (2025)

This package is evidence and successor-planning authority. Canonical process
authority still requires a reviewed `SC-SNOWFREEZE-001` amendment before
production implementation.

## Included Scope

- Recover exact equations, units, signs, coefficients, assumptions, test
  regimes, uncertainty, and exclusions from primary sources.
- Decide whether a two-part sky/canopy model or a three-part
  sky/canopy/trunk model is the proper openWEPP hillslope-scale formulation.
- Decide when air temperature is an admissible effective canopy radiometric
  temperature and when it is not.
- Decide whether canopy cover or LAI can substitute for hemispherical
  radiometric sky-view fraction.
- Bind the product requirement that effective sky view be derived from
  existing canopy cover, structural cover, LAI, and scientifically relevant
  height information rather than requested as another user coefficient.
- Identify an atmospheric longwave estimator compatible with current hourly
  forcing and record its demonstrated error and polar-night limitation.
- Preserve explicit snow outgoing longwave and the positive-toward-snow sign
  convention needed by the shared surface-energy balance.
- State the exact contract and runtime prerequisites for EB-02.

## Excluded Scope

- No Rust, test, fixture, reference-library, or usersum edit.
- No canonical science-contract amendment.
- No empirical calibration, source coefficient refit, or site-specific
  parameter.
- No production use of canopy cover as sky-view fraction.
- No required user-supplied sky-view, hemispherical-photo, LiDAR, or other
  remote-sensing input.
- No gap-edge, individual-tree, or ray-tracing implementation.
- No default activation or EB-04 result-bearing execution.

## Deliverables

- `artifacts/pre-execution-intent.md`
- `artifacts/required-reading-map.md`
- `artifacts/source-authority-ledger.csv`
- `artifacts/source-reconciliation.md`
- `artifacts/equation-ledger.csv`
- `artifacts/atmospheric-longwave-formulation.md`
- `artifacts/canopy-to-sky-view-decision.md`
- `artifacts/operand-readiness-ledger.csv`
- `artifacts/formulation-decision.md`
- `artifacts/uncertainty-and-scope.md`
- `artifacts/source-acquisition-needed.csv`
- `artifacts/science-summary.md`
- `artifacts/figures/*.svg`
- `artifacts/figures/*.md`
- `artifacts/gate-evidence.md`
- `artifacts/exact-diff-reconciliation.md`
- `artifacts/line-count-governance.md`
- `artifacts/review-agent-a.md`
- `artifacts/review-agent-b.md`
- `artifacts/finding-disposition.md`
- `artifacts/verification-agent-a.md`
- `artifacts/verification-agent-b.md`
- `artifacts/final-disposition.md`

## Intended Write Set

- `docs/ROADMAP.md`
- `docs/planning/snow-surface-energy-balance-roadmap.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260730-snow-surface-eb-01a-longwave-authority-research-001/**`

## Validation And Acceptance

- Run the deterministic package generator/checker.
- Parse every CSV and generated SVG.
- Require SVG title, description, image role, and same-stem Markdown sidecar.
- Validate local Markdown links.
- Run canonical scoped Markdown lint and validation.
- Run `git diff --check`.
- Reconcile the exact terminal diff to the intended write set.

Rust, Nextest, Clippy, comparator, and empirical gates are `NOT APPLICABLE`
unless the terminal diff discovers executable, contract, or fixture impact.

## Exit Criteria

1. Every EB-01 longwave authority gap has a source-backed disposition.
2. The selected equations state their units, signs, assumptions, and regime.
3. The role of trunk heating is resolved at the openWEPP hillslope scale.
4. Sky-view fraction is not silently aliased to canopy cover or LAI.
5. Canopy-temperature use is bounded by the evidence.
6. Atmospheric-longwave uncertainty and polar-night limitations are visible.
7. EB-02 receives precise contract-research and runtime-implementation
   dispositions; the two need not share a status.
8. Validation, dual review, finding disposition, and dual exact-tree
   verification pass.

## Stop-Loss

Stop and retain EB-02 on hold if a load-bearing equation or coefficient cannot
be verified from source bytes, if the only available canopy-temperature rule
is a site-fitted surrogate, or if current canopy cover must be mislabeled as a
radiometric view factor to proceed.

Also stop and retain EB-02 on hold, or adjudicate the limitation explicitly,
if an authority-backed deterministic effective-vegetation-area mapping cannot
be formed from existing canopy state. Do not escape that stop-loss with an
invented or site-fitted blend, a new user-entered sky-view coefficient, or a
required remote-sensing dataset.

## Review And Verification

Require two independent reviews:

- Review A: source authority, radiative formulation, canopy-temperature
  interpretation, regimes, and claim limits.
- Review B: runtime operands, units, signs, reproducibility, package
  governance, and successor admission.

Every finding must be dispositioned. After corrections, require two independent
terminal verifications of the exact final tree.

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation for two read-only review subagents and two read-only
terminal-verification subagents. Their write access is limited to the four
named package-local review/verification artifacts. They must not edit
production code, canonical contracts, fixtures, or roadmap decisions.

## Decision Log

- Decision: treat hemispherical sky-view fraction as a radiative geometry
  operand, not as ordinary plan-view canopy cover.
  Rationale: the source equation weights solid angle by `sin(theta) cos(theta)`;
  canopy cover and LAI do not preserve that geometry.
  Date/author: 2026-07-30 / Codex.
- Decision: target a two-component stand-scale formulation away from gaps and
  edges; retain explicit trunk models as excluded high-resolution physics.
  Rationale: Rutter et al. (2023) identify canopy density as first order, while
  Musselman and Pomeroy (2017) find trunk influence highly localized.
  Date/author: 2026-07-30 / Codex.
- Decision: derive effective sky view internally from coefficients already
  available to openWEPP; do not add a user-entered sky-view coefficient.
  Rationale: FSM supplies an authoritative diffuse-transmission base, while a
  new user or remote-sensing requirement would be error-prone and would
  improperly make optional observations an implementation blocker.
  Date/author: 2026-07-30 / user and Codex.
