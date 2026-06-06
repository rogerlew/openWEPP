# ADR0017 Comparator Distrust Ratification

Status: complete

This work package is an autonomous ExecPlan and must remain a living document
during execution. It follows `/workdir/openWEPP/docs/codex_exec_plans.md`.

Execution mode: package-end-to-end.

Autonomy: execute all phases through disposition without requesting user
direction unless hard-blocked.

Contract-first sequence: governance contracts, contract-derived tests,
pre-implementation contract gate, then implementation/docs/test edits.

Evidence artifacts must use truthfulness labeling (`Static:` vs `Ran:`).

## Purpose / Big Picture

ADR-0017 restores the openWEPP correctness posture that legacy comparator output
is an investigation flag, not a target. After this package, ADR-0017 is ratified
as accepted governance, comparator/ledger packages have a first-class
`HARNESS-SURFACE-MISMATCH` verdict, openWEPP-defect labels require like-for-like
unit/lineage evidence plus independent correctness authority, and stale
unowned `HOLD` findings are prohibited while truthful owned `HOLD` remains
valid.

## Progress

- [x] (2026-06-05) Scaffolded ADR0017 ratification work package.
- [x] (2026-06-05) Ratified ADR-0017 and synchronized ADR index/back-reference
  status.
- [x] (2026-06-05) Amended canonical governance and affected snow/water
  contracts.
- [x] (2026-06-05) Added contract-derived ratification tests.
- [x] (2026-06-05) Dispatched and dispositioned dual reviews.
- [x] (2026-06-05) Ran focused validation and authority anti-evasion guards.
- [x] (2026-06-05) Completed verification, disposition, and worker handoff
  artifacts.

## Surprises & Discoveries

- Observation: HPHYS0298 has already been retracted in-package after the
  unit-artifact review.
  Evidence:
  `docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/artifacts/disposition.md`.
- Observation: Review found stale HPHYS0296-0298 three-verdict taxonomy in
  `SC-SNOWFREEZE-001` and `SC-WATBAL-001`.
  Evidence: reviewer B artifact for this package.

## Decision Log

- Decision: Ratification scope is governance-only plus contract/test gates; no
  production physics edits are authorized.
  Rationale: ADR-0017 changes comparator interpretation and evidence standards,
  not process equations.
  Date/Author: `2026-06-05` / `Codex`.
- Decision: HPHYS0296-0298 historical ledger invariants are amended in-place to
  reference the ADR0017 peer taxonomy instead of relying only on a new
  superseding invariant.
  Rationale: Leaving stale three-verdict rows created local contradiction for
  future packages.
  Date/Author: `2026-06-05` / `Codex`.

## Outcomes & Retrospective

ADR0017 ratification is complete. The package accepted and addressed all review
and verification findings, added canonical comparator-flag governance,
installed `SC-SNOWFREEZE-001#INV-SNOWFREEZE-039` and
`SC-WATBAL-001#INV-WATBAL-087`, hardened HPHYS0296-0298 historical taxonomy,
and registered a contract-derived regression test that rejects placeholder
closeout artifacts.

## Context and Orientation

openWEPP is the Rust simulation engine. ADR-0011 says science contracts and
conservation/physics evidence govern correctness, while legacy comparator deltas
are confidence-tiered investigation evidence. ADR-0016 accepted a fixed
`wepp_260430` comparator for the negative-melt bug so that corrected
negative-melt behavior no longer creates a dual-authority conflict. ADR-0017
keeps that fixed comparator but explicitly demotes comparator agreement from a
target to a flag.

The key governance problem is that HPHYS0298 through HPHYS0313 repeatedly treated
comparator differences as presumptive openWEPP defects even when the paired
surfaces were not proven like-for-like. HPHYS0298 compared baseline snowfall
depth `hrsnow` with openWEPP water-equivalent output and has now been retracted
in-package. HPHYS0313 caught another branch/surface misattribution around
`hrsnow` versus `driftg`.

