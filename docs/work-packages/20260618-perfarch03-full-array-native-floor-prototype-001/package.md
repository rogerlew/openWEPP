# PERFARCH03 - Full Array-Native Floor Prototype (resume the perf program)

Status: complete 2026-06-18 (GO branch-floor disposition; <=10x / <=5x viability gate cleared for the measured flow)

Package type: **Architecture feasibility — the DECISIVE floor measurement that was never run.**
Build a prototype (throwaway/flag-gated, not production migration), measure the **true** array-native
floor of one real hot-path flow, and decide whether ≤10× (≤5×) is reachable. The full migration is
downstream, gated on this number.

## Why this exists — the perf program was suspended on a half-measure, not closed

The target — **≤10× (ideally ≤5×)** vs legacy on H2637 — is a **viability gate**. Current state:
**73.12×** (666.82 s; legacy 9.12 s). The read-side id-table work (PERFIDX01-04) took 978→666 s
(−31.9%, bit-identical) but kept the logical `BTreeMap` authoritative and the kernels producing
**logical** output. The array-authoritative re-architecture was scoped (PERFARCH02 / proposed ADR-0023)
but **only an input-only seam was ever built** (PERFARRAY01/02).

**The "21× floor" everyone stopped on is NOT the architecture's floor — it is the floor of a
half-measure.** PERFARRAY02's array-native runoff segment (817.8 µs/OFE-day) decomposes as:

| Component | µs/OFE-day | What it is |
|---|---:|---|
| kernel run | ~481 | the runoff kernel — **still building a `BoundarySymbol`-keyed logical payload internally** |
| `from_logical_payload` conversion | ~325 | pure logical→array conversion (would not exist if the kernel were array-native) |
| evaluate + apply | ~12 | array writeback |
| **array-native segment** | **817.8** | = **21.16×** legacy |

PERFARRAY02 migrated the kernel **input** (reads) but the kernel **still produces logical output and
runs logical machinery internally**. So 325 µs is pure conversion overhead and an unknown share of the
481 µs is internal symbol/map machinery — **not** physics. Legacy does the **whole OFE-day (all phases)**
in 38.65 µs, so a 481 µs **single-phase** run is almost certainly still symbol-machinery-bound, not
physics-bound. **That decomposition was never measured. This package measures it.**

## The two un-exploited facts

1. **The kernel-run cost was never separated into irreducible physics vs removable symbol machinery.**
   This is the whole question and it is unmeasured.
2. **Cache:** legacy RSS ~4.6 MB (cache-resident inner loop) vs openWEPP ~229 MB (~50×, pointer-chasing
   `BTreeMap`/`String`, cache-thrashing). A dense, few-MB, cache-resident array state is a multiplier on
   top of instruction-count savings — never exploited.

## The experiment — one flow, FULLY array-native (not input-only)

Take one representative hot-path flow (the WB11 **runoff** phase + its real guards/conservation is the
anchor, since PERFARRAY02 has the most data there) and make it **fully array-native end-to-end**:

- the kernel **reads** its inputs from a dense `Vec`/struct-of-arrays indexed by `SymbolId`;
- the kernel **computes** the real physics + real guards + real conservation;
- the kernel **writes** its outputs to the dense array by `SymbolId`;
- **ZERO `BoundarySymbol`, `BTreeMap`, `format!`, logical-payload construction, or logical↔array
  conversion anywhere in the hot loop** — the logical surface exists ONLY at the prototype boundary
  (seed once / materialize once, measured separately, as PERFARCH02 did with `export_once`);
- dense, **cache-resident** state (target a few-MB working set for the flow, like legacy).

This is the difference from PERFARRAY02: that pilot kept `from_logical_payload` + internal logical
machinery; **this strips all of it.**

## Honest-measurement discipline (non-negotiable — the comparator-artifact + ksatadj lessons)

- **Real work, not a stripped fake.** The prototype runs openWEPP's actual runoff physics, guards, and
  conservation — a prototype that skips them measures a fictional floor and is worthless.
- **Bit-identity validate** the prototype flow against the current path (the array path must produce the
  same result), so the measured cost is for a *correct* array-native flow.
- **Decompose the residual:** report kernel-physics cost vs any remaining machinery; prove the hot loop
  is logical-free (perf evidence: no `BTreeMap`/`memcmp`/`from_logical_payload`/`format!` in the flow).
- **Measure boundary seed/materialize separately** (transitional, removed by the full migration's
  authority flip), so the floor is the array-native execution.

## The decision (the deliverable)

Per-OFE-day array-native cost for the runoff flow + an extrapolation to the full OFE-day (all phases),
vs the budgets (**≤10× = 386 µs/OFE-day**, **≤5× = 193 µs/OFE-day**, legacy = 38.65 µs/OFE-day):

- **GO** — the array-native floor extrapolates to ≤10× (or ≤5×): **commit to the full migration** (rewrite
  the kernel phases array-native; the legacy fixed-array model). Author the staged migration plan + revive
  ADR-0023.
- **NO-GO / FLOOR-BOUND** — the array-native physics/loop is itself above budget: report the honest floor,
  and pivot the attack (reduce per-OFE-day work / substep structure / vectorization / a different
  state model) — **not** "perf is unreachable." State exactly where the irreducible cost lives.

