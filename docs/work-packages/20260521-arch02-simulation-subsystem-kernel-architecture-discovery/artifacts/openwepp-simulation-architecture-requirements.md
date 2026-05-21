# openWEPP simulation architecture requirements

Evidence: Static
Ran evidence: none

## Requirement set

### OA-SKA-001 Architecture-first contract authority
- Static: [DIRECT] ADR-0011 establishes architecture-first and top-down science contracts (`/home/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md:18`, `:23`).
- Static: [INFERENCE] Simulation architecture shall treat typed state/contracts as the authority boundary; legacy code is secondary provenance.

### OA-SKA-002 Explicit subsystem ownership
- Static: [DIRECT] wepp-forest currently mixes orchestration and broad mutable state ownership (`/home/workdir/wepp-forest/src/contin.for:57`, `/home/workdir/wepp-forest/src/wshdrv.f90:308`).
- Static: [INFERENCE] Every openWEPP state surface shall have one owning subsystem and one owning crate/module.

### OA-SKA-003 Directed dependency graph
- Static: [DIRECT] watershed topology validation and stop-on-invalid behavior exists in legacy (`/home/workdir/wepp-forest/src/wshinp.for:214`, `:269`).
- Static: [INFERENCE] openWEPP shall define an explicit directed dependency graph for simulation phases and topology nodes before execution.

### OA-SKA-004 Deterministic scheduler semantics
- Static: [DIRECT] Bevy docs indicate parallel default ordering ambiguity without explicit constraints.
- Source: https://docs.rs/bevy/latest/bevy/ecs/system/index.html
- Static: [INFERENCE] openWEPP scheduler shall require explicit phase ordering and deterministic reduction order for any parallel execution.

### OA-SKA-005 Authoritative timestep entrypoints
- Static: [DIRECT] Rapier uses a single `step` entrypoint for staged simulation advancement.
- Source: https://docs.rs/rapier3d/latest/rapier3d/pipeline/struct.PhysicsPipeline.html
- Static: [INFERENCE] openWEPP shall expose one authoritative entrypoint per mode: hillslope-daily, hillslope-hourly, watershed-routing.

### OA-SKA-006 Typed kernel I/O contracts
- Static: [DIRECT] wepp-forest kernelized modules expose typed inputs/state/flux/status structures (`/home/workdir/wepp-forest/fpm-src/watbal_process_types.f90:59`, `:166`, `:189`).
- Static: [INFERENCE] openWEPP kernels shall not read global mutable state directly; all inputs/outputs must be explicit typed structs.

### OA-SKA-007 Uniform status and error taxonomy
- Static: [DIRECT] kernel status with `ok/finite_ok/domain_ok/message_id` and fail-stop callsites are present in watershed kernels (`/home/workdir/wepp-forest/fpm-src/watbal_process_types.f90:189`, `/home/workdir/wepp-forest/src/wshrun.f90:179`).
- Static: [INFERENCE] openWEPP shall use a uniform typed status taxonomy and reject silent fallback in production paths.

### OA-SKA-008 Sidecar and legacy bridge isolation
- Static: [DIRECT] sidecar and HBP bridge logic is orchestration-owned in legacy (`/home/workdir/wepp-forest/src/infile.for:1544`, `/home/workdir/wepp-forest/src/hbp_mode2_bridge.f90:10`).
- Static: [INFERENCE] openWEPP shall isolate sidecar parsing and legacy bridge I/O into adapter layers outside kernel crates.

### OA-SKA-009 Canonical symbol continuity with explicit aliases
- Static: [DIRECT] canonical WEPP symbols (`runoff`, `runvol`, `sbrunf`, `drainq`, `sep`, `st`, `frzw`) are still the science-facing vocabulary in legacy/kernels (`/home/workdir/wepp-forest/src/wshrun.f90:124`, `:173`).
- Static: [INFERENCE] openWEPP shall preserve canonical symbol names in contract tables and provide explicit alias maps when Rust field names differ.

### OA-SKA-010 Dimensional safety at interfaces
- Static: [DIRECT] `uom` provides compile-time dimensional analysis with zero-cost quantity operations.
- Source: https://docs.rs/uom/latest/uom/
- Static: [INFERENCE] openWEPP shall adopt unit-safe wrappers at subsystem/kernel boundaries where mixed units are currently implicit.

### OA-SKA-011 Topology representation and validation utilities
- Static: [DIRECT] `petgraph` supports directed graphs with arbitrary node/edge data.
- Source: https://docs.rs/petgraph/latest/petgraph/
- Static: [INFERENCE] openWEPP should use explicit graph representations for watershed topology and scheduler dependency validation.

### OA-SKA-012 Controlled parallelism policy
- Static: [DIRECT] Rayon exposes explicit task and thread-pool controls (`join`, `scope`, `ThreadPoolBuilder`).
- Source: https://docs.rs/rayon/latest/rayon/
- Static: [INFERENCE] openWEPP may parallelize independent partitions only under deterministic merge rules and explicit pool configuration.

### OA-SKA-013 No reflective mutation control surface
- Static: [DIRECT] rancor exposes runtime reflection-based property/method mutation over API (`/workdir/rancor/Rancor/Rancor/UnitModel.cs:67`, `/workdir/rancor/Rancor/WebAPI/Controllers.cs:52`).
- Static: [INFERENCE] openWEPP shall prohibit string-reflection mutation of simulation state in production execution paths.

### OA-SKA-014 Explicit comparator confidence-tier routing
- Static: [DIRECT] ADR-0011 defines confidence tiers and comparator deltas as investigation signals (`/home/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md:32`, `:34`).
- Static: [INFERENCE] architecture outputs shall carry comparator-tier metadata per subsystem surface.

### OA-SKA-015 Compatibility errors are explicit
- Static: [DIRECT] ADR-0011 requires no silent fallback for missing sidecar requirements (`/home/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md:42`, `:43`).
- Static: [INFERENCE] adapter layers shall return typed errors for missing required artifacts and ambiguous compatibility states.
