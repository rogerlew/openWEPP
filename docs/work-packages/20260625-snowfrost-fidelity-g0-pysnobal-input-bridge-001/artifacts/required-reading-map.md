# Required Reading Map

Status: executed-hold

Evidence mode: Static + Ran.

| Path | Tier | Rationale | Applicability trigger | Read status |
| --- | --- | --- | --- | --- |
| `AGENTS.md` | Core | Root governance and package authorization rules. | Always before edits. | Static: read. |
| `docs/codex_exec_plans.md` | Core | ExecPlan expectations and review/disposition requirements. | Always before edits. | Static: read. |
| `docs/work-packages/AGENTS.md` | Core | Work-package execution, gates, review, and evidence rules. | Always before edits. | Static: read. |
| `docs/work-packages/README.md` | Core | Current package log and queue context. | Always before edits. | Static: read and updated. |
| `docs/work-packages/20260625-snowfrost-fidelity-g0-pysnobal-input-bridge-001/package.md` | Core | Package scope and gates. | Always before edits. | Static: read and updated. |
| `crates/AGENTS.md` | Conditional | Rust runner edits. | Before crate edits. | Static: read. |
| `tests/AGENTS.md` | Conditional | Integration-test edits. | Before test edits. | Static: read. |
| `docs/specifications/science-contracts/AGENTS.md` | Conditional | Contract authority and science-governance posture. | Before contract-derived tests or any SC edits. | Static: read. |
| `docs/specifications/unit-governance.md` | Conditional | Unit conversion, registry, and alias-risk governance. | Before exporter schema/conversion edits. | Static: read. |
| `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md` | On-demand | Snow/frost authority, WAT Snow-Depth/Snow-Water distinction, hourly aliases. | Before snow/frost lineage tests or any SC edits. | Static: read. |
| `docs/specifications/wepp-input-files/specs/climate-file.spec.md` | On-demand | `.cli` climate units and daily forcing fields. | Before climate input mapping edits. | Static: read. |
| `tools/snowfreeze_observed/README.md` | On-demand | Existing observed/fidelity harness conventions. | Before Python harness/doc edits. | Static: read and updated. |
| `/workdir/pysnobal/README.md` | On-demand | PySnobal user-facing input schema. | Before Python runner/schema finalization. | Static: read. |
| `/workdir/pysnobal/pysnobal/defaults.py` | On-demand | PySnobal exact column mapping. | Before Rust CSV schema finalization. | Static: read. |
| `/workdir/pysnobal/pysnobal/c_snobal/libsnobal/vars.c` | On-demand | `T_g`/`z_g` semantics. | Before ground-temperature lineage finalization. | Static: read. |
| `/workdir/pysnobal/pysnobal/c_snobal/libsnobal/g_soil.c` | On-demand | Ground heat flux role for `T_g`. | Before ground-temperature lineage finalization. | Static: read. |
