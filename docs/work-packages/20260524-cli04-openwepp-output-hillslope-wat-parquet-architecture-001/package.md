# 20260524-cli04-openwepp-output-hillslope-wat-parquet-architecture-001

## Status
- state: completed
- date: 2026-05-24
- timezone: UTC

## Objective
Define and implement the shared openWEPP output architecture for hillslope and
watershed parquet surfaces, and land real hillslope `wat` parquet emission in
`openwepp-cli-hill` with schema/metadata parity to WEPPpy/WEPPpyo3 interchange
authority.

## Why This Package Exists
CLI03 established `.run` output keys and output-crate boundary, but current
optional output emission in `openwepp-cli-hill` is placeholder text rather than
real parquet. Downstream WEPPpy contracts consume metadata-rich parquet schemas,
including post-`wepp_260430` water-balance additions (notably
`InterceptionStorage`) used for closure/audit workflows.

This package resolves two architecture gaps:
1. shared crate boundary decision (`openwepp-output` vs
   `openwepp-hillslope-output`) for hillslope and watershed output families,
2. concrete hillslope `wat` parquet writer implementation and CLI wiring.

Parquet dependency posture for this package is explicit:
- `arrow2` is deprecated upstream and must not be introduced for new
  implementation in this package.
- Writer implementation must use the maintained `arrow-rs` ecosystem with typed
  errors and contract-tested metadata behavior.
- CLI04 standard stack:
  - `parquet` crate for parquet serialization (`ArrowWriter` path),
  - `arrow-array` for Arrow array / record-batch surfaces,
  - `arrow-schema` for schema + field metadata (`units`, `description`,
    dataset-version metadata keys).
- `arrow-schema` is a companion crate within `arrow-rs`, not an alternative to
  `arrow-rs`.

This package is runtime/output-contract affecting and follows contract-first
sequencing:
1. implement required contract/spec amendments,
2. implement contract-derived tests,
3. record pre-implementation contract gate evidence, then
4. modify production code.

## Scope
### Included
- Decide and ratify shared output crate boundary for hillslope + watershed
  output families.
- Record and enforce dependency policy for parquet implementation (`arrow2`
  prohibited for new implementation; required stack is `arrow-rs` ecosystem
  crates `parquet` + `arrow-array` + `arrow-schema`).
- Amend runner/output contracts/specs to encode shared boundary ownership,
  parquet metadata requirements, and output-family routing expectations.
- Implement contract-derived tests for hillslope `wat` parquet validity,
  schema metadata keys, and field-level units/description parity requirements.
- Record pre-implementation contract gate evidence before production edits.
- Implement hillslope `wat` parquet writer and wire `openwepp-cli-hill`
  `outputs.wat` path to emit real parquet.
- Preserve required pass/loss behavior and manifest checksum behavior.
- Record authoritative parity matrix between openWEPP emitted `wat` parquet and
  WEPPpy/WEPPpyo3 interchange schema expectations.

### Explicitly Out of Scope
- Full watershed parquet implementation for all families.
- New water-balance physics model changes.
- Legacy WEPP orchestration changes in `wepppy/wepp_runner`.
- Legacy dependency migrations outside CLI04 write set.

## Deliverables
1. CLI04 contract implementation evidence:
   - `artifacts/cli04-contract-implementation-evidence.md`
2. Output architecture authority/guard map:
   - `artifacts/cli04-output-architecture-authority-and-guard-map.md`
3. CLI04 contract-derived test implementation evidence:
   - `artifacts/cli04-contract-test-implementation-evidence.md`
4. CLI04 pre-implementation contract gate:
   - `artifacts/cli04-preimplementation-contract-gate.md`
5. CLI04 implementation and test evidence:
   - `artifacts/cli04-implementation-and-test-evidence.md`
6. Hillslope WAT schema parity matrix:
   - `artifacts/cli04-hillslope-wat-schema-parity-matrix.md`
7. Shared output crate rename/boundary plan:
   - `artifacts/cli04-crate-rename-and-shared-boundary-plan.md`
8. Kernel-profile/runtime-contract compliance checklist:
   - `artifacts/cli04-kernel-profile-compliance-checklist.md`
9. WEPPpy consumer boundary note:
   - `artifacts/cli04-wepppy-consumer-boundary-note.md`
