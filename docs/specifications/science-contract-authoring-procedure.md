# Science Contract Authoring Procedure

Status: Active
Last updated: 2026-07-27
Scope: openWEPP process-based science contracts (`SC-<DOMAIN>-<NNN>`)

## Purpose

Define the mandatory workflow for authoring, reviewing, promoting, and changing
openWEPP science contracts. Artifact structure is defined separately in
`docs/specifications/science-contract-spec.md`; provenance sidecar structure and
lifecycle are defined in
`docs/specifications/science-contract-provenance-spec.md`.

Principle: correctness over completion. Contract work remains in `HOLD` until
correctness criteria, review disposition, and verification gates are satisfied.
Schedule pressure is not a valid reason to bypass invariant correctness.

This procedure is normative for contract promotion readiness and complements:

- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0003-parity-semantic-not-bit.md`
- `docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md`
- `docs/specifications/science-contract-spec.md`
- `docs/specifications/science-contract-provenance-spec.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/unit-governance.md`
- `docs/specifications/correctness-authority-model.md`
- `docs/specifications/external-authority/README.md`
- `docs/specifications/science-contracts/README.md`

## Authority and Evidence Rules

1. Contract derivation order is fixed:
   1. WEPP technical references, including `references/50201000`.
   2. Peer-reviewed literature invariants.
   3. Physical/common-sense invariants.
   4. Static legacy code inspection as secondary provenance only.
2. Legacy static-code provenance defaults to the pinned baseline in
   `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`:
   `/workdir/wepp-forest_260430_baseline` at commit
   `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
3. Citations to a different legacy snapshot must include explicit commit SHA and
   rationale in the contract gap/disposition register.
4. Every non-trivial invariant must include explicit citation anchors.
5. Evidence tags are required per claim: `[DIRECT]` for directly observed facts
   and `[INFERENCE]` for reasoned interpretation.
6. Evidence mode at document/review level must be explicit: `Static` for
   read/reasoned work and `Ran` for executed-command/runtime evidence.
7. Every invariant must have an explicit guard mapping: runtime guard, typed
   failure, explicit branch, or governance guard with an explicit `HOLD` rule.
8. Variable-symbol continuity is mandatory: canonical contract symbols default
   to WEPP/legacy names; differing openWEPP boundary names require explicit alias
   mappings.
9. Legacy-migration process-physics fidelity is mandatory: production
   implementations for in-scope migrated physics must port baseline-authoritative
   behavior. Surrogate/proxy/heuristic formulas are not promotable closure unless
   explicitly documented as non-promotable research branches under `HOLD`.
10. Science authority, data authority, and calibration readiness remain
    distinct under ADR-0042. Insufficient measured data limits calibration,
    identifiability, validation, and transferability claims; it does not
    authorize proxy physics or block implementation of available authoritative
    science.
11. Applicable parameterized contracts must state calibration applicability,
    observation requirements/operator, identifiable combinations, evidence
    gaps, readiness obligations, and prohibited claims. Synthetic recovery and
    execution assumptions must never be labeled empirical calibration.
12. Calibration, independent-validation, and diagnostic observation roles must
    be assigned prospectively. Calibration evidence cannot also carry an
    independent-validation claim.

## Canonical Locations

Canonical `SC-*` authority files must live in:

- `docs/specifications/science-contracts/contracts/SC-<DOMAIN>-<NNN>.md`

Canonical lifecycle registry must live in:

- `docs/specifications/science-contracts/index.md`

Provenance sidecars, when used, must follow
`docs/specifications/science-contract-provenance-spec.md`. Work-package artifacts
are evidence and workflow records, not authority replacements.

## Required Contract Cycle Layout

All review, disposition, and verification artifacts for a contract cycle must
live under one work package. Suggested layout:

- `docs/work-packages/<wp>/artifacts/science-contracts/<contract_id>/contract_ref.md`
- `docs/work-packages/<wp>/artifacts/science-contracts/<contract_id>/review_agent_a.md`
- `docs/work-packages/<wp>/artifacts/science-contracts/<contract_id>/review_agent_b.md`
- `docs/work-packages/<wp>/artifacts/science-contracts/<contract_id>/disposition.md`
- `docs/work-packages/<wp>/artifacts/science-contracts/<contract_id>/verification_agent_a.md`
- `docs/work-packages/<wp>/artifacts/science-contracts/<contract_id>/verification_agent_b.md`

