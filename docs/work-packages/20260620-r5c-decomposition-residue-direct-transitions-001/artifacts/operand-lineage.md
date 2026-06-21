# Operand Lineage

Static: conservation-sensitive residue/root state is in scope as direct-frame
operands only. Public output authority remains compatibility-owned.

| Operand | Units | Source authority | R5C producer | Consumer/status |
|---|---:|---|---|---|
| `surface_residue_seed_kg_m2` | `kg m^-2` | `SC-RESIDUE-001` PL17 `sumsrm_seed` | direct decomposition input | authoritative inside R5C direct frame |
| `root_residue_seed_kg_m2` | `kg m^-2` | `SC-RESIDUE-001` PL17 `sumrtm_seed` | direct decomposition input | authoritative inside R5C direct frame |
| `temperature_factor` | fraction | `SC-RESIDUE-001` PL17 temperature modifier | direct decomposition compute | diagnostic/downstream direct operand |
| `surface_water_factor` | fraction | `SC-RESIDUE-001` PL17 standing-residue precipitation factor | direct decomposition compute | diagnostic/downstream direct operand |
| `flat_water_factor` | fraction | `SC-RESIDUE-001` PL17 `Ws` factor | direct decomposition compute | diagnostic/downstream direct operand |
| `environment_index` | fraction | `SC-RESIDUE-001` PL17 `envinx` | direct decomposition compute | diagnostic/downstream direct operand |
| `surface_decay_factor` | fraction | `SC-RESIDUE-001` PL17 `exp(-envinx * oratea)` | direct decomposition compute | diagnostic/downstream direct operand |
| `root_decay_factor` | fraction | `SC-RESIDUE-001` PL17 `exp(-envinx * orater)` | direct decomposition compute | diagnostic/downstream direct operand |
| `surface_residue_kg_m2` | `kg m^-2` | `SC-RESIDUE-001` PL17/event transfer | direct decomposition state | downstream residue partition input |
| `root_residue_kg_m2` | `kg m^-2` | `SC-RESIDUE-001` PL17/event transfer | direct decomposition state | downstream residue partition input |
| `standing_residue_kg_m2` | `kg m^-2` | `SC-RESIDUE-001` residue state surfaces | residue partition input | direct residue state |
| `flat_residue_kg_m2` | `kg m^-2` | `SC-RESIDUE-001` residue state surfaces | residue partition state | downstream hydrology/growth operand |
| `buried_residue_kg_m2` | `kg m^-2` | `SC-RESIDUE-001` residue state surfaces | residue partition input | direct residue state |
| `total_residue_kg_m2` | `kg m^-2` | mass sum over accepted direct residue pools | residue partition compute | diagnostic/downstream direct operand |
| `cover_fraction` | fraction | `SC-RESIDUE-001` cover boundary surfaces | residue partition input | downstream direct operand, not public output |

## Anti-Alias Obligations

Static:

- Decomposition tests must distinguish updated surface/root residue pools from
  raw seeds, precipitation, storage bounds, and event fractions.
- Residue partition tests must distinguish flat residue from standing, buried,
  root, total residue, direct storage, runoff publication, and cover fraction.
- Exact self-consistency is insufficient; fixture values must make plausible
  aliases numerically different.