## Included Scope

- Accept ADR-0017 and update ADR-0016 / decision index references.
- Amend governance docs so comparator/ledger packages require:
  `HARNESS-SURFACE-MISMATCH`, `LEGACY-DEFECTIVE`, `OPENWEPP-DEFECTIVE`, and
  `UNRESOLVED` verdicts; like-for-like unit/lineage proof before openWEPP-defect
  labels; independent correctness authority with no criterion-C waiver; and
  owned `HOLD` records.
- Amend `SC-SNOWFREEZE-001` and `SC-WATBAL-001` with ADR0017 governance
  invariants for snow/`RM` continuation and water-balance residual routing.
- Add a focused Rust integration test that fails before these governance surfaces
  exist and passes after ratification.
- Record evidence, review/disposition, verification, gates, and handoff.

## Intended Write Set

- `docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md`
- `docs/decisions/0016-promote-260430-baseline-as-canonical-comparator-and-abandon-kernel-rewrite.md`
- `docs/decisions/README.md`
- `docs/codex_exec_plans.md`
- `docs/specifications/correctness-authority-model.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/unit-governance.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260605-adr0017-comparator-distrust-ratification-001/**`
- `Cargo.toml`
- `tests/integration/adr0017_comparator_distrust_ratification_contract.rs`
- `tests/integration/hphys0313_snowpack_settling_carry_recursion_contract.rs`
  (contract-version assertion follow-through only)

## Excluded Scope

- No production Rust kernel edits.
- No production kernel/runtime physics edits.
- No full H1..H39 rerun.
- No attempt to reclassify every historical ledger row in this package. This
  package ratifies the governance gate and records follow-on reclassification
  obligations.
- No remote comparator branch or tag maintenance.

## Security-Impact Gate

This package is governance/docs/test-only. It does not edit runtime production
code, parser paths, subprocess invocation, network behavior, credentials,
serialization formats, or external I/O. Security impact is `none` unless
validation surfaces an unexpected production-code diff.

## Dependencies

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `docs/work-packages/README.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0016-promote-260430-baseline-as-canonical-comparator-and-abandon-kernel-rewrite.md`
- `docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md`
- `docs/specifications/correctness-authority-model.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/unit-governance.md`
- `docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/artifacts/review-disposition.md`
- `docs/work-packages/20260605-hphys0299-hourly-snow-partition-unit-provenance-closure-001/artifacts/disposition.md`
- `docs/work-packages/20260605-hphys0313-snowpack-settling-carry-recursion-closure-001/artifacts/review_claude_settling_route_misattribution.md`

## Plan of Work

First, update ADR status and registry references. Second, amend canonical
governance documents and affected snow/water contracts with explicit ADR0017
verdict, like-for-like, independent-authority, and owned-HOLD gates. Third, add
a contract-derived integration test registered in `Cargo.toml`. Fourth, validate
with focused docs/tests and source-level anti-evasion guards. Fifth, complete
artifacts with evidence, dual review, review disposition, dual verification,
and handoff.

## Validation and Acceptance

Acceptance requires:

1. ADR-0017 is `Accepted`, ADR-0016 records accepted amendment, and the decisions
   README marks ADR-0017 accepted.
2. Governance docs and affected contracts contain `HARNESS-SURFACE-MISMATCH`,
   like-for-like unit/lineage proof, no criterion-C waiver for
   `OPENWEPP-DEFECTIVE`, and owned `HOLD` requirements.
3. `cargo test --test adr0017_comparator_distrust_ratification_contract -- --nocapture`
   passes.
4. `markdown-doc lint --path docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md`
   passes.
5. Authority anti-evasion guards pass.

## Idempotence and Recovery

All changes are flat-file documentation/test edits in the openWEPP worktree.
Rerun validation commands safely. If a ratification edit must be reverted, use
`git checkout -- <path>` before commit; no external systems are modified.
