# Canonical Hourly Lane D Routing-Coefficient Projection Authority

Status: `QUEUED`
Package ID:
`20260708-laned-router-canonical-hourly-laned-routing-coeff-projection-authority-001`
Owner: Codex
Scaffold date: `2026-07-08`
Evidence mode: `Static scaffold; no contract or implementation executed`
Execution order: after
`20260708-groundwater-baseflow-srivastava-authority-001` closes or records a
handoff that does not invalidate this package's authority basis, and before the
groundwater/baseflow single-OFE + Lane D MOFE implementation package.

## Objective

Decide, contract-first, whether openWEPP production should make hourly water
balance plus Lane D active routing canonical for both single-OFE and MOFE
hillslopes by expanding routing-coefficient authority beyond explicit native
`routing_coefficients`.

The package must replace the current all-explicit-coefficients production
eligibility rule with a broader but still fail-closed authority rule only if it
can justify projected coefficients from legacy cropland/runtime surfaces:

- explicit native `routing_coefficients` remain authoritative and win;
- disturbed/native forest route coefficients retain their disturbed-table
  authority;
- legacy cropland may gain a contract-ratified projection to Lane D
  coefficients;
- old non-hourly water balance, DC01-only surface routing, and non-Lane-D MOFE
  paths remain in the codebase for legacy validation, comparator evidence,
  protected rollback, and regression diagnosis, not new production consumers.

## Rationale

Groundwater/baseflow, HBP hourly water/sediment consumption, and future
watershed integration become much simpler if production has one water/sediment
path: hourly source shape, Lane D active surface routing, routed hydrograph
erosion shape, and typed downstream consumers. Keeping parallel production
implementations for single-OFE daily/DC01 and MOFE Lane D would multiply
consumer and closure surfaces.

Projected coefficients do not need to be exact measured Papanicolaou operands.
They must be source-authorized, physically plausible, bounded, deterministic,
and fidelity-adequate against a predeclared legacy-ballpark envelope.

## Required Reading

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/standards/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/ROADMAP.md` `## Watershed Runtime Performance Queue`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/specifications/wepp-input-files/specs/plant-file.spec.md`
- `docs/work-packages/20260708-laned-router-conditional-default-activation-001/package.md`
- `docs/work-packages/20260708-plant-file-native-lanuse-routing-doc-001/package.md`
- this package's `artifacts/required-reading-map.md`

Conditional:

- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-GWBASEFLOW-001.md`
  after the groundwater/baseflow authority package closes.
- `docs/specifications/science-contracts/index.md`
- `docs/work-packages/20260707-laned-router-d16-hybrid-disturbed-route-coeff-source-acquisition-001/package.md`
- `docs/work-packages/20260707-laned-router-d16-hybrid-route-coeff-authoring-bridge-001/package.md`

Baseline code / on demand:

- `/workdir/wepp-forest_260430_baseline/src/frcfac.for`
- `/workdir/wepp-forest_260430_baseline/src/param.for`
- `/workdir/wepp-forest_260430_baseline/src/bigout.for`
- `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for`
- other baseline management/cropland/rill-friction sources only when the
  source audit requires them.

## Scope

### Included

- Package-local scaffold, artifacts, prompt, and catalog/roadmap pointers.
- Contract-first audit of current Lane D coefficient authority and default
  eligibility.
- Source audit for legacy cropland surfaces that could project Lane D static
  coefficients:
  - `k_o`;
  - `form_C_d`;
  - `D_r_m`;
  - `lambda`;
  - `vegetation_C_d`.
- Canonical production-path policy:
  - hourly water balance plus Lane D active routing is the target production
    path for single-OFE and MOFE;
  - non-hourly and non-Lane-D paths remain validation/reference/rollback
    surfaces only.
- A predeclared fidelity-envelope design for coefficient projection acceptance:
  water-balance closure, runoff magnitude, sediment magnitude, routed timing,
  and protected rollback/off identity.
- Contract design or amendment plan for `SC-OFEROUTE-001`,
  `plant-file.spec.md`, and any affected `SC-*` boundaries.
- Worker handoff for implementation/default-eligibility changes if authority
  closes.

### Excluded

- No production Rust implementation in this authority package unless explicitly
  amended before execution.
- No coefficient table tuning to a single fixture.
- No silent fallback when projection authority is missing.
- No deletion of daily/DC01/non-Lane-D legacy paths.
- No groundwater/baseflow implementation.
- No HBP hourly water/sediment watershed consumer implementation.
- No wepppy edits.

## Intended Write Set

Package and catalog:

- `docs/work-packages/20260708-laned-router-canonical-hourly-laned-routing-coeff-projection-authority-001/**`
- `docs/ROADMAP.md`
- `docs/work-packages/README.md`

Contract/spec authority, only if executing beyond scaffold:

- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/specifications/wepp-input-files/specs/plant-file.spec.md`
- `docs/specifications/science-contracts/index.md` only if lifecycle metadata
  changes.
- `SC-SED-001`, `SC-RUNOFFPART-001`, `SC-WATBAL-001`, and
  `SC-GWBASEFLOW-001` only for explicit boundary notes if needed.

Protected:

- Do not edit `docs/specifications/science-contracts/contracts/SC-GWBASEFLOW-001.md`
  or `docs/specifications/science-contracts/index.md` while M-T2A is actively
  running unless the operator directs coordination.
- Do not edit Rust production code in the scaffold.
- Do not remove tests or legacy/reference paths.

## Phase Plan

### Phase A - Intake and Ordering Check

1. Record `git status --short --branch`.
2. Confirm M-T2A final disposition or handoff is available; if it is still
   running, keep this package queued and do not execute authority amendments.
3. Produce `artifacts/current-authority-audit.md` for the current
   all-explicit-coefficients rule, default/no-coeff fallback, and mixed
   authority fail-closed behavior.

### Phase B - Legacy Cropland Source Audit

1. Map legacy cropland surfaces that can support static Lane D operands.
2. Classify each candidate as:
   - direct coefficient authority;
   - bounded projection authority;
   - diagnostic/context only;
   - rejected alias.
3. Record exact baseline file:line evidence and current openWEPP projection
   surfaces.
4. Complete `artifacts/legacy-cropland-source-audit.md`.

### Phase C - Projection Rule and Canonical Path Design

1. Draft a deterministic projection rule or hold if no defensible projection
   exists.
2. Define provenance labels for explicit, disturbed/native, projected-cropland,
   and unsupported coefficient authority.
3. Decide whether mixed explicit/projected authority is allowed when every
   scheduled OFE resolves to a complete coefficient set.
4. Define the canonical production-path policy and legacy/reference retention
   policy.
5. Complete `artifacts/projection-rule-design.md` and
   `artifacts/canonical-path-policy.md`.

### Phase D - Fidelity Envelope and Contract Disposition

1. Predeclare acceptance surfaces for the implementation package:
   - single-OFE runoff/sediment;
   - MOFE runoff/sediment;
   - active closure/DC01 no-double-feed;
   - routed-hydrograph erosion consumer;
   - protected explicit-disable/legacy validation path identity.
2. Amend contracts/specs if authority closes in this package.
3. If authority cannot close, record a hold with the missing source or rejected
   projection evidence.
4. Complete review, verification, final disposition, and worker handoff.

## Subagent Authorization

This package explicitly authorizes subagent spawning/delegation for read-only
science/contract review, verification, and comparator-design review. Authorized
roles:

- review: inspect projection-rule authority, canonical-path policy, and
  no-surrogate/no-silent-fallback posture.
- verification: independently check source-line mappings, contract deltas, and
  gate claims.
- comparator design: review the proposed fidelity envelope and fixture/cohort
  surfaces for tolerance-fitting risk.

Expected outputs are package-local `artifacts/review-*.md`,
`artifacts/verification-*.md`, and optionally
`artifacts/comparator-design-*.md`. Write access is bounded to this package's
artifact directory unless the operator explicitly expands scope.

## Required Artifacts

- `artifacts/README.md`
- `artifacts/required-reading-map.md`
- `artifacts/current-authority-audit.md`
- `artifacts/legacy-cropland-source-audit.md`
- `artifacts/projection-rule-design.md`
- `artifacts/canonical-path-policy.md`
- `artifacts/fidelity-envelope.md`
- `artifacts/contract-disposition.md`
- `artifacts/gate-results.md`
- `artifacts/review-*.md`
- `artifacts/verification-*.md`
- `artifacts/disposition.md`
- `artifacts/final-disposition.md`
- `artifacts/worker-handoff.md`

## Gates

Required for scaffold:

- `git diff --check`
- Markdown/doc lint for touched docs.

Required for execution:

- Contract/profile/BEI checks required by touched `SC-*` contracts.
- Source-line evidence for every legacy projection claim.
- No-tolerance-fitting audit for the fidelity envelope.
- Dual review and verification disposition.

Conditionally required if implementation scope is added:

- Focused Lane D / `ofe_routing` / routing-coefficient tests.
- Single-OFE and MOFE active Lane D evidence.
- Protected explicit-disable/legacy validation identity.
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace --profile full`
- `cargo deny check`

## Exit Criteria

`SCAFFOLDED`:

- Package directory, prompt, artifacts, ROADMAP pointer, and catalog entry
  exist.
- The package is explicitly queued after M-T2A and before M-T2B.
- No contract or implementation claim is made.

`EXECUTED-COMPLETE-AUTHORITY`:

- Projection authority is accepted contract-first.
- Canonical hourly Lane D production policy is recorded.
- Fidelity envelope and implementation handoff are complete.
- Legacy/reference path retention is explicitly non-production.

`EXECUTED-HOLD-*`:

- Projection authority cannot be safely ratified.
- Hold audit names exact blocker, evidence, rejected routes, and first
  actionable follow-on.

## Final Outcome

Queued scaffold. Execution has not started.
