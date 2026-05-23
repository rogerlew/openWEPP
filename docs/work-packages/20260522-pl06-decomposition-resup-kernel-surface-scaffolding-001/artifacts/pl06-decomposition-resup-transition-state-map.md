# PL06 Decomposition/Resup Transition State Map

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- Decomposition/residue placeholder transitions enforce required symbol surfaces and typed ordering guards.
- Ordering flags are required runtime contracts and must equal `1.0`.

Ran:
- Decomposition and growth boundary failure tests executed and passed.

## Transition Inputs

### Common Decomposition Ordering Inputs

- `pl_order_decomp_before_soil = 1.0`
- `pl_order_growth_after_decomp = 1.0`

### Common Decomposition Seed Inputs

- `iresd_seed`
- `sumrtm_seed`
- `sumsrm_seed`

### Decomposition/Residue (Annual or Fallow) Required Inputs

- `pl_growth_slot_0001_crop_0001_imngmt` in `{1, 3}`
- `pl_decomp_slot_0001_crop_0001_resmgt`

Output request metadata:
- `phase_class in {DecompositionTransition, ResiduePartitionTransition}`
- `decomposition_context.management_class = AnnualOrFallow`

### Decomposition/Residue (Perennial) Required Inputs

- `pl_growth_slot_0001_crop_0001_imngmt = 2`
- `pl_decomp_slot_0001_crop_0001_mgtopt`
- `pl_decomp_slot_0001_crop_0001_ncut`
- `pl_decomp_slot_0001_crop_0001_ncycle`

Output request metadata:
- `phase_class in {DecompositionTransition, ResiduePartitionTransition}`
- `decomposition_context.management_class = Perennial`

## Transition Failure Surface

- `HS-DECOMP-E-001` missing required symbol
- `HS-DECOMP-E-002` non-finite required value
- `HS-DECOMP-E-003` ordering flag mismatch
- `HS-DECOMP-E-004` unsupported management class domain

## Evidence Links

- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs:33`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs:546`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs:559`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs:567`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs:2153`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs:2203`