`contract_ref.md` and `disposition.md` must reference the canonical contract path
and commit SHA under review.

## Authoring Workflow

1. Confirm the work package authority envelope, protected boundaries, and
   intended write set before editing canonical contracts.
2. Amend canonical `SC-*` authority before production kernel/runtime edits when
   contract authority is applicable.
3. Add or update contract-derived tests before production kernel/runtime edits.
4. Record pre-implementation contract-gate evidence before production edits.
5. Modify production code only after the contract and test authority gates are
   satisfied.
6. Update work-package evidence truthfully using `Static:` and `Ran:` labels.
7. Do not close a package while known invariant, closure, or contract violations
   remain unresolved.
8. When authoritative science exists but measured data are insufficient,
   implement the science and complete applicable calibration-readiness
   obligations. Use a data-limited disposition rather than a science-authority
   hold unless a required readiness defect remains unresolved.
9. Retain a readiness matrix with every schema obligation marked `PASS`,
   `BLOCKED`, or `NOT_APPLICABLE`, plus evidence path and rationale. A required
   current-scope `BLOCKED` row prevents completion.

## Binding Exposure Workflow

Contracts with historical or package-local addenda must expose binding residue in
the core contract before narrative can be moved to a provenance sidecar.

1. Build or update the contract's `Binding Exposure Index` as defined in
   `docs/specifications/science-contract-spec.md`.
2. Classify each addendum or sidecar entry as `active`, `superseded`, or
   `historical` using the provenance lifecycle rules.
3. Map every binding addendum residue to canonical `INV-*` or `OBL-*` IDs.
4. Treat any active binding residue without an existing canonical ID as a flagged
   binding addition. It requires dual review, finding disposition, and
   verification before promotion.
5. Do not remove, weaken, or hide an invariant by relocating narrative. If the
   binding set cannot be conserved without a science decision, keep the package
   in `HOLD` and route a follow-on.
6. Future addenda belong in the provenance sidecar by default; binding residue
   must be promoted into core invariants/obligations and indexed.

## ADR-0017 Comparator Governance

This is the `ADR-0017 comparator-distrust governance` adjudication section for comparator
flags and verdict alignment.

Comparator agreement is an investigation flag, not an acceptance target.
Comparator/ledger contract work must satisfy these rules before assigning defect
ownership:

1. `OPENWEPP-DEFECTIVE` verdicts require like-for-like unit and lineage-stage
   proof plus independent correctness authority.
2. Independent correctness authority may not be waived for an
   `OPENWEPP-DEFECTIVE` verdict.
3. The `HARNESS-SURFACE-MISMATCH` verdict is a peer verdict for unit or surface-pairing
   defects.
4. Depth-vs-water-equivalent, raw-vs-released, lineage-stage mismatch, or a
   suspicious conversion-like ratio such as approximately `10x` for snow depth
   or approximately `1000x` for meters versus millimetres resolves to
   `HARNESS-SURFACE-MISMATCH` or `UNRESOLVED`, not an openWEPP defect.
   (These are the accepted scale-threshold heuristics.)
5. `HOLD` is valid only when it records an owner/follow-on package, next
   evidence gate, and reason closure is blocked.
6. Criterion-C-style independent correctness authority may not be waived for
   comparator-flag decisions.

## Symbol Alias and Unit Governance Workflow

1. Apply `docs/specifications/unit-governance.md` before claiming promotion
   readiness.
2. Use canonical WEPP symbols as primary IDs in `Variables and Units`.
3. Include alias mappings whenever external names differ across Rust structs,
   JSON, CLI, sidecars, parquet columns, or publication names.
4. Fail closed on missing unit declarations, ambiguous aliases, unguarded unit
   conversions, or suspicious dimensional ratios until like-for-like evidence is
   recorded.
5. Unit conversions must be named, directional, provenance-backed, and tested.
   Raw dimensional literals are non-compliant unless allowlisted with provenance
   and follow-up disposition.
