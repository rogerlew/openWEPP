# Follow-on architecture implementation WP queue

Evidence: Static
Ran evidence: none

## Queue principles
- Static: [INFERENCE] Sequence follows dependency direction from contracts -> topology/scheduler -> kernel integration -> compatibility/reporting.
- Static: [INFERENCE] No package is marked implementation-ready unless upstream ownership and status contracts are complete.

## Proposed queue
1. `20260521-arch03-sim-contract-crate-and-status-taxonomy-001`
- Scope: implement unified typed status + closure + symbol alias registry.
- Blocks: all downstream implementation WPs.

2. `20260521-arch04-topology-graph-and-validation-gate-001`
- Scope: watershed/hillslope topology graph model + closure checks.
- Depends on: `arch03`.

3. `20260521-arch05-hillslope-phase-scheduler-graph-001`
- Scope: deterministic hillslope phase scheduler and explicit dependency edges.
- Depends on: `arch03`, `arch04`.

4. `20260521-arch06-watershed-dispatch-scheduler-graph-001`
- Scope: deterministic watershed routing/impoundment dispatch scheduler.
- Depends on: `arch03`, `arch04`.

5. `20260522-arch07-kernel-trait-boundary-and-writeback-contract-001`
- Scope: kernel trait signatures and orchestrator-controlled writeback contracts.
- Depends on: `arch03`, `arch05`, `arch06`.

6. `20260522-arch08-sidecar-and-legacy-bridge-adapter-isolation-001`
- Scope: isolate sidecar/HBP compatibility adapters into edge modules.
- Depends on: `arch03`, `arch04`.

7. `20260522-arch09-unit-safe-boundary-types-001`
- Scope: dimensional safety wrappers for runoff/flow/storage/rate interfaces.
- Depends on: `arch03`, `arch07`.

8. `20260522-arch10-summary-accumulator-kernelization-001`
- Scope: daily/monthly/yearly/EOS accumulation kernels with typed status.
- Depends on: `arch06`, `arch07`.

9. `20260522-arch11-comparator-tier-routing-metadata-integration-001`
- Scope: confidence-tier metadata propagation through reporting/comparator outputs.
- Depends on: `arch03`, `arch10`.

10. `20260522-arch12-wave4-readiness-closeout-001`
- Scope: architecture gate review, unresolved risk disposition, and Wave 4 ratification packet.
- Depends on: `arch03`..`arch11`.

## Gate note
- Static: [INFERENCE] If any high-severity ownership ambiguity remains unresolved at `arch12`, disposition is `HOLD`.
