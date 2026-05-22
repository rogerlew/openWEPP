# Science Contract Authoring Procedure

Status: Active
Last updated: 2026-05-20
Scope: openWEPP process-based science contracts (`SC-<DOMAIN>-<NNN>`)

## Purpose

Define a reusable, mandatory workflow for writing and accepting openWEPP
science contracts with:

1. top-down scientific authority,
2. required dual-agent review,
3. explicit finding disposition,
4. required agent verification of applied fixes.

Principle: correctness over completion. Contract work must remain in `HOLD`
until correctness criteria are satisfied; schedule pressure is not a valid
reason to bypass invariant correctness.

This procedure is normative for contract promotion readiness and complements:

- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0003-parity-semantic-not-bit.md`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/README.md`

## Canonical Contract Location (Normative)

Canonical `SC-*` authority files must be stored in:

- `docs/specifications/science-contracts/contracts/SC-<DOMAIN>-<NNN>.md`

Canonical contract lifecycle registry must be stored in:

- `docs/specifications/science-contracts/index.md`

Work-package artifacts are evidence and workflow records, not canonical
authority location.

## Authority and Evidence Rules

1. Contract derivation order is fixed:
   1. WEPP technical references (including `references/50201000`)
   2. peer-reviewed literature invariants
   3. physical/common-sense invariants
   4. static legacy code inspection (secondary provenance only)
2. Legacy static-code provenance default is the pinned baseline in
   `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`:
   - `/workdir/wepp-forest_260430_baseline`
   - baseline commit:
     `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
   Citations to a different legacy snapshot must include explicit commit SHA and
   rationale in the contract gap/disposition register.
3. Every non-trivial invariant must include explicit citation anchors.
4. Evidence tags are required per claim:
   - `[DIRECT]` for directly observed source/output facts
   - `[INFERENCE]` for reasoned interpretation
5. Evidence mode at document/review level must be explicit:
   - `Static` for read/reasoned work
   - `Ran` for executed-command/runtime evidence
6. Every invariant must have an explicit guard mapping:
   - runtime guard (hard error / typed failure / explicit branch), or
   - governance guard (non-runtime promotion gate with explicit `HOLD` rule).
   Invariants without guard mapping are incomplete and block promotion.
7. Variable-symbol continuity is mandatory:
   - canonical contract symbols default to `wepp-forest` / legacy WEPP names,
   - if openWEPP boundary names differ, contracts must include explicit alias
     mappings from canonical symbols to boundary/API field names.

## Required File Layout Per Contract Cycle

All review/disposition/verification artifacts must live under one work package.

Suggested layout:

`docs/work-packages/<wp>/artifacts/science-contracts/<contract_id>/`

Required files:

1. `contract_ref.md` (path + commit SHA reference to canonical `SC-*.md` file)
2. `review_agent_a.md`
3. `review_agent_b.md`
4. `disposition.md`
5. `verification_agent_a.md`
6. `verification_agent_b.md`

`disposition.md` must reference the canonical path and commit SHA under review.

## Contract Draft Requirements

A contract draft is review-ready only when it contains, at minimum:

1. Stable ID and lifecycle metadata:
   - `contract_id`, `title`, `status`, `maturity`, `owner`, `contract_version`
2. Scientific scope and out-of-scope boundaries.
3. Variable/units table for all externally relevant symbols.
4. Invariant table with stable invariant IDs and citation anchors.
5. Allowed degenerate states and invalid states.
6. Producer obligations and consumer obligations.
7. Boundary disposition definitions per invariant family.
8. Tolerance statement or explicit link to tolerance authority.
9. Gap register for unresolved science or evidence limits.
10. Guard map table linking each invariant ID to its enforcement path
    (runtime guard or governance guard), failure behavior, and gate impact.
11. Symbol alias map table whenever canonical WEPP symbols and openWEPP
    boundary/API names differ.

The draft must exist in the canonical `SC-*` file path before dual-agent review
begins.

## Required Dual-Agent Review Gate

Two independent agent reviews are mandatory for every contract revision.

### Independence Requirements

1. Agent A and Agent B must receive independent review prompts.
2. Agent B must not be primed with Agent A findings before submitting its own
   first review.
3. Reviews must include severity-ranked findings with file/line references.

### Review Output Requirements

Each reviewer output must include:

1. Evidence header (`Static` or `Ran`).
2. Findings ordered by severity.
3. For each finding:
   - severity,
   - file path + line reference,
   - issue statement,
   - why it matters scientifically/governance-wise,
   - proposed disposition (`accept`, `amend`, `reject`).
4. Final recommendation:
   - `GO`, `GO-WITH-AMENDMENTS`, or `HOLD`.

## Disposition Workflow (Required)

After both reviews land, the author must produce `disposition.md` with one row
per finding.

Required disposition fields:

1. `finding_id`
2. `source` (`agent_a` or `agent_b`)
3. `severity`
4. `decision` (`accepted`, `amended`, `rejected`)
5. `action_taken`
6. `artifact_ref` (file/line or commit reference)
7. `notes`

Rules:

1. Every high-severity finding requires explicit decision and action evidence.
2. Rejected findings require rationale tied to contract authority.
3. No silent closure of findings is allowed.

## Fix Pass and Mandatory Agent Verification

After fixes are applied, verification by agents is a separate hard gate.

### Verification Requirements

1. Agent A verifies closure of all accepted/amended findings.
2. Agent B verifies no new regressions and validates rejected-finding rationale.
3. Verification outputs are stored in:
   - `verification_agent_a.md`
   - `verification_agent_b.md`

Each verification output must state:

1. which findings are `closed` / `still-open`,
2. whether fix implementation matches disposition claims,
3. final verification verdict: `PASS`, `PASS-WITH-NOTES`, or `FAIL`.

## Promotion Gate Logic

Contract revision is promotable only if all conditions are true:

1. Two independent reviews completed.
2. Disposition file completed with no missing finding rows.
3. All high-severity findings are closed or explicitly justified with accepted
   authority rationale.
4. Both verification agents return `PASS` or `PASS-WITH-NOTES`.
5. Remaining open items are listed in the gap register with non-promotable
   labeling when applicable.
6. No invariant is left without a declared guard mapping and enforcement path.

If any condition fails, disposition is `HOLD`.

## Symbol Alias Rules (Normative)

1. The `Variables and Units` section in each `SC-*` file must use canonical
   WEPP symbol names (from references and/or `wepp-forest` provenance) as the
   primary symbol IDs.
2. When external names differ (Rust structs, JSON fields, CLI args, parquet
   columns), include an alias map table with at least:
   - canonical symbol,
   - boundary/API name,
   - scope (runtime surface),
   - units check.
3. Symbol substitution without alias documentation is non-compliant and blocks
   promotion.

## Minimal Prompt Templates

### Reviewer prompt (A/B)

"Review `SC-...` contract for scientific authority alignment, invariant
soundness, evidence-label correctness, and promotion-readiness. Return
severity-ranked findings with file/line references and a final recommendation:
GO / GO-WITH-AMENDMENTS / HOLD."

### Verifier prompt (A/B)

"Given `disposition.md` and updated `SC-...` contract, verify each accepted or
amended finding is correctly resolved and check for regressions. Return closure
status per finding and verdict: PASS / PASS-WITH-NOTES / FAIL."

## Change Management

1. Any change to this procedure must update linked checklists/templates in the
   same commit.
2. If procedure requirements are intentionally bypassed for urgent research
   reasons, the bypass must be documented in the work-package disposition with
   explicit risk acceptance.