6. Publication metadata must trace to the same unit authority as runtime symbols.

## Kernel Profile Gate

If a package changes production kernel behavior or runtime projection semantics
that control kernel branch execution, the contract revision must satisfy:

- `docs/specifications/science-contracts/kernel-process-contract-profile.md`

Missing kernel-profile compliance keeps disposition in `HOLD`.

## External Constitutive Suite Gate

For kernel-affecting packages that touch process families with external
constitutive suites:

1. Reference applicable suite IDs and linked invariant IDs (`SC-*#INV-*`).
2. Add suite-required fixture/tolerance assertions before production code
   changes.
3. Ensure suite metadata conforms to
   `docs/specifications/external-authority/suite-schema.md`.
4. Treat legacy parity comparators as investigation evidence only; they may not
   replace constitutive suite obligations for acceptance.
5. Run required anti-evasion checks when touching suite posture, cohort fixtures,
   or required-case bindings.

## Required Dual-Agent Review Gate

Two independent agent reviews are mandatory for every contract revision.

1. Agent A and Agent B receive independent prompts.
2. Agent B is not primed with Agent A findings before submitting its first
   review.
3. Reviews include an evidence header (`Static` or `Ran`), severity-ranked
   findings with file/line references, scientific/governance impact, proposed
   disposition, and final recommendation: `GO`, `GO-WITH-AMENDMENTS`, or `HOLD`.

## Disposition Workflow

After both reviews land, produce `disposition.md` with one row per finding.
Each row must include:

- `finding_id`
- `source` (`agent_a` or `agent_b`)
- `severity`
- `decision` (`accepted`, `amended`, `rejected`, `deferred`, or `follow-up`)
- `action_taken`
- `artifact_ref`
- `rationale`

`amended` is retained as a compatibility value for older contract-review artifacts and means accepted with an amended fix path. Accepted and amended findings must be fixed and verified. Rejected findings require rationale
tied to contract authority. Deferred or follow-up findings must be linked from
disposition and worker-handoff artifacts. No silent closure is allowed.

## Mandatory Verification Gate

After fixes are applied, agent verification is a separate hard gate.

1. Agent A verifies closure of accepted findings.
2. Agent B verifies no new regressions and validates rejected-finding rationale.
3. Each verification output states which findings are `closed` or `still-open`,
   whether implementation matches disposition claims, and a verdict of `PASS`,
   `PASS-WITH-NOTES`, or `FAIL`.

## Promotion Gate Logic

A contract revision is promotable only if all conditions are true:

1. Two independent reviews completed.
2. Disposition completed with no missing finding rows.
3. Accepted findings are fixed and verified.
4. Rejected findings carry authority-backed rationale.
5. Both verification agents return `PASS` or `PASS-WITH-NOTES`.
6. Remaining open items are listed in the gap register with non-promotable
   labeling when applicable.
7. No invariant is left without declared guard mapping and enforcement path.
8. For legacy migration packages, touched process-physics routines include
   baseline source-to-openWEPP provenance mapping evidence.
9. Binding Exposure Index checks pass when contract addenda or provenance
   sidecars are present.

If any condition fails, disposition is `HOLD`.

## Change Management

1. Any change to this procedure must update linked checklists/templates in the
   same work package.
2. Changes to artifact shape belong in `science-contract-spec.md`; changes to
   sidecar lifecycle belong in `science-contract-provenance-spec.md`.
3. If procedure requirements are intentionally bypassed for urgent research
   reasons, the bypass must be documented in the work-package disposition with
   explicit risk acceptance.

## Minimal Prompt Templates

Reviewer prompt:

`Review SC-... for scientific authority alignment, invariant soundness, evidence-label correctness, Binding Exposure Index conservation, and promotion readiness. Return severity-ranked findings with file/line references and final recommendation: GO / GO-WITH-AMENDMENTS / HOLD.`

Verifier prompt:

`Given disposition.md and the updated SC-... contract, verify each accepted or rejected finding disposition and check for regressions. Return closure status per finding and verdict: PASS / PASS-WITH-NOTES / FAIL.`
