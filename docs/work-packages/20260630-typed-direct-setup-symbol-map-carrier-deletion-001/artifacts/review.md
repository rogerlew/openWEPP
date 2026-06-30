# Review

Evidence mode: Static/Ran.

## Findings

1. `HOLD`: Stage 1 is not complete. Production direct no longer constructs the
   symbol registry/hot tables, but it still constructs and reads
   `HillslopeWritebackSurface` seed authorities in
   `execute_hillslope_direct_production_days`, `build_direct_production_run_frame`,
   `seed_direct_production_lane_constructor_inputs`, and
   `DirectProductionDayInputBuilder::new`.

2. The Stage 1A code change is identity-preserving for the measured endpoint:
   H2637 HBP/loss/plot/WAT/PASS are byte-identical against the clean
   `5b139058` baseline, direct manifests still report
   `compatibility_edge_invocations=0`, and the multi-OFE/Wave-2 focused test
   passes.

3. The next package must add a typed direct seed authority before more deletion:
   per-lane soil/layer, slope/topology, management/growth/residue/PMET,
   snow/frost, erosion, and coupling/publication seed values must be derived
   from parsed input structs rather than from runtime-surface symbols.

## Disposition

Accepted. Close this package HOLD after recording the blocker and do not start
Stage 2 deletion.
