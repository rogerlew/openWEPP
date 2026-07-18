# Correctness Authority Model

Status: Active
Last updated: 2026-06-18
Scope: openWEPP kernel/process correctness adjudication and gate authority

## Purpose

Define the normative ranking and adjudication rules for correctness acceptance
in openWEPP, including how canonical `SC-*` contracts, external-authority
suites, and legacy comparators are used.

## Canonical Authority Planes

1. **Contract authority plane (normative text):**
   - Canonical process authority is the `SC-*` contract set under
     `docs/specifications/science-contracts/contracts/`.
   - For empirical or conceptual models lacking stronger external authority,
     canonical contracts may include ADR-0024 source-intent anchors extracted
     from the pinned reference implementation.
2. **Executable authority plane (normative gates):**
   - Runtime acceptance is adjudicated by gate outcomes tied to `SC-*`
     invariants and external-authority suites.

Work-package artifacts remain evidence only and do not replace either plane.

## Authority Ranking (Normative)

| Rank | Authority class | Role | Acceptance effect |
|---|---|---|---|
| A0 | `SC-*` canonical contract authority, including ratified source-intent anchors | Defines invariant truth and required guard posture. | Required. Missing/ambiguous authority is `HOLD`. |
| A1 | Hard invariant gates (closure/bounds/domain) | Validates conservation, bounds, and typed fail-closed behavior. | Required. Failure is blocking. |
| A2 | External-authority legacy/sanity suites (Level-3) | Validates legacy-anchored branch/conformance laws as structured investigation evidence. | Non-blocking investigation signal; not acceptance authority. |
| A3 | External-authority constitutive suites (Level-4) | Validates constitutive physics laws not adjudicable by conservation alone. | Required for touched process families. Failure is blocking. |
| A4 | External validation suites (Level-5 measured data) | Validates system-level behavior versus empirical observations. | Non-blocking unless package/release explicitly upgrades scope. |
| A5 | Independent solver suites (Level-6) | Cross-checks selected canonical scenarios with independent solvers. | Non-blocking unless package/release explicitly upgrades scope. |
| A6 | Legacy comparator suites | Detects change and regression signatures versus legacy baselines. | Investigation signal only; not acceptance authority. |

## Adjudication Rules (Normative)

1. `A0` and `A1` are mandatory for all kernel-affecting packages.
2. If a package touches a process family with defined Level-4 suites, the
   relevant `A3` suites are mandatory and blocking.
3. Level-3 legacy/sanity suites (`A2`) must remain non-blocking investigation
   evidence and cannot be the sole acceptance oracle.
4. `A4` and `A5` default to periodic/manual validation unless a release gate
   explicitly promotes them to required.
5. `A6` (legacy comparator) cannot be used as a sole acceptance oracle.
6. Legacy-only deviations (`A2`/`A6`) route to investigation/disposition; they
   do not override passing `A0/A1/A3` outcomes.

## ADR-0017 Comparator-Flag Adjudication (Normative)

ADR-0017 ratifies the operational rule that a legacy comparator is a flag, not
a target. Comparator agreement can increase confidence, and comparator
divergence can identify where to investigate, but neither state is sufficient
acceptance authority.

Comparator/ledger packages must use the peer verdict set
`HARNESS-SURFACE-MISMATCH`, `LEGACY-DEFECTIVE`, `OPENWEPP-DEFECTIVE`, and
`UNRESOLVED`. A row may be labeled `OPENWEPP-DEFECTIVE` only after the package
proves both requirements:

1. the paired surfaces are like-for-like in units and lineage stage; and
2. independent correctness authority is met through contract/source,
   conservation, documented physics-law, external constitutive, or accepted
   corrected-fix evidence.

Criterion-C-style independent correctness authority may not be waived for an
`OPENWEPP-DEFECTIVE` verdict. A depth-vs-water-equivalent, raw-vs-released, or
lineage-stage mismatch must resolve to `HARNESS-SURFACE-MISMATCH` or
`UNRESOLVED`, not to an openWEPP defect. Suspicious unit-ratio signatures such
as approximately `10x` or `1000x` must be treated as surface-mismatch hypotheses
until disproven.

`HOLD` remains valid when it is owned and scoped: the artifact must state the
owner or owning follow-on package, the next evidence gate, and the reason
closure is not yet possible. Ownerless or unscoped `HOLD` findings are
governance failures. Findings that invalidate prior verdicts must retract or
supersede those verdicts in the package that published them.

