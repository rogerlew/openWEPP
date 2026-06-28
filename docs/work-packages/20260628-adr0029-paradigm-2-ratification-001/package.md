# ADR-0029 Paradigm 2 Ratification Package

Status: `executed-complete`
Date: `2026-06-28`
Owner: Codex
Type: governance/documentation ratification package

## Objective

Verify the load-bearing claims in ADR-0028 and ADR-0029, then ratify ADR-0028
first and ADR-0029 second if the evidence is consistent. This package is
governance/documentation only: no code, physics, science-contract, fixture,
schema, default, or runtime behavior change is authorized.

## Read-First Basis

- `docs/decisions/0028-observed-data-admission-authority.md`
- `docs/decisions/0029-commit-paradigm-2-multilayer-snow.md`
- ADR-0011, ADR-0017, ADR-0025, and ADR-0026
- `docs/planning/paradigm2-multilayer-snow-specification.md`
- `docs/work-packages/20260628-snow-density-paradigm-assessment-001/`
- `docs/work-packages/20260605-adr0017-comparator-distrust-ratification-001/`

## Execution Summary

ADR-0028 is verified as already operationalized in the snow/frost 10.3.x arc:
`INV-SNOWFREEZE-050` is the forcing-robust observed-data rubric, and the
candidate gates for Harder-Pomeroy, sublimation, and climate-class density
consume that rubric while preserving ADR-0017 comparator-as-flag discipline.

ADR-0029 is verified as consistent with the accepted ADR set and the local
evidence:

- the current no-env snow default is the `15` / `179` bulk floor and beats the
  legacy flag profile at `16` / `176`;
- SNOWDENSITY-10.3.22 closed as `HOLD-GATE-FAILURE-NON-PROMOTION`;
- the variable-length layer pattern already exists in `DirectFrostLaneState`
  under the ADR-0026 winter-column exception;
- PARADIGM-2 Stage 0 is complete as a pure, unwired `openwepp-meteorology`
  surface-energy-balance crate addition.

## Verification Result

The checklist in `artifacts/verification-checklist.md` is complete. No blocking
inconsistency was found, so the package ratifies:

- ADR-0028: `Proposed` -> `Accepted`
- ADR-0029: `Proposed` -> `Accepted`

The WP-local climate-class-first ADR candidate in the paradigm-assessment
package is marked superseded by ADR-0029.

## Intended Write Set

- `docs/decisions/0028-observed-data-admission-authority.md`
- `docs/decisions/0029-commit-paradigm-2-multilayer-snow.md`
- `docs/decisions/README.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260628-snow-density-paradigm-assessment-001/artifacts/adr-candidate-snow-density-paradigm.md`
- `docs/work-packages/20260628-adr0029-paradigm-2-ratification-001/**`

## Protected Boundaries

No production code, runtime selector, default activation, science contract,
fixture, public output schema, density cap, frost behavior, or physics equation
is changed by this package.

## Gates

| Gate | Status | Evidence |
|---|---|---|
| ADR-0028 checklist complete | Pass | `artifacts/verification-checklist.md` |
| ADR-0029 checklist complete | Pass | `artifacts/verification-checklist.md` |
| ADR status and index updated consistently | Pass | ADR files and `docs/decisions/README.md` |
| Superseded WP-local ADR candidate marked | Pass | Paradigm-assessment ADR-candidate artifact |
| Markdown/governance gates | Pass | `artifacts/gate-results.md` |

## Disposition

`RATIFIED-COMPLETE`.

ADR-0028 and ADR-0029 are Accepted as of 2026-06-28 by Roger Lew and Codex via
this package. The prior Paradigm-1-first WP-local ADR candidate is superseded by
ADR-0029.
