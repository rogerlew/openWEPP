# REFACTOR024 - CLIM06 Frost Integration Test Line-Count Split

Status: complete (executed 2026-06-14)

Package type: mechanical refactor (behavior-preserving)

## Objective

Mechanically split
`tests/integration/clim06_frost_frozen_soil_kernel_contract.rs` so the 2000+
line integration test no longer carries line-count WARN debt, while preserving
all test assertions, fixture values, helper semantics, contract coverage, and
frost frozen-soil kernel behavior.

## Rationale

The target file is 2743 lines. It is below the 3000-line mandatory-refactor
threshold, but above the 2000-line WARN threshold. The file combines the CLIM06
fixture seed, shared frost helpers, and multiple independent contract-test
clusters. A mechanical module split improves reviewability without changing
science contracts or production code.

## Refactor Seam Declaration

Source file:

- `tests/integration/clim06_frost_frozen_soil_kernel_contract.rs`

Target layout:

- `clim06_frost_frozen_soil_kernel_contract.rs`: integration-test crate root
  with explicit `#[path = "..."]` module declarations only.
- `clim06_frost_frozen_soil_kernel_contract/support.rs`: shared imports,
  fixture seed data, execution helpers, and assertion helpers.
- `clim06_frost_frozen_soil_kernel_contract/contract_gates.rs`: CLIM06,
  SIMIMPL33, and early FDHP01 contract gate tests.
- `clim06_frost_frozen_soil_kernel_contract/fine_layer.rs`: fine-sublayer,
  frwatc, C1b, and C2 exchange tests.
- `clim06_frost_frozen_soil_kernel_contract/thermal_front.rs`: FDHP01 heat-flow,
  lower-front, upper-front, and thaw/freeze progression tests.
- `clim06_frost_frozen_soil_kernel_contract/publication.rs`: SIMIMPL32/FQ4
  publication, layered-store authority, lineage, and hard-fail seam tests.

Public surface expected to remain stable:

- Integration-test function names and assertions.
- `cargo test --test clim06_frost_frozen_soil_kernel_contract` test coverage.
- No production public API changes.

## Included Scope

- Mechanical movement of test blocks and shared helper blocks into child
  modules.
- Minimal `pub(super)` visibility needed for sibling test modules.
- Mechanical formatting only.
- Package artifacts, review, verification, gates, and disposition.

## Excluded Scope / Protected Boundaries

- No process-physics formula, constant, threshold, unit, fixture, or guard
  changes.
- No science-contract amendment.
- No production-code changes.
- No comparator tuning or new validation cohort.
- No new tests beyond preserving the existing coverage under the split layout.

## Deliverables

1. Test refactor:
   - `tests/integration/clim06_frost_frozen_soil_kernel_contract.rs`
   - `tests/integration/clim06_frost_frozen_soil_kernel_contract/support.rs`
   - `tests/integration/clim06_frost_frozen_soil_kernel_contract/contract_gates.rs`
   - `tests/integration/clim06_frost_frozen_soil_kernel_contract/fine_layer.rs`
   - `tests/integration/clim06_frost_frozen_soil_kernel_contract/thermal_front.rs`
   - `tests/integration/clim06_frost_frozen_soil_kernel_contract/publication.rs`
2. Package catalog update:
   - `docs/work-packages/README.md`
3. Package artifacts under `artifacts/`.

## Intended Write Set

- `docs/work-packages/20260614-refactor024-clim06-frost-test-line-count-split-001/**`
- `docs/work-packages/README.md`
- `tests/integration/clim06_frost_frozen_soil_kernel_contract.rs`
- `tests/integration/clim06_frost_frozen_soil_kernel_contract/**`

## Dependencies

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/README.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `tests/AGENTS.md`

## Phase Plan

### Phase A - Intake, Sizing, and Surface Freeze

- Capture pre-refactor line count and symbol inventory.
- Capture test-name inventory for parity.
- Record that contract-first sequencing does not require amendments because
  the work is mechanical movement only.

### Phase B - Mechanical Extraction

- Create integration-test child modules by concern.
- Move shared fixture/helper code into `support.rs`.
- Move test blocks without assertion edits.

### Phase C - Validation and Evidence

- Run focused integration test:
  `cargo test --test clim06_frost_frozen_soil_kernel_contract`.
- Run the required Rust closure loop:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

### Phase D - Review, Verification, and Disposition

- Complete dual local review artifacts.
- Complete dual local verification artifacts.
- Complete final disposition and worker handoff.

## Exit Criteria

- No touched `.rs` file is at or above 2000 lines.
- All original test function names remain present in the split modules.
- Required cargo closure gates are run and recorded with exit codes.
- No contract or production implementation changes are made.
- No review finding remains undispositioned.

## Subagent Requirement

Subagent authorization: not requested by the user. The executing agent will
perform equivalent independent local reviews and verification passes and record
that path in the artifact files.

## Security Impact Gate

- security_impact: low
- dedicated_security_review_required: no
- rationale: integration-test organization only; no parser, subprocess,
  network, serialization, unsafe, or production runtime surface changes.
