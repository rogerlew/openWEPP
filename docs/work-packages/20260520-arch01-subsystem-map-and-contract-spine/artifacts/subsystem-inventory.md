# Subsystem Inventory

Status: complete
Date: 2026-05-20 UTC
Evidence mode: `Static`
Ran evidence: none in this kickoff execution

## Subsystems

| Subsystem ID | Name | Responsibility | Primary Inputs | Primary Outputs | Authority | Evidence |
|---|---|---|---|---|---|---|
| SS-01 | Input Contract Layer | Parse/validate `.run`, legacy `.txt` sidecars, soil, management, climate, watershed topology; normalize legacy and schema modes into one typed run model; reject missing required selectors. | WEPP input files + `.run` + legacy sidecars + release sidecars | typed run/config model | `docs/contracts/README.md`, `docs/contracts/openwepp-runner-contract.md`, ADR-0011 | `[DIRECT][Static]` |
| SS-02 | Typed State Surface Layer | Define canonical typed state surfaces, units, and tolerance-bearing state contracts. | typed run/config model | typed state surfaces + units manifests | `docs/specifications/README.md`, `docs/contracts/routine-interface-v1.md` | `[DIRECT][Static]` |
| SS-03 | Hillslope Simulation Orchestrator | Execute hillslope time progression; dispatch kernels; emit hillslope artifacts. | SS-01 config + SS-02 state + climate forcing | HBP shard + hillslope parquet + diagnostics | `docs/architecture/README.md`, ADR-0006 | `[DIRECT][Static]` |
| SS-04 | Watershed Routing Orchestrator | Load watershed structure; consume hillslope shards; route channels/network. | watershed topology + HBP shard set + SS-02 state | watershed trajectories + watershed parquet + diagnostics | `docs/architecture/README.md`, ADR-0004, ADR-0006 | `[DIRECT][Static]` |
| SS-05 | Kernel Routine Interface Layer | Provide routine descriptors and pure kernel execution contract (`describe/validate/run/validate_output`). | SS-02 state slices | state deltas/fluxes + kernel status | `docs/contracts/routine-interface-v1.md`, ADR-0008 | `[DIRECT][Static]` |
| SS-06 | Invariant and Closure Layer | Enforce physical invariants, contract invariants, and closure checks with tier-aware outcomes. | trajectories/state surfaces + comparator context | hard-fail errors or investigation records | ADR-0011, `docs/specifications/README.md`, `docs/numerics/README.md` | `[DIRECT][Static]` |
| SS-07 | Replay and Comparator Layer | Drive replay from HBP; produce parity deltas and first-divergence attribution. | HBP shard + replay spec + tolerance manifests | comparator delta reports + investigation packets | ADR-0006, ADR-0011, ADR-0003 | `[DIRECT][Static]` |
| SS-08 | Output and Interchange Layer | Emit/ingest HBP and parquet surfaces under pinned interchange contracts. | SS-03/SS-04 trajectories | HBP files + parquet files | ADR-0005, `docs/contracts/README.md` | `[DIRECT][Static]` |
| SS-09 | Runner/Release and Error Governance Layer | Own launcher contract, release sidecar validation, and strict failure posture (no silent fallback). | binary artifacts + sidecars + engine selector | typed launch outcomes + release lint verdicts + telemetry | ADR-0007, `docs/contracts/openwepp-binary-release-contract.md`, `docs/contracts/openwepp-runner-contract.md` | `[DIRECT][Static]` |

## Boundary Notes

- Legacy static inspection is secondary authority only for ordering/provenance and not acceptance authority (`[DIRECT][Static]`, ADR-0011).
- Legacy `.run` + sidecar compatibility is an initial bridge, not a silent
  fallback surface (`[DIRECT][Static]`, ADR-0011, `docs/contracts/README.md`).
- Current repository state is docs-first and pre-kernel; subsystem decomposition is architecture-authority aligned and code-ready rather than code-derived (`[INFERENCE][Static]`).
- `references/50201000` is now synced under `openWEPP/references` from `wepp-forest` (2026-05-20), so chapter-level citation extraction is unblocked (`[DIRECT][Static]`).
