# refactor014-kernel-line-count-governance-checklist

Status: complete
Evidence mode: Ran

## Static:
- Baseline file:
  - `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/kernel_core.rs`: 5638
    lines in reference backup (`/tmp/kernel_core.rs.bak`).
- Post-refactor governance objective:
  - reduce any single file above 3000-line threshold; preserve behavior.

## Ran:
- `constants.rs`: 74
- `types.rs`: 180
- `helpers.rs`: 1399
- `routing.rs`: 1934
- `diagnostics.rs`: 1098
- `validation.rs`: 930
- `kernel_core.rs`: 36
- `kernel.rs`: 7

## Disposition:
- All files are below 2000 lines. `routing.rs` (1934) is below 3000 and no
  3000+ exception is required.
