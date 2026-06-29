# Line-Count Governance

Evidence class: Ran.

Command:

```bash
wc -l \
  crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs \
  crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs \
  crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs \
  docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md \
  tests/integration/paradigm2_stage3_liquid_routing_meltwater_temperature.rs
```

Result:

```text
  2901 crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs
  1564 crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs
   821 crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs
  3083 docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md
   215 tests/integration/paradigm2_stage3_liquid_routing_meltwater_temperature.rs
```

Disposition:

- The runner builder remains below `3000` lines after adding the Stage 3 selector
  parser.
- The hydrology implementation files remain below `2000` lines.
- `SC-SNOWFREEZE-001` is a long canonical science contract; this package added
  v110 Stage 3 authority to the existing contract rather than splitting contract
  authority into a non-canonical side file.