A NO-GO here is real information (the physics/loop is the floor) and redirects the attack; it is **not** a
reason to abandon the viability target.

## Scope

In scope: the full array-native runoff-flow prototype (throwaway/flag-gated); bit-identity validation of
the flow; the floor measurement + physics-vs-machinery decomposition + cache/RSS measurement; the GO/NO-GO.

Out of scope:

- **No production migration** (downstream, gated on GO). The prototype is a measurement instrument.
- No claim from the PERFARRAY02 21× (it is the input-only half-measure, not this floor).
- Irrigation deferred.

## Acceptance Criteria

- **Fully array-native runoff flow** (kernel in + out + internal state), **proven logical-free in the hot
  loop** (perf evidence), bit-identical to the current path.
- **Floor measured:** array-native runoff per-OFE-day cost on the same machine as PERFIDX06; the
  physics-vs-machinery decomposition; the full-OFE-day extrapolation; the cache-resident working-set size.
- **Boundary seed/materialize measured separately.**
- **Decision:** GO (≤10×/≤5× reachable → full-migration plan + ADR-0023 revival) or NO-GO/FLOOR-BOUND
  (the honest floor + the redirected attack), with numbers, not assertions.
- Rust gates on whatever prototype code is produced; determinism preserved for the validated flow.

## Deliverables

- `artifacts/perfarch03-prototype.md` (what was built; how it is fully array-native; flag/harness)
- `artifacts/perfarch03-bit-identity.md` (the flow validated identical)
- `artifacts/perfarch03-floor-measurement.md` (array-native cost; physics-vs-machinery; OFE-day extrapolation)
- `artifacts/perfarch03-logical-free-proof.md` (perf evidence: no BTreeMap/memcmp/format!/conversion in the loop)
- `artifacts/perfarch03-cache-working-set.md` (RSS / working-set vs legacy's ~4.6 MB)
- `artifacts/perfarch03_disposition.md` (GO + migration plan / NO-GO + redirected attack)

## Execution Result

PERFARCH03 is complete with verdict **GO - branch floor clears <=5x and <=10x**.

The artifact-local prototype validates a representative WB11 warm-rain runoff
branch against the current public production kernel, then measures the fully
array-native branch hot loop separately from logical boundary materialization.
The validated branch output write set is 543 state symbols plus 8 flux symbols;
all numeric output values matched the production payload by exact `f64::to_bits()`.

Median release-binary results:

| Metric | us/OFE-day | Ratio vs legacy us/OFE-day | Projected H2637 seconds |
|---|---:|---:|---:|
| Current logical production kernel, same branch | 140.826054 | 3.643624x | 33.229457 |
| Array physics only | 0.074554 | 0.001929x | 0.017592 |
| Array dense output write only | 1.063708 | 0.027522x | 0.250994 |
| Array combined hot loop | 0.959423 | 0.024823x | 0.226386 |
| Boundary materialize once | 108.068963 | 2.796092x | 25.500061 |

The array hot loop contains no logical `BoundarySymbol`, `BTreeMap`,
`from_logical_payload`, or `format!` machinery at the reporting threshold in
the thresholded `perf report`. Dense slot working set is 18,208 bytes and direct
release-binary RSS is 3,072 KiB.

This result is a branch-floor measurement, not a full production migration or a
full H2637 endpoint proof. It authorizes the next package: revive the ADR-0023
array-authoritative migration direction and port production WB11 runoff first,
while preserving exact identity, measuring H2637 after each rung, and keeping
logical materialization out of migrated hot loops.

## Dependencies

- `docs/work-packages/20260618-perfarray02-wb11-request-accessor-authority-split-001/artifacts/{perfarray02-floor-measurement,perfarray02_disposition}.md` (the input-only 21×; the kernel-output lever)
- `docs/work-packages/20260618-perfidx06-high-ofe-target-assessment-001/artifacts/{perfidx06-bottleneck-analysis,perfidx06-legacy-ratio}.md` (73.12×; the budgets; the spread profile)
- `docs/work-packages/20260616-perfarch01-indexed-runtime-surface-design-001/artifacts/staged-implementation-plan.md` (the ~89-90% estimate; the staged plan)
- `docs/work-packages/20260618-perfarch02-array-authoritative-hot-path-state-redesign-001/artifacts/{perfarch02-redesign-shape,perfarch02-proposed-adr,perfarch02-floor-prototype}.md` (ADR-0023 shape; the surface prototype + harness pattern)
- `docs/decisions/0022-indexed-runtime-surface-representation.md` (the registry/`SymbolId` foundation)
- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_runoff_reconciliation.rs` + the WB11/runoff kernel path
- `docs/numerics/README.md`; `AGENTS.md`; `docs/work-packages/AGENTS.md`; `crates/AGENTS.md`

## Subagent Requirement

None required. The prototype + the floor measurement are local; record command + perf evidence.

## Autonomy

Execute end-to-end through the fully-array-native prototype, bit-identity validation, the floor
measurement + physics-vs-machinery decomposition + cache measurement, and the GO/NO-GO decision.
**Do not repeat the input-only pilot; do not conclude from the 21% half-measure.** The floor number is
the deliverable — it tells us whether the viability target is reachable by the full re-architecture, and
if not, exactly where the irreducible cost lives.
