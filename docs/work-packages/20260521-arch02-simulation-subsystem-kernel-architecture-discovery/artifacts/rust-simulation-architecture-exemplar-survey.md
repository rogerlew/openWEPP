# rust simulation architecture exemplar survey

Evidence: Static (web documentation)
Ran evidence: none

## Exemplar observations

### Bevy ECS scheduling model
- Static: [DIRECT] Bevy documents that systems run in parallel/non-deterministic order by default and require explicit ordering constraints to remove ambiguity.
- Source: https://docs.rs/bevy/latest/bevy/ecs/system/index.html
- Static: [DIRECT] `bevy_ecs::schedule` exposes topological ordering metadata and conflict-aware multithreaded execution constructs.
- Source: https://docs.rs/bevy_ecs/latest/bevy_ecs/schedule/index.html
- Static: [INFERENCE] openWEPP should model subsystem execution as an explicit dependency graph with deterministic topological ordering and declared conflicts.

### Rapier pipeline orchestration
- Static: [DIRECT] Rapier's `PhysicsPipeline` centers each timestep around a single `step` call with explicit subsystem inputs (bodies/colliders/joints/phases) and a staged internal pipeline.
- Source: https://docs.rs/rapier3d/latest/rapier3d/pipeline/struct.PhysicsPipeline.html
- Static: [INFERENCE] openWEPP should use one authoritative timestep orchestrator per domain (hillslope daily/hourly, watershed routing) with explicit typed input bundles.

### uom dimensional analysis
- Static: [DIRECT] `uom` provides automatic type-safe zero-cost dimensional analysis and treats units at boundaries while preserving quantity semantics in core code.
- Source: https://docs.rs/uom/latest/uom/
- Static: [INFERENCE] openWEPP should enforce unit-safe newtypes around kernel boundaries where mixed depth/volume/rate units currently cause ambiguity risk.

### petgraph dependency modeling
- Static: [DIRECT] `petgraph` describes graph types/algorithms for directed or undirected node-edge models with arbitrary attached data.
- Source: https://docs.rs/petgraph/latest/petgraph/
- Static: [INFERENCE] openWEPP can use DAG/topology graph modeling for watershed dependency validation and scheduler planning.

### Rayon controlled parallelism
- Static: [DIRECT] Rayon documentation highlights `join`, `scope`, and `ThreadPoolBuilder` for controlled task decomposition and custom thread-pool configuration.
- Source: https://docs.rs/rayon/latest/rayon/
- Static: [INFERENCE] openWEPP can parallelize independent hillslope or channel slices only when deterministic reduction order is explicitly defined.

### nalgebra typed linear algebra
- Static: [DIRECT] nalgebra supports compile-time and runtime dimensions in one matrix abstraction, with compile-time-sized structures statically allocated.
- Source: https://docs.rs/nalgebra/latest/nalgebra/
- Static: [INFERENCE] openWEPP can use compile-time dimensioned structures for fixed small-kernel vectors/matrices while keeping runtime-sized structures for variable soil-layer profiles.

## Recommended adoption set for openWEPP
- Static: [INFERENCE] Adopt now: Bevy-style explicit schedule graph concepts, Rapier-style authoritative timestep entrypoint, uom dimensional safety at kernel interfaces.
- Static: [INFERENCE] Adopt selectively: petgraph for topology validation, Rayon for bounded deterministic parallel slices, nalgebra for targeted numeric kernels.
- Static: [INFERENCE] Avoid direct adoption of game-engine runtime assumptions (render-loop semantics, frame-rate coupling).

## Notes
- Static: [INFERENCE] These exemplars are architectural pattern references, not behavioral or physics authorities.
