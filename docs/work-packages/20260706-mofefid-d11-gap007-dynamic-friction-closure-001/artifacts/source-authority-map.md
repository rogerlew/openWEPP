# Source Authority Map

Status: **PASS**.

## Operand Map

| Operand | Source candidate | Decision | Evidence |
| --- | --- | --- | --- |
| `I` | WB14 hourly rainfall depth profile on the direct day frame, consumed as `depth_m / 3600 s` | source-authorized for Lane D shadow | Static: `DirectDayFrame.wb14_hourly_rainfall_m` is produced at R4K from the runtime hyetograph/liquid-input path and already feeds hourly erosion rainfall logic. |
| `LAI` | Post-growth `DirectDayFrame.evapotranspiration_compute_inputs.leaf_area_index` | source-authorized for Lane D shadow | Static: growth downstream updates ET inputs when active; inactive periods retain the lane plant state. The shadow reads the executed day frame, not the pre-growth input object. |
| `h_c` | Parsed typed-management `canhgt` retained on lane authority | source-authorized for Lane D shadow | Static: `canhgt` is projected from typed management. Lane D requires it to be finite and positive when post-growth `LAI > 0`; absent canopy (`LAI = 0`) may route with zero vegetation resistance. |

## Rejected Aliases

Static:

- `runoff.runvol_m3 / area_m2` is rainfall-excess/source depth, not skin
  rainfall intensity `I`.
- `dc01_surface_hourly_weights` is a shape vector, not rainfall intensity.
- Cover fractions, residue depth, ridge/random roughness, and Chapter-10
  hydraulics terms are not implicit aliases for `h_c` or D-val friction
  constants.
- All-lane `I = 0`, `LAI = 0`, or `h_c = 0` placeholders are not acceptable
  when source-authorized dynamic operands are present.

## Disposition

Static: `SC-OFEROUTE-001` rev 21 ratifies these three sources for the opt-in
Lane D shadow. Future production/default activation remains out of scope and
must prove its own consumer path.
