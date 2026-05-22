# 20260522-inimpl31-implement-sc-infile-hbp-parser-001

## Status
- state: active
- date: 2026-05-22
- timezone: UTC

## Objective
Implement a first-class openWEPP HBP reader/parser surface with owned specification,
owned parser contract, typed parser implementation, and integration tests aligned
with existing `SC-INFILE-*` implementation patterns.

## Why This Package Exists
HBP (`H<hillslope_id>.hbp`) is now a core simulation-boundary input surface for
watershed routing and replay workflows. openWEPP needs explicit ownership of HBP
reader behavior and typed contracts rather than relying on implicit legacy bridge
handling.

## Scope
### Included
- Author canonical HBP surface specification under `docs/specifications/wepp-input-files/specs/`.
- Author parser contract `SC-INFILE-HBP-001` under `docs/specifications/science-contracts/contracts/`.
- Register HBP in input-surface registry.
- Wire and ship `openwepp-input-contract::parsers::hbp` as an exported parser surface.
- Add HBP integration tests (schema1 + schema2 + strict/compat path policy + typed failures).
- Produce INIMPL31 closeout artifacts.

### Explicitly Out of Scope
- Watershed shard-set manifest orchestration logic (`HBP2-R01..R08`) beyond parser-local surface.
- Legacy bridge deprecation/removal.
- Vendoring reconciliation for `wepppyo3`.

## Deliverables
1. `docs/specifications/wepp-input-files/specs/hbp-file.spec.md`
2. `docs/specifications/science-contracts/contracts/SC-INFILE-HBP-001.md`
3. `crates/openwepp-input-contract/src/parsers/hbp.rs` integrated and exported
4. `tests/integration/infile_hbp_parser_contract.rs`
5. Registry updates in `docs/specifications/wepp-input-files/input-surface-registry.md`
6. Required artifacts in this package `artifacts/` directory

## Dependencies
- `/workdir/wepp-forest/docs/contracts/hillslope-binary-pass-format.md`
- `/workdir/wepp-forest/docs/contracts/watershed-hillslope-pass-reader-contract.md`
- `/workdir/wepppyo3/wepp_interchange/src/hill_hbp.rs`
- `/home/workdir/openWEPP/docs/specifications/wepp-input-file-parser-contract-authoring-procedure.md`

## Intended Write Set
- `crates/openwepp-input-contract/Cargo.toml`
- `crates/openwepp-input-contract/src/parsers/mod.rs`
- `crates/openwepp-input-contract/src/parsers/hbp.rs`
- `Cargo.toml`
- `tests/integration/infile_hbp_parser_contract.rs`
- `docs/specifications/wepp-input-files/specs/hbp-file.spec.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-HBP-001.md`
- `docs/specifications/wepp-input-files/input-surface-registry.md`
- `docs/work-packages/README.md`
- package-local files under this work-package directory

## Phase Plan
### Phase 0 - Provenance and Contract Mapping
- Map wepp-forest HBP contract invariants + wepppyo3 reader behavior to parser-local guard surfaces.

### Phase 1 - Parser Integration
- Export and dependency-wire parser module.
- Ensure strict/compat naming policy and typed errors are deterministic.

### Phase 2 - Surface Tests
- Add deterministic fixture synthesis and parser-contract integration tests.
- Cover schema1 and schema2 parse-success plus typed failure branches.

### Phase 3 - Contract + Spec Authoring
- Author canonical HBP spec and `SC-INFILE-HBP-001` parser contract.
- Register surface in input-surface registry.

### Phase 4 - Closeout
- Run gates and produce handoff/disposition/review/verification artifacts.

## Exit Criteria
- HBP parser is exported, tested, and passing workspace gates.
- HBP spec and parser contract are authored and linked.
- Input-surface registry includes HBP with active contract mapping.
- No unresolved high-severity findings remain.

## Security Impact and Review Gate
- security_impact: none
- dedicated_security_review_required: no
- Rationale: parser-surface + docs/governance changes only.
