# Kernel Process Contract Profile

Status: Active
Last updated: 2026-07-27
Scope: openWEPP kernel-authority contracts and kernel-adjacent runtime projection
contracts (`SC-*` files that define executable process behavior)

## Purpose

Define a single, mandatory contract format for kernel process authority so
kernel packages do not drift into unique per-package structures.

This profile complements, and does not replace:

- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/README.md`
- `docs/specifications/unit-governance.md`
- `docs/specifications/correctness-authority-model.md`
- `docs/specifications/external-authority/README.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`

## Applicability (Normative)

This profile is required when a work package:

1. implements or changes production kernel process behavior, or
2. changes runtime projection semantics that directly control kernel branch
   execution.

Examples include PL/WB queue packages such as `PL11+` and `WB10+`.

## Canonical Authority Location (Normative)

Kernel process authority must live in canonical science-contract files:

- `docs/specifications/science-contracts/contracts/SC-<DOMAIN>-<NNN>.md`

Work-package artifacts may summarize or disposition changes, but are not the
authority location.

## Correctness Authority Alignment (Normative)

Kernel process contracts and their tests must align to the repository-level
correctness authority model:

- `docs/specifications/correctness-authority-model.md`

Minimum alignment obligations:

1. Contract invariants (`SC-*`) remain canonical truth definitions.
2. Hard invariant gates (closure, bounds, domain guards) are mandatory and
   blocking for touched behavior.
3. External constitutive suites (Level-4) are mandatory for touched process
   families where suites are defined.
4. Legacy comparator outcomes are investigation signals only and cannot be sole
   acceptance authority.
5. Suite metadata and lane/failure semantics must conform to:
   - `docs/specifications/external-authority/suite-schema.md`

## Required Section Schema (Normative)

Applicable `SC-*` files must include all sections below in this order (exact
titles may vary slightly, but content obligations are mandatory):

1. Purpose and scientific scope.
2. Authority anchors (top-down citations).
3. Variables and units (canonical symbols first).
4. Algorithm state surfaces:
   - required inputs,
   - required outputs,
   - mutated state surfaces.
5. Algorithm specification:
   - numbered step sequence,
   - branch conditions,
   - equation references and/or pseudocode sufficient to reproduce logic.
6. Branch/guard table:
   - each branch trigger,
   - guard class,
   - typed failure behavior.
7. Invariants and invariant guard map.
8. Symbol alias map (canonical -> boundary/API names).
9. Constants/parameters table with provenance anchors.
10. Unit-governance map:
    - boundary-symbol registry entries or explicit registry gaps,
    - typed boundary requirements or scalar exceptions,
    - conversion helpers and raw-literal exceptions,
    - output metadata linkage for published symbols.
11. Tolerance and numeric notes.
12. Calibration and identifiability posture:
    - applicability or explicit `CALIBRATION_NOT_APPLICABLE` rationale,
    - calibratable/fixed/external parameters and units,
    - validity domains versus evidence bounds versus execution assumptions,
    - observation operator, identifiable combinations, evidence gaps, and
      prohibited claims,
    - orthogonal science-implementation, calibration-evidence, and
      identifiability status fields,
    - readiness matrix dispositions, evidence paths, and skip rationales,
    - calibration-readiness and synthetic-recovery obligations where
      structurally meaningful.
13. Test-vector obligations:
    - minimum scenario families,
    - expected observable outputs/invariants.
14. Gap register and promotability labels.

## Algorithm Detail Requirements (Normative)

Algorithm sections must be implementation-authoritative, not descriptive only.
At minimum, include:

1. Step-local preconditions and postconditions.
2. All branch selectors and priority/ordering when multiple conditions apply.
3. Units and domain bounds for each computed intermediate.
4. Closure or conservation relation when mass/energy/accounting transfers are
   involved.
5. Explicit handling for degenerate states (zero-demand, empty pools, dormant
   periods, etc.).
6. Unit conversions named by directional helper or recorded as explicit
   follow-up gaps under `docs/specifications/unit-governance.md`.

## Typed Failure Requirements (Normative)

Contract and implementation must agree on typed-failure posture:

1. No silent defaulting/clamping for invalid domain states.
2. Each hard-fail invariant family maps to an explicit typed error class.
3. Governance-only gaps must be labeled and promotion-gated.

## Compliance Checklist Requirement (Normative)

Each kernel-affecting work package must include an artifact proving profile
conformance. Minimum required checklist items:

1. Canonical `SC-*` file updated.
2. All required schema sections present.
3. Algorithm steps and branch table updated for changed behavior.
4. Guard/error mapping updated and aligned with code errors.
5. Unit-governance map completed for touched dimensional surfaces.
6. Calibration/identifiability posture and readiness matrix are complete, or
   calibration is explicitly not applicable.
7. Test-vector obligations reflected in tests and evidence.

## Non-Compliance Rule

If a kernel-affecting package does not satisfy this profile, package
disposition remains `HOLD` until compliance is established or a documented
risk-acceptance exception is approved.
