# 20260521-inimpl09-management-full-typed-datamodel-001

## Status
- `state`: active
- `date`: 2026-05-21
- `timezone`: UTC

## Objective
Close the management parser HOLD path by implementing a full typed `.man`
datamodel in openWEPP, with aligned spec/contract semantics, parser behavior,
fixtures, and integration tests.

## Why This Package Exists
`INIMPL06` delivered a control-surface parser baseline but intentionally rejects
non-zero scenario sections (`ncrop`, `nop`, `nini`, `nseq`, `ncnt`, `ndrain`,
`nscen`). That blocks canonical management files and prevents downstream
parameter-specific access through typed section/scenario structures.

## Scope
### Included
- Ratify full typed management datamodel requirements for `infile-management-man`
  across:
  - information/header surfaces,
  - section scenario registries (plant/op/initial/surface/contour/drain/yearly),
  - management loop schedule expansion,
  - scenario reference closure and date-domain guards.
- Update management input specification:
  - `docs/specifications/wepp-input-files/specs/plant-file.spec.md`
- Update parser contract:
  - `docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md`
- Implement parser and typed output model in:
  - `crates/openwepp-input-contract/src/parsers/management.rs`
  - supporting parser/model files under
    `crates/openwepp-input-contract/src/parsers/` as needed
- Add/expand fixtures and integration tests:
  - `tests/fixtures/infile/management/**`
  - `tests/integration/infile_management_parser_contract.rs`
- Run and record required gates (`fmt`, `clippy`, `test`, `deny`).
- Produce review/disposition/verification artifacts for closure.

### Explicitly Out of Scope
- Watershed parser surfaces and channel-bundle management variants.
- Runtime mutator DSL parity with wepppy (`__setitem__`-style bulk override API).
- Legacy downgrade/export tooling (for example `2016.3+` to `98.4` conversion).
- Rangeland process-kernel implementation (openWEPP does not implement
  rangeland simulation behavior).

## Dependencies
- `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/plant-file.spec.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md`
- `/home/workdir/openWEPP/docs/work-packages/20260521-inimpl06-implement-sc-infile-management-parser-001/`
- `/home/workdir/openWEPP/docs/work-packages/20260520-infile03-author-sc-infile-management-001/`
- `/home/workdir/openWEPP/references/vendorable/usersum2024.pdf`
- `/home/workdir/wepp-forest/src/infile.for`
- `/home/workdir/wepp-forest/src/tilage.for`
- `/home/workdir/wepppy/wepppy/wepp/management/managements.py`
- `/home/workdir/wepppy/wepppy/wepp/management/data` (management fixture corpus source)

## Intended Write Set
- `docs/specifications/wepp-input-files/specs/plant-file.spec.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md`
- `crates/openwepp-input-contract/src/parsers/management.rs`
- optional support files under `crates/openwepp-input-contract/src/parsers/`
- `tests/integration/infile_management_parser_contract.rs`
- `tests/fixtures/infile/management/**`
- package-local artifacts under this work-package directory

## Phase Plan
### Phase 0 - Evidence Freeze and Gap Baseline
- Snapshot current management parser limitations and open HOLD findings.
- Produce datamodel decomposition artifact mapping legacy symbols to typed
  openWEPP structs without replacing canonical symbols.

### Phase 1 - Spec and Contract Alignment
- Update `.man` spec and contract to define executable typed datamodel minimums
  per section and datver branch.
- Reconcile guard map coverage (`G-MAN-001..G-MAN-008`) with explicit invariant
  checks and typed error taxonomy.
- Disposition any stale/misaligned HOLD statements that conflict with current
  implementation intent.
- Codify non-goal policy for rangeland (`landuse=2`) so behavior is explicit:
  no rangeland simulation path in openWEPP; parser/contract must either reject
  with typed unsupported errors or mark as non-executable at boundary export.

### Phase 2 - Parser and Typed Model Implementation
- Implement section grammar parsing for non-zero scenario counts.
- Construct typed scenario registries and schedule graph output.
- Enforce scenario-reference domain closure, section-order closure, and
  date-domain guards (including `G-MAN-008`).
- Preserve strict/compat policy semantics and typed failure behavior.

### Phase 3 - Tests and Fixture Matrix
- Source fixture corpus from `/home/workdir/wepppy/wepppy/wepp/management/data`
  and curate representative `.man` cases into
  `tests/fixtures/infile/management/**` with provenance notes.
- Add positive fixtures for canonical non-zero `.man` structures, including
  cross-section references.
- Add negative fixtures for malformed section arity/order, dangling references,
  and invalid date domains.
- Ensure tests cover at least baseline datver branches used by current
  allowlist (`95.7`, `98.4`, `2016.3`, `2017.1`).

### Phase 4 - Gates and Closeout
- Run:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`
- Produce review/disposition/verification artifacts.

## Deliverables
1. Datamodel decomposition artifact:
   - `artifacts/management-typed-datamodel-decomposition.md`
2. Updated management input specification and parser contract.
3. Parser implementation and typed output model updates.
4. Fixture and integration test expansions.
5. Gate evidence summary:
   - `artifacts/wave-gate-evidence.md`
6. Review and closure artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/inimpl09_disposition.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Exit Criteria
- Parser accepts canonical non-zero `.man` scenario sections with typed output.
- Typed output includes section registries + schedule surfaces required by
  `SC-INFILE-MANAGEMENT-001`.
- Scenario references and schedule closure invariants are enforced with typed
  errors.
- Date-domain guard coverage is executable (no unresolved `G-MAN-008` gap for
  implemented surfaces).
- Rangeland non-goal stance is codified in spec/contract and represented with
  explicit parser behavior (no implicit rangeland simulation support).
- No unresolved high-severity review findings remain.

## Security Impact and Review Gate
- `security_impact`: `none`
- `dedicated_security_review_required`: `no`
- Rationale: parser/spec/contract/test work only; no network or service
  exposure.
