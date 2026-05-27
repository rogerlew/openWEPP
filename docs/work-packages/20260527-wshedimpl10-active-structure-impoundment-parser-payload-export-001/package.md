# 20260527-wshedimpl10-active-structure-impoundment-parser-payload-export-001

## Status
- state: package-complete-with-hold
- date: 2026-05-27
- timezone: UTC
- decision: HOLD

## Objective
Execute WSHED10 by implementing active-structure branch payload export in the
watershed impoundment parser and validating parser/runtime seam behavior for
`GAP-IMPOUND-006` and `GAP-SYSTEM-007`.

## Why This Package Exists
WSHED09 retained HOLD with unresolved active-structure impoundment closure.
`GAP-IMPOUND-006` and `GAP-SYSTEM-007` explicitly identified missing parser
branch-payload export as a blocker to active-structure coefficient projection.

## Scope
### Included
- Extend `.imp` parser model to retain typed payloads for active structures:
  - drop spillway,
  - culvert 1/2,
  - rockfill,
  - emergency spillway,
  - filter barrier,
  - perforated riser.
- Add parser contract-derived fixture/test coverage proving active payload export
  surfaces are preserved and typed.
- Re-anchor runtime seam fail-closed messaging to reflect that parser payloads
  are now exported, while active coefficient projection remains unimplemented.
- Update canonical contract gap text and registry notes for WSHED10 evidence.
- Run required validation gates and capture artifact evidence.

### Explicitly Out of Scope
- Full active-structure coefficient derivation/runtime projection into
  `ws10_impoundment_{id}_{a,b,c,d,e,ha,ht,hlm}`.
- Full WS12 active-structure physics parity migration in watershed kernel
  outflow composition.
- Channel sediment parity (`GAP-SYSTEM-008` / `GAP-ROUTE-009` / `GAP-SED-006`).

## Deliverables
1. `artifacts/wshedimpl10-watershed-validation-and-comparator-rerun-report.md`
2. `artifacts/wshedimpl10-hold-lift-decision-report.md`
3. `artifacts/wshedimpl10-contract-implementation-evidence.md`
4. `artifacts/wshedimpl10-contract-test-implementation-evidence.md`
5. `artifacts/wshedimpl10-preimplementation-contract-gate.md`
6. `artifacts/wshedimpl10-implementation-and-test-evidence.md`
7. `artifacts/wshedimpl10-kernel-profile-compliance-checklist.md`
8. `artifacts/owned-file-manifest.md`
9. `artifacts/gate-results.md`
10. `artifacts/wshedimpl10_disposition.md`
11. `artifacts/worker-handoff.md`
12. `artifacts/review_agent_a.md`
13. `artifacts/review_agent_b.md`
14. `artifacts/verification_agent_a.md`
15. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Amend canonical contract gap language (`SC-IMPOUND-001`, `SC-SYSTEM-001`,
   registry note) to reflect WSHED10 scope.
2. Add contract-derived parser test coverage for active payload export.
3. Record pre-implementation contract gate evidence.
4. Implement parser/runtime seam code changes.

## Autonomous Execution Intent (Required)
This package executes end-to-end through implementation, validation, and
disposition without requesting additional user direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts include explicit `Static:` and/or `Ran:` labels.

## Provenance and Authority Posture
- Canonical authority remains in
  `docs/specifications/science-contracts/contracts/SC-*.md`.
- Work-package artifacts are evidence, not authority replacement.
- No surrogate process-physics substitution is introduced in production kernel
  paths.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl09-end-to-end-validation-comparator-rerun-and-hold-lift-disposition-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-IMPOUNDMENT-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/watershed_impoundment.rs`
- `/workdir/openWEPP/crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`
- `/workdir/openWEPP/tests/integration/infile_watershed_impoundment_parser_contract.rs`
- `/workdir/openWEPP/tests/fixtures/infile/watershed_impoundment/strict_valid_active_payloads.imp`

## Intended Write Set
- `docs/work-packages/20260527-wshedimpl10-active-structure-impoundment-parser-payload-export-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-input-contract/src/parsers/watershed_impoundment.rs`
- `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`
- `tests/integration/infile_watershed_impoundment_parser_contract.rs`
- `tests/fixtures/infile/watershed_impoundment/strict_valid_active_payloads.imp`

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm WSHED10 authorization from WSHED09 handoff and queue posture.

### Phase B - Contract/test prep
- Add active-payload export test vector(s) in parser integration tests.
- Record pre-implementation contract gate evidence.

### Phase C - Parser/runtime seam implementation
- Extend parser structures and parsing logic to retain active branch payload
  surfaces.
- Preserve fail-closed runtime seam behavior for still-unimplemented active
  coefficient projection with updated truthful error wording.

### Phase D - Validation and governance evidence
- Execute required tests and package gates.
- Update contract gap rows and registry notes.

### Phase E - Disposition and handoff
- Publish HOLD/GO decision with residual ownership and follow-on package scope.

## Exit Criteria
- Active-structure parser payloads are retained in typed parse outputs.
- Contract-derived parser test vector for active payload export passes.
- Runtime seam test still fails closed for unimplemented active coefficient
  projection with updated truthful message.
- Required validation gates are executed and recorded.
- Contract gap language is updated to reflect post-WSHED10 posture.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: parser/seam and docs/test updates only; no external connectivity or
  secret-surface changes.

## Execution Outcome Summary
- WSHED10 parser model now exports active-structure branch payloads for drop,
  culvert, rockfill, emergency, filter, and riser sections.
- New active-fixture parser contract vector validates typed export surfaces.
- Runtime seam retains fail-closed active-structure projection posture; message
  now accurately states projection is unimplemented rather than parser payload
  absence.
- Disposition remains `HOLD` pending follow-on active-structure coefficient
  projection/runtime migration package(s).
