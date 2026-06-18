# PERFARCH03 Kickoff — Full Array-Native Floor Prototype (resume perf)

Execution mode: architecture feasibility — the DECISIVE floor measurement. Build a prototype, measure
the **true** array-native floor, decide GO/NO-GO on the full re-architecture. No production migration.

Autonomy: execute end-to-end (prototype → bit-identity → floor measurement + physics-vs-machinery
decomposition + cache → GO/NO-GO). **Do not repeat the input-only pilot; do not conclude from the 21×.**

## Why — the perf program is SUSPENDED on a half-measure, and ≤10× is a viability gate

openWEPP is **73.12×** legacy on H2637 (666.82 s vs 9.12 s). Target: **≤10× (ideally ≤5×)** — imperative.
The read-side id-table work topped out at 73× (logical surface still authoritative; kernels still produce
logical output). PERFARRAY02's **21×** was an **input-only** pilot: its 817.8 µs/OFE-day = kernel-run 481 +
**logical-payload conversion 325** (`from_logical_payload`) + 12 — the kernel **still builds a
`BoundarySymbol`-keyed logical payload** and an unknown share of the 481 is internal symbol machinery, not
physics. Legacy does the WHOLE OFE-day in 38.65 µs, so 481 µs for one phase is machinery-bound. **Nobody
measured the floor of a FULLY array-native kernel. That is this package.**

## The experiment — one flow, FULLY array-native

Anchor on the WB11 **runoff** phase + its real guards/conservation. Make it array-native **end-to-end**:
kernel reads inputs from a dense `Vec`/struct-of-arrays by `SymbolId`, computes the real physics, writes
outputs to the dense array by `SymbolId`. **ZERO `BoundarySymbol`/`BTreeMap`/`format!`/logical-payload/
conversion in the hot loop** — logical surface only at the prototype boundary (seed once / materialize
once, measured separately). Dense, **cache-resident** state (few MB, like legacy's 4.6 MB).

The difference from PERFARRAY02: that pilot kept `from_logical_payload` + internal logical machinery;
**strip ALL of it** — that is the point (measure the floor without the machinery).

## Hard stops

1. **Real work, not a fake.** Run the actual runoff physics/guards/conservation. A stripped prototype
   measures a fictional floor (the comparator-artifact / ksatadj lessons).
2. **Bit-identity** the array flow vs the current path (correct array path, not a faster-but-different one).
3. **Prove the hot loop is logical-free** (perf evidence: no `BTreeMap`/`memcmp`/`from_logical_payload`/
   `format!`). **Decompose** the residual: physics vs machinery.
4. **Measure boundary seed/materialize separately** (transitional). Measure the **working-set / RSS**.
5. Irrigation deferred.

## The decision

Array-native runoff per-OFE-day cost + full-OFE-day extrapolation, vs budgets (**≤10× = 386 µs/OFE-day**,
**≤5× = 193 µs/OFE-day**, legacy = 38.65):

- **GO** → reaches ≤10× (or ≤5×): commit to the full migration (rewrite kernel phases array-native; revive
  ADR-0023; author the staged plan).
- **NO-GO / FLOOR-BOUND** → the array-native physics/loop is itself over budget: report the honest floor
  and **redirect the attack** (per-OFE-day work / substeps / vectorization / state model). State where the
  irreducible cost lives. This is information, **not** "perf is unreachable."

## Truthfulness

Floor, decomposition, cache, identity are empirical — label `Ran:`. Report the number honestly; do not
launder a half-measure as the floor. If the prototype can't be made fully logical-free without a contract
change, say so and scope it.

## Required reading

- `docs/work-packages/20260618-perfarch03-full-array-native-floor-prototype-001/package.md`
- `docs/work-packages/20260618-perfarray02-wb11-request-accessor-authority-split-001/artifacts/{perfarray02-floor-measurement,perfarray02_disposition}.md`
- `docs/work-packages/20260618-perfidx06-high-ofe-target-assessment-001/artifacts/{perfidx06-bottleneck-analysis,perfidx06-legacy-ratio}.md`
- `docs/work-packages/20260616-perfarch01-indexed-runtime-surface-design-001/artifacts/staged-implementation-plan.md`
- `docs/work-packages/20260618-perfarch02-array-authoritative-hot-path-state-redesign-001/artifacts/{perfarch02-redesign-shape,perfarch02-proposed-adr}.md` (+ the `perfarch02-floor-prototype/` harness pattern)
- `docs/decisions/0022-indexed-runtime-surface-representation.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_runoff_reconciliation.rs`
- `AGENTS.md`, `docs/work-packages/AGENTS.md`, `crates/AGENTS.md`, `docs/numerics/README.md`
