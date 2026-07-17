# Required Reading Map

Status: active intake record

## Applicable Instructions

`tools/agents/find-agents --for` was run over the declared write set before
substantive edits.

| Surface | Instruction chain |
| --- | --- |
| Standard and standards catalog | `AGENTS.md`; `docs/standards/AGENTS.md` |
| ADR and decision catalog | `AGENTS.md` |
| Package and package catalog | `AGENTS.md`; `docs/work-packages/AGENTS.md` |

The docs-maintainer skill is active. Its Markdown lint, reference, catalog, and
American-English workflow applies where the openWEPP repository provides the
corresponding tools.

## Core Reading

| Path | Purpose |
| --- | --- |
| `AGENTS.md` | Root validation and repository invariants |
| `docs/standards/AGENTS.md` | Canonical-standard authoring rules |
| `docs/work-packages/AGENTS.md` | Package, review, verification, and evidence rules |
| `docs/codex_exec_plans.md` | Living execution-plan and review requirements |
| `docs/standards/local-ci-gate-selection.md` | Current proportional gate intent |
| `docs/decisions/0021-module-coverage-closure-thresholds.md` | Binding coverage/CRAP quality and current frequency |
| `docs/specifications/correctness-authority-model.md` | Correctness authorities and external suite lanes |
| `docs/standards/rust-scientific-coding-standard.md` §7 | Test-family and obligation authoring authority |

Core local reading is approximately 106,000 bytes, within the `OK` budget of
400,000 bytes. The package-local files are additional small execution records.

## Conditional And On-Demand Reading

| Path | Trigger |
| --- | --- |
| `.config/nextest.toml` | Current executor profiles and scheduling claims |
| `.github/workflows/release-gates.yml` | CI trigger and lane mapping |
| `tools/release/run_release_candidate_gates.sh` | Current release/validation command composition |
| `tools/release/run_adjudicated_crap_gate.sh` | Current full-workspace coverage acquisition |
| `tests/AGENTS.md` | Test-tree local workflow |
| `docs/governance/scientific-assurance-v2-architecture.md` | Assurance dependency/build boundary |
| `docs/governance/scientific-assurance-dossier-lifecycle.md` | Assurance impact and currency semantics |
| `crates/openwepp-assurance/src/v2/planner.rs` | Existing mechanical current/stale/blocked/selected planner |

External sources are recorded with their exact use in `research-basis.md`.
