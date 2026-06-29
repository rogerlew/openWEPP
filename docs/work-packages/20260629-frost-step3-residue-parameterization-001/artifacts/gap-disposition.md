# GAP-SNOWFREEZE-002 Disposition Input

Evidence mode: Ran.

`GAP-SNOWFREEZE-002` remains open.

The Step 2 Sleepers timing candidate defects are not cleared by this package.
However, the residue-lifecycle hypothesis cannot be tested by simply repointing
the Sleepers fixtures to the existing `Dec_*` cropland-management files:

- The entry-gate `Dec_4899` run reached the frost solver.
- The solver-side `residue_depth_m` was flat for all `32874` trace rows.
- Therefore, the existing cropland-management `Dec_*` fixture does not provide
  the required seasonal forest litter/residue insulation trajectory.

Package routing: decision branch `C`.

Follow-on: promote the "Surface residue / litter cover" dimension of
`docs/backlog/20260626-frost-daylength-canopy-decline-hemisphere-robust.md` into
a work package. The first actionable goal is to implement or expose a first-class
forest litter/residue cover path that can produce recurring autumn leaf-drop and
winter/spring litter decay at `frost.runtime_residue_depth_m`, then rerun the
Sleepers Step 3 timing attribution.

The Step 4 carry-forward note remains active: the Step 1 `>0.25`
systematic-timing-fraction cutoff is diagnostic-script-local and must be
adjudicated deliberately if `INV-SNOWFREEZE-048/050` ratification inherits it.
