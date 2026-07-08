# Required Reading Map

Status: scaffolded and amended for YAML-only output.

## Core

| Path | Purpose |
|---|---|
| `AGENTS.md` | repository governance |
| `docs/work-packages/AGENTS.md` | work-package governance |
| `docs/standards/AGENTS.md` | standards governance |
| `docs/standards/prompt-wording-guidance.md` | execution prompt wording |
| `crates/AGENTS.md` | Rust crate implementation rules |
| `docs/specifications/wepp-input-files/specs/plant-file.spec.md` | legacy/native flat management datver context |
| `docs/specifications/wepp-input-files/specs/landuse-migration-cli.spec.md` | CLI specification drafted by this package |
| `docs/specifications/wepp-input-files/specs/management-yaml.spec.md` | canonical YAML target specification |
| `docs/contracts/openwepp-management-lanuse-authority-contract.md` | native landuse authority and explicit producer model |
| `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` | Lane D coefficient authority and native datver policy |
| `docs/work-packages/20260708-openwepp-management-yaml-canonical-authorization-001/package.md` | YAML authorization dependency |
| `docs/work-packages/20260708-laned-router-ow-lanuse-canonical-production-datver-authority-001/package.md` | M-T2Q canonical datver authority |
| `docs/work-packages/20260708-laned-router-canonical-hourly-laned-routing-coeff-projection-authority-001/artifacts/ow-lanuse-canonical-consensus-addendum.md` | no-sidecar consensus |

## Conditional

| Path | Trigger |
|---|---|
| `docs/specifications/science-contracts/AGENTS.md` | if contract edits occur |
| `tests/AGENTS.md` | before integration tests |
| `crates/openwepp-input-contract/src/parsers/management.rs` | frozen flat source parser integration |
| `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/01_management.rs` | runtime YAML consumer proof |
| `/home/workdir/wepppy/wepppy/nodb/mods/disturbed/route_coefficients.py` | Disturbed table embedding |
| `/home/workdir/wepppy/docs/adrs/ADR-0014-disturbed-openwepp-route-coefficients.md` | Disturbed coefficient provenance |

## Budget

Scaffold reading uses targeted excerpts. Full implementation should record byte
counts before broad contract or source reads.
