Scope: local repository Rust/docs implementation task; flat-file reads/edits
only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in
`docs/work-packages/20260708-openwepp-management-yaml-canonical-authorization-001/package.md`
sequentially through disposition.

Required reading:

Core:
- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/standards/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`
- `docs/specifications/wepp-input-files/specs/management-yaml.spec.md`
- `docs/specifications/wepp-input-files/specs/plant-file.spec.md`
- `docs/specifications/wepp-input-files/input-surface-registry.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/contracts/openwepp-management-lanuse-authority-contract.md`
- `docs/work-packages/20260708-openwepp-management-yaml-canonical-authorization-001/artifacts/required-reading-map.md`

Conditional:
- `docs/specifications/science-contract-authoring-procedure.md` before creating `SC-INFILE-MANAGEMENT-YAML-001`.
- `docs/specifications/science-contracts/kernel-process-contract-profile.md` before kernel-facing contract edits.
- `crates/openwepp-input-contract/src/parsers/management.rs` before source-model mapping.
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/01_management.rs` before runtime consumer edits.
- `tests/integration/infile_management_parser_contract.rs` before parser/schema integration tests.

Files:
- `docs/specifications/wepp-input-files/specs/management-yaml.spec.md`
- `docs/work-packages/20260708-openwepp-management-yaml-canonical-authorization-001/**`
- authority and implementation files declared by `package.md`.

Task: authorize and implement canonical producer-side openWEPP management YAML
with typed schemas for `ow-lanuse-1+`, then prove the real openWEPP runtime
consumer reads YAML and projects route coefficients into PL schedule surfaces.

Constraints:
- contract/spec-first sequencing;
- YAML is a primary input surface, not a sidecar;
- no native flat `.man` writer;
- no coefficient projection from legacy fields;
- producers emit lowercase `.yaml` only and default migrated flat `.man` sources
  to informal `.man.yaml` naming;
- consumers accept `.yaml`, `.YAML`, `.yml`, and `.YML` input paths and still
  validate document content;
- typed errors and fail-closed behavior;
- real consumer proof before production-readiness closure.

Subagent requirement: REQUIRED for implementation review/verification when
available. This prompt explicitly authorizes subagent spawning/delegation to
read-only review and verification subagents for contract, schema, runtime
consumer, and crates.io/schema-readiness review. Outputs:
`artifacts/review-*.md` and `artifacts/verification-*.md`. Write access:
read-only unless a later prompt assigns a bounded implementation write set.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.
