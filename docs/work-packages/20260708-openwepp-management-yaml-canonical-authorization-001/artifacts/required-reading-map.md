# Required Reading Map

Status: scaffolded.

## Core

| Path | Purpose |
|---|---|
| `AGENTS.md` | repository governance |
| `docs/work-packages/AGENTS.md` | work-package governance |
| `docs/standards/AGENTS.md` | standards governance |
| `docs/standards/prompt-wording-guidance.md` | execution prompt wording |
| `docs/specifications/science-contracts/AGENTS.md` | contract-first sequencing |
| `crates/AGENTS.md` | Rust implementation rules |
| `tests/AGENTS.md` | integration-test rules |
| `docs/specifications/wepp-input-files/specs/management-yaml.spec.md` | draft YAML target spec |
| `docs/specifications/wepp-input-files/specs/plant-file.spec.md` | flat management datver context |
| `docs/specifications/wepp-input-files/input-surface-registry.md` | input-surface registry |
| `docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md` | current flat management parser contract |
| `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` | Lane D route-coefficient authority |
| `docs/contracts/openwepp-management-lanuse-authority-contract.md` | native landuse authority |
| `artifacts/crate-ownership-assessment.md` | recommended long-term schema crate ownership |

## Conditional

| Path | Trigger |
|---|---|
| `docs/specifications/science-contract-authoring-procedure.md` | before creating `SC-INFILE-MANAGEMENT-YAML-001` |
| `docs/specifications/science-contracts/kernel-process-contract-profile.md` | before kernel-facing contract edits |
| `crates/openwepp-input-contract/src/parsers/management.rs` | source-model mapping |
| `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/01_management.rs` | runtime consumer edits |
| `tests/integration/infile_management_parser_contract.rs` | parser/schema integration tests |

## Budget

Scaffold reading uses targeted excerpts. Full implementation should record byte
counts before broad contract or source reads.
