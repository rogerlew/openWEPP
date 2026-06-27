# Line-Count Governance Checklist

Status: complete
Evidence mode: Ran

Ran:

```text
wc -l crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs crates/openwepp-runner/src/hillslope/snowbench_jennings_phase.rs crates/openwepp-runner/src/bin/openwepp-snowbench.rs tests/integration/snowdensity10_3_5b_hourly_partition_jennings_contract.rs docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md docs/work-packages/20260627-snowdensity-10-3-5b-hourly-partition-jennings-validation-001/package.md
```

Results:

| File | Lines | Disposition |
|---|---:|---|
| `06_simimpl28_hourly_forcing.rs` | 1394 | Existing large runtime forcing file; package added localized selector and partition diagnostics only. |
| `00_builders_and_authority.rs` | 3098 | Existing large direct-publication authority file; package added localized env selector and field wiring only. |
| `snowbench_jennings_phase.rs` | 525 | New diagnostic module; acceptable for CSV parsing, scoring, threshold summary, and report writing. |
| `openwepp-snowbench.rs` | 259 | Existing diagnostic CLI; added `jennings-phase` subcommand. |
| `snowdensity10_3_5b_hourly_partition_jennings_contract.rs` | 115 | Focused contract/tool smoke test. |
| `SC-SNOWFREEZE-001.md` | 2064 | Existing canonical contract; v92 amendment only. |
| `package.md` | 202 | Work-package execution spec. |

Disposition: no new broad refactor was introduced to reduce existing file size.
The new diagnostic module is cohesive and package-scoped.
