# Required Reading Map

Status: queued

Evidence mode: not-run.

| Path | Tier | Rationale | Applicability trigger | Read status |
| --- | --- | --- | --- | --- |
| `AGENTS.md` | Core | Root governance and package authorization rules. | Always before edits. | queued |
| `docs/codex_exec_plans.md` | Core | ExecPlan expectations and review/disposition requirements. | Always before edits. | queued |
| `docs/work-packages/AGENTS.md` | Core | Work-package execution, gates, review, and evidence rules. | Always before edits. | queued |
| `docs/work-packages/README.md` | Core | Current package log and queue context. | Always before edits. | queued |
| `docs/work-packages/20260625-snowfrost-fidelity-g0-pysnobal-input-bridge-001/package.md` | Core | Package scope and gates. | Always before edits. | queued |
| `crates/AGENTS.md` | Conditional | Rust runner edits. | Before crate edits. | queued |
| `tests/AGENTS.md` | Conditional | Integration-test edits. | Before test edits. | queued |
| `docs/specifications/science-contracts/AGENTS.md` | Conditional | Contract authority and science-governance posture. | Before contract-derived tests or any SC edits. | queued |
| `docs/specifications/unit-governance.md` | Conditional | Unit conversion, registry, and alias-risk governance. | Before exporter schema/conversion edits. | queued |
| `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md` | On-demand | Snow/frost authority, WAT Snow-Depth/Snow-Water distinction, hourly aliases. | Before snow/frost lineage tests or any SC edits. | queued |
| `docs/specifications/wepp-input-files/specs/climate-file.spec.md` | On-demand | `.cli` climate units and daily forcing fields. | Before climate input mapping edits. | queued |
| `tools/snowfreeze_observed/README.md` | On-demand | Existing observed/fidelity harness conventions. | Before Python harness/doc edits. | queued |
| `/workdir/pysnobal/README.md` | On-demand | PySnobal user-facing input schema. | Before Python runner/schema finalization. | queued |
| `/workdir/pysnobal/pysnobal/defaults.py` | On-demand | PySnobal exact column mapping. | Before Rust CSV schema finalization. | queued |
| `/workdir/pysnobal/pysnobal/c_snobal/libsnobal/vars.c` | On-demand | `T_g`/`z_g` semantics. | Before ground-temperature lineage finalization. | queued |
| `/workdir/pysnobal/pysnobal/c_snobal/libsnobal/g_soil.c` | On-demand | Ground heat flux role for `T_g`. | Before ground-temperature lineage finalization. | queued |
