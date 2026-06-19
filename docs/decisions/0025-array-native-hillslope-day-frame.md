# ADR-0025: Array-native HillslopeDayFrame hot-path runtime

**Status:** Accepted
**Date:** 2026-06-18 UTC (ratified 2026-06-18)
**Deciders:** Roger Lew (operator ratification 2026-06-18), Claude Code (specification authoring), Codex (implementation)
**Builds on / supersedes:** completes [ADR-0023](0023-array-authoritative-hot-path-state.md)
(dense-authority principle) and **supersedes its incremental application**; narrows
[ADR-0022](0022-indexed-runtime-surface-representation.md) (registry → I/O-only)
**Design authority:** [`docs/architecture/array-native-runtime-specification.md`](../architecture/array-native-runtime-specification.md)

## Context

≤10× (ideally ≤5×) vs legacy on H2637 is the production viability gate. openWEPP is **73.12×**; PERFIDX06
profiling proves the cost is symbol-keyed-map machinery as a class (`BTreeMap`/`memcmp`/alloc/`format!`),
not physics — PERFARCH03 measured the physics floor at sub-µs (a fully array-native runoff branch is 146.8×
faster, byte-identical).

Two incremental rungs under ADR-0023 failed for the same structural reason: **partial dual-representation
migration is dominated by bookkeeping.** PERFMIG01 (writeback-only) went +0.47%; PERFMIG02 (reader /
materialization retirement) was flat/negative (REDIRECT), and retiring six symbols cost *more* than
materializing all of them (stale-logical removal > avoided inserts). The win requires migrating a
**complete unit** so internal seams vanish, not retiring seams one at a time.

## Decision

Adopt a **comprehensive array-native hot-path architecture**: replace the symbol-keyed
`BTreeMap<BoundarySymbol, BoundaryValue>` per-OFE-day state with a single typed dense **HillslopeDayFrame**
(named unit-typed scalar fields + struct-of-arrays for hourly/layer families) that all 14 phases mutate in
place. Kernels become pure functions over `&mut HillslopeDayFrame` — no writeback payloads, no symbol
resolution, no inter-phase materialization. Guards become typed field checks with a two-tier policy:
compile-time invariants for static bounds/finiteness plus runtime-derived bounds where current behavior is
state/branch dependent. Migration preserves finite/domain/closure invariants, message-id classes, and
boundary-level diagnostic attribution policy. Logical/symbol surfaces survive only at explicit I/O and
diagnostic boundaries: the minimal HBP edge is ~5 scalars captured as typed values during the run, and
current WB13 publication operand extraction must be migrated from runtime-surface reads to typed
frame/projection access before logical hot-path deletion.

The full design — frame structure, the I/O-edge analysis, the performance model (~14–20 µs/OFE-day ≈ 0.5×
legacy), the identity-gating strategy, the staged plan, the open forks — is the linked specification, which
this ADR ratifies as binding design authority.

## Consequences

Positive:
- Phase-to-phase seams (the 108 µs tax that sank PERFMIG01/02) **vanish by construction**; the boundary
  collapses to file I/O.
- Removes symbol-keyed map work, allocation churn, `format!`, and dual publication from the hot path **as a
  class** (the PERFIDX06 prescription).
- Fulfils the declared "kernels are pure functions over typed state" kernel boundary; collapses RSS
  ~228 MB → ~3 MB (cache-resident).
- Projects closure of the ≤10× / ≤5× viability gate per the model; acceptance depends on staged H2637
  endpoint measurements.

Negative / cost:
- Large blast radius (~10.8k kernel lines + scheduler + contract). Mitigated by staged, shadow-run,
  identity-gated execution.
- Changes the kernel signature and the orchestrator's state-threading; migration must carry an explicit
  contract-compatibility map across crates while mixed-mode execution exists. The registry/indexed-surface
  machinery (ADR-0022) leaves the hot path.

## Required gates

- Exact identity per migrated phase/branch (`to_bits()`), including snow/frost/irrigation/MOFE — shadow-run
  differential against the logical path until a stage is authoritative.
- H2637 `.hbp` + `wat.parquet` byte-identical, `pass.parquet` Arrow-equal, per stage.
- Publication operand-lineage ledger is complete (HBP + WB13/WAT/PASS + manifest provenance), and typed
  publication projection fixtures are green before logical hot-path deletion.
- Guard parity fixtures cover static and runtime-derived bounds with accept/reject parity on message-id
  classes and diagnostic attribution policy.
- H2637 endpoint + RSS measured same-machine vs PERFIDX06 per stage; the endpoint is the perf authority.
- Migrated hot-frame state records layout/type-size and allocation evidence, including frame slot/field
  counts and proof that normal success paths do not retain `format!`, owned symbol cloning, collection
  rebuilds, or equivalent allocation helpers in the migrated phase loop.
- Determinism + `SC-*` closure/conservation gates green throughout.
- The Stage-1/2 **falsification check**: a properly-sized hydrology island that does not move the endpoint
  halts the program for re-profiling.
- Kernel-contract transition compatibility is explicit and tested for each stage (or a versioned breaking
  contract is ratified before cutover).
- Workspace Rust gates + `cargo deny` + markdown lint on touched scopes.

## Authority posture

Ratified 2026-06-18 — ADR-0025 is the accepted hot-path runtime authority. ADR-0023's dense-authority
*principle* is retained and fulfilled by this ADR; its *incremental application* (the PERFMIG01/02
symbol/phase migration tactic) is superseded — **no further writeback-only or materialization-retirement
rungs.** Execution proceeds as the staged, identity-gated `PERFDEEP0N` series under the Required-gates
discipline above.

Amendment 1, 2026-06-19 — the linked specification's Rust implementation-gotcha review adds the
layout/type-size and allocation-evidence gate above. This amendment does not change the adopted
architecture; it hardens the evidence required before direct-frame stages can claim hot-path closure.

## Non-decisions

This ADR does not change physics/numerics (results are byte-identical), `SC-*` science contracts, HBP
(ADR-0012) or parquet/JSON output schemas (ADR-0019/0020), the subprocess-per-hillslope model (ADR-0004),
the 14-phase DAG ordering, MOFE routing topology, determinism policy, or irrigation activation. It does not
authorize a big-bang rewrite — execution is the staged, identity-gated `PERFDEEP0N` series defined in the
specification.
