# Disposition

Result: `EXECUTED-HOLD-STAGE1-TYPED-SEED-AUTHORITY-MISSING`

This package cannot proceed to symbol-map carrier deletion. Stage 1A removed an
unnecessary direct-production setup object: `SymbolRegistry` / `HotSymbolTables`
and indexed/lane-dense writeback authority are now constructed only for
compatibility execution, not for production direct. The snowbench export setup
also uses the direct-production setup posture for its static context.

The primary Stage 1 gate remains unmet. Production direct still constructs and
reads `HillslopeWritebackSurface` seed authorities for:

- lane seed surfaces from `runtime_surface` / persistent lane writeback state;
- day-zero lane constructor seeding;
- `DirectProductionDayInputBuilder` per-lane authority construction;
- direct coupling/publication metadata still read from `execution_runtime_surface`.

The first actionable follow-on is a typed direct seed-authority package:
introduce parsed-input-derived per-lane authority structs for soil/layers,
slope/topology, management/growth/residue/PMET, snow/frost, erosion, and
coupling/publication metadata. Once production direct can construct
`DirectRunFrame` and `DirectProductionDayInputBuilder` from those typed
authorities without `HillslopeWritebackSurface`, resume carrier deletion and the
Stage 3 no-compatibility proof.

Evidence supporting HOLD:

- Static inventory found `208` remaining direct-publication seed reads from
  runtime-surface symbols and `266` runtime-input symbol insertions feeding the
  setup bridge.
- H2637 output identity is preserved for the Stage 1A increment.
- RSS improved: clean `5b139058` baseline `110916 KiB`; current `91796 KiB`.
- Multi-OFE/Wave-2 focused gate still passes.