## ADR-0024 Source-Intent Anchors (Normative)

Reference-implementation source intent is allowed as an `A0` provenance basis
when all conditions hold:

1. the governed model is empirical or conceptual and lacks stronger external
   physical, measured-data, or independent-solver authority for the claimed
   behavior;
2. the source anchor cites the pinned reference implementation by file, routine,
   and commit provenance;
3. the canonical `SC-*` contract extracts the intended algorithm, branch
   conditions, units, and guard posture into an invariant or obligation; and
4. known legacy output artifacts, non-conservation, disabled branches, or
   implementation bugs are flagged as non-authoritative.

This is not a legacy-comparator promotion. Legacy binary behavior remains `A6`
and cannot be used as a sole acceptance oracle. The source-intent anchor becomes
binding only through the canonical contract text. If source intent is ambiguous
or contradicted by known bugs that cannot be separated from the algorithm, the
correct disposition is `HOLD`, not a manufactured acceptance target.

## Release/CI Lane Enforcement (Normative)

Gate timing and lifecycle placement are governed by
`docs/standards/testing-and-gate-strategy.md`. Authority class and scientific
outcome remain governed here: affected A0 admission, A1 hard-invariant, and A3
constitutive suites are non-deferrable; A2/A4/A5/A6 execution integrity and
investigation outcomes remain separate axes unless prospectively promoted.

The current conservative release implementation is wired through:

- `.github/workflows/release-gates.yml`
- `tools/release/run_release_candidate_gates.sh`

Until `TESTGATE-CI-01` completes shadow acceptance and cutover, its trigger
layout remains executable transition behavior rather than independent timing
authority. The target lane policy is:

1. `required` lane:
   - affected A0/A1/A3 suites run at increment closure,
   - complete required suites run at campaign closure and release,
   - blocks on any `hard-fail` suite failure.
2. `periodic` lane:
   - runs at its declared campaign, backstop, or scheduled trigger,
   - may be invoked on demand with `--run-authority-periodic`,
   - blocks only for `hard-fail` suite failures.
3. `manual` lane:
   - runs only when explicitly requested or selected for release by policy,
   - blocks only for `hard-fail` suite failures.

Failure-class policy:

1. `hard-fail`:
   - gate exits non-zero,
   - disposition remains `HOLD` until resolved; there is no generic risk-
     acceptance, waiver, downgrade, bless, or accept-current path for affected
     A0/A1/A3 failures.
2. `investigation`:
   - gate records failure in authority-lane report output,
   - does not fail the workflow by default,
   - requires explicit investigation/disposition artifact follow-through.

## External-Authority Suite Minimum Schema (Normative)

Every external-authority suite must define, at minimum:

1. `suite_id` (stable identifier).
2. `authority_level` (`3`, `4`, `5`, or `6`).
3. `domain` and `process_family`.
4. `sc_invariant_refs` (one or more `SC-*#INV-*` references).
5. `external_citations` (source + version/edition + commit/date where
   applicable).
6. `fixtures` (path(s), units basis, and fixture class).
7. Fixture integrity metadata per fixture:
   - `hash` (`sha256`),
   - `source_repo`,
   - `source_commit`,
   - `source_path`,
   - `source_sha256`,
   - `transform_note`.
8. `tolerances` (absolute/relative/mixed with explicit units).
9. `gate_lane` (`required`, `periodic`, `manual`).
10. `failure_class` (`hard-fail` or `investigation`).
11. `provenance` (author/date/update lineage).

## Legacy Comparator Demotion and Retirement Policy (Normative)

1. Legacy comparator results are retained as change-detection evidence only.
2. No acceptance gate may be defined as "match legacy column X" alone.
3. Legacy comparator retirement is coverage-based, not calendar-based.

Minimum retirement readiness criteria:

1. Hard-gate (`A1`) coverage runs on CI for kernel PRs.
2. Level-4 constitutive suites (`A3`) exist for active release-critical
   process families.
3. Required authority-stack lanes pass for a sustained release window.

## Required Locations

- Canonical authority model: this file.
- External suite framework entrypoint:
  `docs/specifications/external-authority/README.md`.
- Canonical contract registry:
  `docs/specifications/science-contracts/index.md`.