10. Package governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/cli04_disposition.md`
11. Dual review artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
12. Dual verification artifacts:
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Implement canonical contract/spec authority amendments.
2. Implement contract-derived tests.
3. Record pre-implementation contract-gate evidence.
4. Only then modify production runner/output code.

Any sequencing violation keeps package disposition in `HOLD`.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label evidence mode using `Static:` and/
or `Ran:` sections. Claims without explicit evidence labeling are
non-compliant.

## Provenance and Authority Exception (Explicit)
Default legacy comparator baseline remains
`/workdir/wepp-forest_260430_baseline` at
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.

For this package's WAT/output schema authority, an explicit exception applies:
post-`wepp_260430` `wepp-forest` output-contract lineage is authoritative for
WAT export semantics required for closure, including `InterceptionStorage`.
This exception is justified by stakeholder and downstream consumer authority
artifacts listed in Dependencies.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/decisions/0005-parquet-via-wepppyo3-interchange.md`
- `/workdir/openWEPP/docs/contracts/openwepp-hillslope-runfile-contract.md`
- `/workdir/openWEPP/docs/contracts/openwepp-runner-contract.md`
- `/workdir/openWEPP/docs/specifications/subsystems/runner/openwepp-hillslope-cli-specification.md`
- `/workdir/openWEPP/docs/contracts/README.md`
- `/workdir/openWEPP/docs/work-packages/20260524-cli03-hillslope-runner-interchange-implementation-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260522-arch19-run-and-parquet-boundary-contracts-001/artifacts/parquet-boundary-contract-authority.md`
- `/workdir/openWEPP/docs/work-packages/20260522-arch19-run-and-parquet-boundary-contracts-001/artifacts/wepppyo3-parquet-schema-reference-inventory.md`
- `/workdir/openWEPP/crates/openwepp-hillslope-output/`
- `/workdir/openWEPP/crates/openwepp-runner/`
- `/workdir/openWEPP/tests/integration/`
- `/workdir/wepp-forest_260430_baseline` @ `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- `/home/workdir/wepppy/wepppy/wepp/interchange/hill_wat_interchange.py`
- `/home/workdir/wepppy/wepppy/wepp/interchange/versioning.py`
- `/home/workdir/wepppy/wepppy/weppcloud/routes/usersum/vendor/wepp-forest/docs/20260504-stakeholder-watbalance.md`

## Intended Write Set
- `Cargo.toml` (workspace crate membership/dependency rewiring if needed)
- `crates/openwepp-hillslope-output/**` (or successor shared output crate path)
- `crates/openwepp-runner/Cargo.toml`
- `crates/openwepp-runner/src/lib.rs`
- `tests/integration/**`
- `docs/contracts/openwepp-runner-contract.md`
- `docs/contracts/openwepp-hillslope-runfile-contract.md`
- `docs/specifications/subsystems/runner/openwepp-hillslope-cli-specification.md`
- `docs/contracts/README.md`
- `docs/work-packages/20260524-cli04-openwepp-output-hillslope-wat-parquet-architecture-001/**`
- `docs/work-packages/README.md`

## Phase Plan
### Phase 0 - Intake
- Confirm queue authority, CLI03 closure baseline, and cross-repo schema
  authority references.

### Phase A - Contract/Architecture Authority
- Amend runner/output contract surfaces for shared output boundary and WAT
  metadata parity requirements.
- Record crate-boundary decision (`openwepp-output` adoption plan).
- Encode parquet dependency constraint (`arrow2` deprecation and prohibited use
  for new implementation) and codify `arrow-rs` stack roles (`parquet`,
  `arrow-array`, `arrow-schema`).

### Phase B - Contract Tests + Pre-Implementation Gate
- Implement contract-derived tests for WAT parquet schema/metadata parity.
- Record pre-implementation contract gate evidence.

### Phase C - Production Implementation
- Implement real `outputs.wat` parquet emission and CLI wiring under typed
  output boundaries using `parquet::arrow::ArrowWriter` with schema/field
  metadata parity to WEPPpy interchange authority.

### Phase D - Verification
- Run targeted integration tests and required repository gates.

### Phase E - Disposition
- Publish evidence set, dual review/verification, and disposition decision.

## Exit Criteria
- Shared output crate boundary decision is ratified and evidence-backed.
- Selected parquet implementation stack is documented as `arrow-rs` ecosystem,
  with `arrow-schema` explicitly treated as companion crate (not alternative).
- Contract/spec updates explicitly encode WAT metadata parity requirements.
- Contract-derived tests cover file validity + metadata key presence + field
  units/descriptions parity for `wat` output.
- Pre-implementation gate is recorded before production edits.
- `openwepp-cli-hill` `outputs.wat` emits real parquet, not placeholder text.
- Optional `InterceptionStorage` handling is encoded as producer-authoritative
  optional term for closure workflows.
- Required repository gates executed if code is changed:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

## Security Impact and Review Gate
- security_impact: medium
- dedicated_security_review_required: yes
- Rationale: runtime output serialization and downstream analytics contract
  surfaces are modified.
