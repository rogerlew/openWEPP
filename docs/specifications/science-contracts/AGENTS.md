# docs/specifications/science-contracts/AGENTS.md
> Agent playbook for canonical openWEPP science contracts.

## Authorship
**This document and all AGENTS.md documents are maintained by GitHub Copilot / Codex, which retain full authorship rights for all AGENTS.md content revisions. Agents may author and revise AGENTS.md documents when and where they see fit.** Revisions must preserve applicable user direction, package scope, review expectations, and higher-precedence governance.

## Mission Snapshot
- Own canonical `SC-*` authority for kernel process behavior and governance.
- Preserve contract-first sequencing for kernel-affecting work.
- Keep physics, equations, constants, guards, and invariants traceable to canonical contract text plus provenance citations.
- Prevent package-local notes from becoming hidden authority.

## Primary Assets / Key Files
- `docs/specifications/science-contracts/index.md` — lifecycle registry only; not a changelog substitute.
- `docs/specifications/science-contracts/contracts/SC-*.md` — canonical process contract authority.
- `docs/specifications/science-contract-authoring-procedure.md` — authoring and amendment workflow.
- `docs/specifications/science-contracts/kernel-process-contract-profile.md` — kernel-profile requirements.
- `docs/specifications/science-contract-spec.md` — artifact schema and Binding Exposure Index rules.
- `docs/specifications/unit-governance.md` — unit wrapper, conversion, and metadata governance.
- `docs/specifications/correctness-authority-model.md` — comparator/adjudication posture.
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`, `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`, and `docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md`.

## Standard Workflow
1. Read root `AGENTS.md`, this file, and the target `SC-*` contract before edits.
2. For kernel-affecting packages, read the package-local `package.md` and `docs/work-packages/AGENTS.md`.
3. Amend canonical `SC-*` authority before contract-derived tests and production code.
4. Preserve variable naming continuity with legacy WEPP symbols; when runtime names differ, add explicit alias mappings.
5. Keep registry rows short and lifecycle-oriented; amendment history belongs in each contract's `## Change log` or invariant/obligation tables.
6. Record provenance, evidence mode, invariants, guard map, test-vector obligations, gap disposition, and change log updates.

## Contract-First Sequencing
For code-authoring work where contract authority applies:
1. Implement required contract amendments.
2. Implement contract-derived tests.
3. Record pre-implementation contract-gate evidence.
4. Modify production code.

## Physics Authority Rules
- openWEPP is not clean-room; agents may read legacy F90 and existing contracts directly.
- Physics/equation authority defaults to `/workdir/wepp-forest_260430_baseline` at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` unless explicitly justified.
- Do not invent physics or substitute empirical/proxy formulas in production kernel/runtime publication paths.
- Migration closure means porting baseline-authoritative routines into openWEPP architecture with explicit provenance mapping.
- If baseline-authoritative process physics is not yet ported, keep disposition in `HOLD` and open an actionable follow-on; do not close with temporary formulas.

## Domain and Guard Rules
- Invalid, missing, physically impossible, or out-of-contract process state must fail closed with typed guards unless a canonical `SC-*` contract explicitly authorizes bounded tolerance normalization.
- Bounded canonicalization is allowed only for contract-cited roundoff or publication-format normalization with explicit threshold, units, provenance, tests, and evidence.
- Do not change process control flow, hide mass imbalance, replace missing authority, or convert material negative storage/flux/SWE into valid values.
- Removing or loosening a prior fail-closed guard requires contract-first amendment, contract-derived regression tests, before/after evidence, and accepted dual-review disposition.

## Registry Rules
- `index.md` is the lifecycle registry and required-reading entry point.
- Required fields stay intact: `contract_id`, `title`, `status`, `maturity`, `owner`, `path`, `evidence_level`, and `last_reviewed`.
- The optional `notes` field should remain short; do not recreate per-contract amendment logs in the registry.
- Before removing unique narrative from the registry, prove it exists in the owning contract or migrate it into the contract first.

## Validation Checklist
- Contract schema/profile checks required by the package.
- Contract-derived tests for changed invariants, guards, aliases, or obligations.
- Unit-governance checks for runtime symbols, conversions, output metadata, or scalar exceptions.
- Full package gate evidence before closure.

## Common Pitfalls
- Do not treat comparator agreement as a target; `docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md` makes it a flag requiring independent correctness authority before `OPENWEPP-DEFECTIVE` labels.
- Do not classify residuals without like-for-like unit and lineage-stage proof.
- Do not move active binding material into sidecars without a Binding Exposure Index and review gate.
- Do not let package-local addenda become canonical authority without contract integration.

## References
- Work packages: `docs/work-packages/AGENTS.md`.
- Rust production code: `crates/AGENTS.md`.
- Tests: `tests/AGENTS.md`.
