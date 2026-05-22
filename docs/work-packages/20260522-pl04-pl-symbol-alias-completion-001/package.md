# 20260522-pl04-pl-symbol-alias-completion-001

## Status
- state: hold
- date: 2026-05-22
- timezone: UTC

## Objective
Complete PL canonical symbol alias continuity by extending
`openwepp-sim-contract` alias registry for plant/landuse/growth/decomposition
runtime symbols and adding deterministic resolution tests.

## Why This Package Exists
PL02 identified missing PL alias coverage as a hard prerequisite for downstream
PL kernel boundary work. Without PL04, runtime state may drift from canonical
WEPP naming continuity and violate architecture symbol policy.

## Scope
### Included
- Add PL canonical symbols and deterministic boundary aliases to the registry.
- Add alias templates for slot-indexed and OFE-scoped PL surfaces.
- Add tests for forward and reverse alias resolution and ambiguity rejection.
- Document alias table coverage and validation rationale.

### Explicitly Out of Scope
- Parser-to-runtime adapter implementation (PL03).
- Growth/decomposition kernel implementation (PL05/PL06).
- Comparator campaign work (PL08).

## Deliverables
1. Alias expansion contract notes:
   - `artifacts/pl04-symbol-alias-expansion-contract.md`
2. Canonical symbol alias table:
   - `artifacts/pl04-canonical-symbol-alias-table.md`
3. Alias template validation notes:
   - `artifacts/pl04-alias-template-validation-notes.md`
4. Alias registry test evidence:
   - `artifacts/pl04-alias-registry-test-evidence.md`
5. Package governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/pl04_disposition.md`
6. Dual review/verification artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Dependencies
- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-pl02-plant-runtime-boundary-contract-001/package.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-pl02-plant-runtime-boundary-contract-001/artifacts/pl-runtime-canonical-symbol-alias-requirements.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-pl02-plant-runtime-boundary-contract-001/artifacts/pl02-follow-on-implementation-handoff.md`
- `/home/workdir/openWEPP/crates/openwepp-sim-contract/src/symbols.rs`

## Intended Write Set
- `crates/openwepp-sim-contract/**`
- `tests/integration/**` (alias resolution coverage)
- `docs/work-packages/20260522-pl04-pl-symbol-alias-completion-001/**`

## Phase Plan
### Phase 0 - Intake
- Confirm PL02 alias requirements and current registry constraints.

### Phase 1 - Alias Expansion
- Add PL canonical mappings and required templates.

### Phase 2 - Verification
- Add/extend tests for deterministic alias resolution and ambiguity guards.

### Phase 3 - Disposition
- Run required gates and complete review/verification/disposition artifacts.

## Exit Criteria
- PL canonical symbol set is represented in alias registry with deterministic
  template patterns.
- Alias forward/reverse resolution tests pass for added PL coverage.
- No ambiguous back-mapping is introduced.
- If code is changed, run and record:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: contract/alias registry + tests only.

## Execution Result

- Added PL schedule/growth/decomposition canonical alias coverage to
  `openwepp-sim-contract` including deterministic indexed/OFE template forms.
- Added integration tests for PL forward/reverse resolution and ambiguity
  guards; all PL04-owned checks passed.
- Required workspace gates were executed; `cargo test --workspace` and
  `cargo deny check` passed, while `cargo fmt --check` and workspace
  `cargo clippy --workspace --all-targets -- -D warnings` are blocked by
  concurrent PL03-owned drift in
  `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`.
- Package state remains `hold` until full workspace gate replay passes after
  PL03 settles.
