# Line-Count Governance

Status: complete
Evidence mode: Static

Line-count scan:

| File | Lines | Disposition |
|---|---:|---|
| `crates/openwepp-hillslope-orchestrator/src/hydrology/09_snow_density.rs` | 524 | Existing focused snow-density module; retained to keep density model logic local. |
| `crates/openwepp-hillslope-orchestrator/src/lib.rs` | 202 | Re-export file; no split needed. |
| `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs` | 2608 | Existing large direct-production builder; package touched only selector mapping. |
| `tests/integration/snowdensity10_3_17_shallow_pack_compaction_guard.rs` | 219 | Focused integration test; acceptable. |

Non-Rust large-file note:

- `tools/snowfreeze_observed/shallow_pack_compaction_guard.py`: 693 lines,
  diagnostic-only harness following existing snowfreeze tool patterns.
- `SC-SNOWFREEZE-001.md`, `docs/planning/snow-frost-fidelity-strategy.md`, and
  `docs/work-packages/README.md` are canonical long-lived documents; this
  package made scoped amendments only.
