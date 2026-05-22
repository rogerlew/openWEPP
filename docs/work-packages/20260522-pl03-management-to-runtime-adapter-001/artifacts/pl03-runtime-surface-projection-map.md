# PL03 Runtime Surface Projection Map

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- PL03 must project strict scheduler-facing PL runtime surfaces from typed parser outputs.

Ran:
- Implemented deterministic projection into three PL surface families with stable symbol naming.

## Surface Families

| runtime surface | symbol family | source fields |
|---|---|---|
| `pl_schedule_surface` | `pl_schedule_*` plus canonical seed aliases | topology count, schedule closure metadata, slot topology/year position, yearly references, control branch selectors |
| `pl_growth_surface` | `pl_growth_*` plus canonical seed aliases | `imngmt`, `itype`, planting/harvest controls, row width, perennial controls |
| `pl_decomp_surface` | `pl_decomp_*` plus canonical seed aliases | residue management controls, residue/root seed state, perennial cut/grazing cardinalities |

## Naming Templates

1. OFE schedule seed:
- `pl_schedule_ofe{ofe_index}_{root}`

2. Slot schedule:
- `pl_schedule_slot_{slot:04}_{root}`
- `pl_schedule_slot_{slot:04}_crop_{crop:04}_{root}`

3. Growth/decomposition:
- `pl_growth_ofe{ofe_index}_{root}`
- `pl_growth_slot_{slot:04}_crop_{crop:04}_{root}`
- `pl_decomp_ofe{ofe_index}_{root}`
- `pl_decomp_slot_{slot:04}_crop_{crop:04}_{root}`

## Required Canonical-Like Seed Aliases Projected

- `lanuse`
- `itype`
- `imngmt`
- `jdharv`
- `jdplt`
- `rw`
- `resmgt`
- `iresd_seed`
- `sumrtm_seed`
- `sumsrm_seed`

## Ordering Preconditions Projected

- `pl_order_decomp_before_soil = 1`
- `pl_order_growth_after_decomp = 1`
- `pl_order_watbal_after_growth = 1`

## Evidence Links

- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:702`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:764`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:768`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:772`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:1868`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:1872`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:1876`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:1886`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:1900`
