# WEPP Input File Parser Contract Authoring Procedure

Status: Active
Last updated: 2026-05-20
Scope: openWEPP parser contracts (`SC-INFILE-<SURFACE>-<NNN>`)

## Purpose

Define a reusable, mandatory workflow for writing and accepting openWEPP parser
contracts with:

1. specification-first parser authority,
2. required typed data-model definitions,
3. required parse-to-simulation propagation mapping,
4. required dual-agent review, disposition, and verification.

Principle: correctness over completion. Parser-contract work must remain in
`HOLD` until correctness criteria are satisfied.

`<SURFACE>` includes hillslope inputs, watershed inputs, and sidecar inputs.

## Canonical Locations (Normative)

Canonical parser contract authority files must be stored in:

- `docs/specifications/science-contracts/contracts/SC-INFILE-<SURFACE>-<NNN>.md`

Canonical parser-specification corpus is stored in:

- `docs/specifications/wepp-input-files/specs/`

Canonical parser-specification authoring procedure is stored in:

- `docs/specifications/wepp-input-specification-authoring-procedure.md`

Normative parser-contract structure requirements are stored in:

- `docs/specifications/wepp-input-files/parser-contract-requirements.md`

Canonical input-surface registry is stored in:

- `docs/specifications/wepp-input-files/input-surface-registry.md`

Work-package artifacts remain evidence/workflow records and are not the
canonical authority location.

## Authority and Evidence Rules

1. Parser-contract authority order is fixed:
   1. openWEPP-owned parser specs under `docs/specifications/wepp-input-files/specs/`
   2. WEPP technical references and literature invariants
   3. physical/common-sense invariants
   4. static legacy code inspection (`wepp-forest`, `wepppy`, `wepppyo3`) for provenance
2. Every non-trivial rule must include explicit evidence anchors.
3. Evidence tags are required per claim:
   - `[DIRECT]` for directly observed source/output facts
   - `[INFERENCE]` for reasoned interpretation
4. Evidence mode at document/review level must be explicit:
   - `Static` for read/reasoned work
   - `Ran` for executed-command/runtime evidence
5. Variable-symbol continuity is mandatory:
   - canonical symbols default to legacy WEPP / `wepp-forest` names,
   - openWEPP boundary names must be linked via explicit alias mapping.
6. Every invariant and parser rule must map to an explicit guard path
   (runtime typed error, explicit branch rejection, or governance `HOLD` gate).

## Required File Layout Per Contract Cycle

All review/disposition/verification artifacts must live under one work package.

Suggested layout:

`docs/work-packages/<wp>/artifacts/parser-contracts/<contract_id>/`

Required files:

1. `contract_ref.md` (canonical path + commit SHA under review)
2. `review_agent_a.md`
3. `review_agent_b.md`
4. `disposition.md`
5. `verification_agent_a.md`
6. `verification_agent_b.md`

## Draft Completeness Gate (Required)

A parser contract is review-ready only when all required sections from
`parser-contract-requirements.md` exist and are populated.

Minimum required content:

1. Stable ID and lifecycle metadata:
   - `contract_id`, `title`, `status`, `maturity`, `owner`, `contract_version`
2. Source grammar/record structure with datver/version applicability.
3. Typed field specification table (symbols, alias, units, types, cardinality,
   requiredness, version applicability, default/derivation rule).
4. Parse-to-simulation propagation map table.
5. State ownership and mutability rules.
6. Derived-value rules and closure hooks.
7. Validation/error taxonomy.
8. Cross-file consistency constraints across primary, watershed, and sidecar
   surfaces (`.run`, `.cli`, `.sol`, `.man`, `.slp`, plus surfaces such as
   `.str`, `.chn`, `.imp`, irrigation sidecars, `pmetpara.txt`, `snow.txt`,
   and `frost.txt` where applicable).
9. Backward-compatibility policy for legacy text sidecars.
10. Guard map linking parser rules/invariants to enforcement paths.

Drafts that omit any required section are non-compliant and must remain `HOLD`.

## Required Dual-Agent Review Gate

Two independent agent reviews are mandatory for each parser-contract revision.

### Independence Requirements

1. Agent A and Agent B must receive independent review prompts.
2. Agent B must not see Agent A findings before first submission.
3. Reviews must include severity-ranked findings with file/line references.

### Reviewer Output Requirements

Each reviewer output must include:

1. Evidence header (`Static` or `Ran`).
2. Findings ordered by severity.
3. For each finding:
   - severity,
   - file path + line reference,
   - issue statement,
   - why it matters for parser correctness, data propagation, or governance,
   - proposed disposition (`accept`, `amend`, `reject`).
4. Final recommendation:
   - `GO`, `GO-WITH-AMENDMENTS`, or `HOLD`.

## Disposition Workflow (Required)

After both reviews land, author must publish `disposition.md` with one row per
finding.

Required fields:

1. `finding_id`
2. `source` (`agent_a` or `agent_b`)
3. `severity`
4. `decision` (`accepted`, `amended`, `rejected`)
5. `action_taken`
6. `artifact_ref` (file/line or commit reference)
7. `notes`

Rules:

1. Every high-severity finding requires explicit closure evidence.
2. Rejections require rationale tied to contract authority.
3. Silent closure is prohibited.

## Fix Pass and Verification Gate

Verification by agents is a separate hard gate after fixes are applied.

### Verification Requirements

1. Agent A verifies closure of accepted/amended findings.
2. Agent B verifies no new regressions and rejected-finding rationale.
3. Outputs are stored in:
   - `verification_agent_a.md`
   - `verification_agent_b.md`

Each verification output must report:

1. finding closure state (`closed` / `still-open`),
2. disposition-to-implementation consistency,
3. verdict: `PASS`, `PASS-WITH-NOTES`, or `FAIL`.

## Promotion Gate Logic

A parser-contract revision is promotable only if all conditions are true:

1. Draft completeness gate passes.
2. Two independent reviews completed.
3. Disposition completed with no missing finding rows.
4. All high-severity findings are closed or explicitly justified.
5. Both verification agents return `PASS` or `PASS-WITH-NOTES`.
6. No parser rule/invariant is left without an enforcement/guard mapping.
7. Propagation map contains no unresolved ownership or mutability ambiguity.
8. Surface is recorded in the parser input-surface registry with explicit
   disposition (`active`, `deferred`, or `unsupported`).

If any condition fails, disposition is `HOLD`.

## Minimal Prompt Templates

### Reviewer prompt (A/B)

"Review `SC-INFILE-...` for parser correctness, datver handling,
field-model completeness, parse-to-simulation propagation integrity,
guard mapping, and promotion readiness. Return severity-ranked findings with
file/line references and recommendation: GO / GO-WITH-AMENDMENTS / HOLD."

### Verifier prompt (A/B)

"Given `disposition.md` and updated `SC-INFILE-...`, verify accepted/amended
findings are resolved, rejected findings are justified, and no propagation
regressions were introduced. Return closure status per finding and verdict:
PASS / PASS-WITH-NOTES / FAIL."

## Relationship to Science Contract Procedure

This procedure follows the same governance pattern as:

- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/wepp-input-specification-authoring-procedure.md`

and specializes it for parser-contract data modeling and state propagation
obligations defined in:

- `docs/specifications/wepp-input-files/parser-contract-requirements.md`

## Change Management

1. Any change to this procedure must update linked parser templates/checklists
   in the same commit.
2. Intentional bypasses require explicit risk acceptance in work-package
   disposition artifacts.
