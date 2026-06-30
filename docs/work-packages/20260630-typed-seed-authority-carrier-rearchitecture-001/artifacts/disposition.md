# Disposition

Result:
`EXECUTED-HOLD-PHASE1-TYPED-PROJECTION-APIS-MISSING`.

The package was scaffolded and statically executed. It stops at Phase 1 before
implementation because the existing codebase has no parse-derived typed
projection API for the computed direct seed authority. The remaining production
direct seed path is:

1. typed parsed inputs to symbol-map static surfaces;
2. lane seed authority surface selection;
3. day-one climate surface merge;
4. `seed_wb11_runtime_surface_inputs` surface mutation;
5. surface reads into direct constructor, day-input authority, coupling
   metadata, and Wave-2/publication flags.

That is exactly the symbol-map authority this package is meant to remove.
Building a typed carrier by reading the day-zero surface would not be
single-authority and would make the shadow seed-identity gate self-referential.

First actionable follow-on:

1. Factor a typed projection core for static per-lane seed authority from
   `ParsedHillslopeRunInputs` and sidecars.
2. Factor typed day-one climate and `Wb11DayZeroProjection` outputs, moving the
   WB11/WB18/WB19/WB12/WB16/MOFE03 formulas out of surface-mutating helpers.
3. Rebuild `DirectLaneConstructorInputs`,
   `DirectProductionLaneDayInputAuthority`, coupling metadata, winter hourly
   geometry, and Wave-2 flags from that typed carrier.
4. Keep surface-writer adapters only for explicit compatibility replay and
   Phase 2 shadow comparison.
5. Resume this package at Phase 1, then run seed identity, output identity,
   symbol-map deletion, no-compatibility proof, and perf/RSS gates.

No production code was changed by this package. The older uncommitted Stage
1A/1B code and held packages remain separate worktree changes.
