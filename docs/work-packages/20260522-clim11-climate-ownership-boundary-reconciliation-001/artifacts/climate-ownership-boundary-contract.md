# Climate Ownership Boundary Contract

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- Reviewed architecture ADRs (`ADR-0004`, `ADR-0006`) and climate seam code
  surfaces in hillslope and watershed orchestrators.
- Reconciled CLIM04 accepted finding `CLIM04-RVW-003` into an explicit routing
  authority contract.

Ran:
- Executed repository inspections to confirm current implementation and test
  usage (`runtime_inputs.rs`, `parser_runtime_seam_integration.rs`).

## Reconciliation Target
Resolve mismatch between:
1. HBP-first watershed architecture narrative (`ADR-0004`, `ADR-0006`), and
2. Existing watershed climate-assignment runtime seam behavior (`WS-CLIM-SEAM-001`).

## Authority Boundary

| Surface | Authority owner | Allowed responsibilities | Disallowed responsibilities |
|---|---|---|---|
| `HS-CLIM-SEAM-001` (`climate parser -> hillslope runtime symbols`) | `openwepp-hillslope-orchestrator::runtime_inputs` | Validate climate runtime policy, adapt climate forcing to canonical kernel symbols, seed hillslope runtime surfaces. | Delegating climate-physics adaptation authority to downstream watershed routing once hillslope execution is authoritative. |
| `WS-CLIM-SEAM-001` (`climate assignment map -> watershed runtime symbols`) | `openwepp-watershed-orchestrator::runtime_inputs` | Project per-hillslope climate assignment metadata/symbols for in-process watershed orchestration surfaces (integration probes, parity instrumentation, shared-adapter migration work). | Replacing HBP as production cross-binary routing authority or redefining hillslope climate physics outcomes post-handoff. |
| Cross-binary hillslope-to-watershed handoff | HBP contract (`ADR-0004`, `ADR-0006`) | Transfer completed hillslope outcomes to watershed routing. | Pulling direct parser climate payloads across the binary boundary as a required routing input when HBP is present. |

## Routing Authority Contract
1. Climate parser output is adapted under seam authority (`HS-CLIM-SEAM-001` and
   scoped `WS-CLIM-SEAM-001`) with typed guards and explicit unit conversion.
2. Production watershed routing authority remains HBP-first.
3. Climate-file-to-hillslope assignment ownership belongs to orchestration
   layers upstream of hillslope execution; watershed may carry assignment
   metadata but does not re-own climate physics after HBP handoff.
4. Any future shared climate adapter extraction (CLIM12) must preserve this
   ownership split and may only centralize implementation, not move authority.

## Acceptance Criteria
1. Ownership boundary is explicit across hillslope seam, watershed seam, and
   HBP cross-binary contract: `met`.
2. No implicit watershed ownership claim remains for production climate routing:
   `met` (codified in `ADR-0013`).
3. Allowed adaptation locations are explicit and testable: `met`.
4. Required relocation of code ownership is not needed in CLIM11 scope and is
   explicitly queued to CLIM12: `met`.

## CLIM04 Finding Closure Mapping

| Finding | Severity | CLIM11 disposition | Notes |
|---|---|---|---|
| `CLIM04-RVW-003` | high | `closed` | Resolved by explicit authority split and ADR ratification (`ADR-0013`) while preserving HBP-first routing authority. |

## Evidence
- [DIRECT] `docs/decisions/0004-subprocess-hillslope-orchestration.md:12-17`
- [DIRECT] `docs/decisions/0006-three-binaries-incl-replay.md:15-19`
- [DIRECT] `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:469-687`
- [DIRECT] `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs:505-687`
- [DIRECT] `tests/integration/parser_runtime_seam_integration.rs:230-258`
- [DIRECT] `docs/work-packages/20260522-clim02-climate-parser-to-runtime-seam-adapters-001/artifacts/climate-seam-adapter-ownership-contract.md:13-22`
- [DIRECT] `docs/decisions/0013-climate-forcing-ownership-boundary.md`
