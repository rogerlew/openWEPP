# Correctness Authority Model

Status: Active
Last updated: 2026-05-31
Scope: openWEPP kernel/process correctness adjudication and gate authority

## Purpose

Define the normative ranking and adjudication rules for correctness acceptance
in openWEPP, including how canonical `SC-*` contracts, external constitutive
suites, and legacy comparators are used.

## Canonical Authority Planes

1. **Contract authority plane (normative text):**
   - Canonical process authority is the `SC-*` contract set under
     `docs/specifications/science-contracts/contracts/`.
2. **Executable authority plane (normative gates):**
   - Runtime acceptance is adjudicated by gate outcomes tied to `SC-*`
     invariants and external constitutive suites.

Work-package artifacts remain evidence only and do not replace either plane.

## Authority Ranking (Normative)

| Rank | Authority class | Role | Acceptance effect |
|---|---|---|---|
| A0 | `SC-*` canonical contract authority | Defines invariant truth and required guard posture. | Required. Missing/ambiguous authority is `HOLD`. |
| A1 | Hard invariant gates (closure/bounds/domain) | Validates conservation, bounds, and typed fail-closed behavior. | Required. Failure is blocking. |
| A2 | External-authority constitutive suites (Level-4) | Validates constitutive physics laws not adjudicable by conservation alone. | Required for touched process families. Failure is blocking. |
| A3 | External validation suites (Level-5 measured data) | Validates system-level behavior versus empirical observations. | Non-blocking unless package/release explicitly upgrades scope. |
| A4 | Independent solver suites (Level-6) | Cross-checks selected canonical scenarios with independent solvers. | Non-blocking unless package/release explicitly upgrades scope. |
| A5 | Legacy comparator suites | Detects change and regression signatures versus legacy baselines. | Investigation signal only; not acceptance authority. |

## Adjudication Rules (Normative)

1. `A0` and `A1` are mandatory for all kernel-affecting packages.
2. If a package touches a process family with defined Level-4 suites, the
   relevant `A2` suites are mandatory and blocking.
3. `A3` and `A4` default to periodic/manual validation unless a release gate
   explicitly promotes them to required.
4. `A5` (legacy comparator) cannot be used as a sole acceptance oracle.
5. Legacy-only deviations route to investigation/disposition; they do not
   override passing `A0-A2` outcomes.

## External-Authority Constitutive Suite Minimum Schema (Normative)

Every external-authority constitutive suite must define, at minimum:

1. `suite_id` (stable identifier).
2. `authority_level` (`4`, `5`, or `6`).
3. `domain` and `process_family`.
4. `sc_invariant_refs` (one or more `SC-*#INV-*` references).
5. `external_citations` (source + version/edition + commit/date where
   applicable).
6. `fixtures` (path(s), units basis, and fixture class).
7. `tolerances` (absolute/relative/mixed with explicit units).
8. `gate_lane` (`required`, `periodic`, `manual`).
9. `failure_class` (`hard-fail` or `investigation`).
10. `provenance` (author/date/update lineage).

## Legacy Comparator Demotion and Retirement Policy (Normative)

1. Legacy comparator results are retained as change-detection evidence only.
2. No acceptance gate may be defined as "match legacy column X" alone.
3. Legacy comparator retirement is coverage-based, not calendar-based.

Minimum retirement readiness criteria:

1. Hard-gate (`A1`) coverage runs on CI for kernel PRs.
2. Level-4 constitutive suites (`A2`) exist for active release-critical
   process families.
3. Required authority-stack lanes pass for a sustained release window.

## Required Locations

- Canonical authority model: this file.
- External suite framework entrypoint:
  `docs/specifications/external-authority/README.md`.
- Canonical contract registry:
  `docs/specifications/science-contracts/index.md`.

