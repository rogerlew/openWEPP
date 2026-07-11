# 6. How the architecture got here

The direct runtime's shape — typed mega-frames, phase spans, streaming
publication — is the residue of a measured, adversarial performance program.
Knowing the history is the fastest way to develop correct instincts about
what changes will and won't be accepted. The normative record is the
[array-native runtime specification](../architecture/array-native-runtime-specification.md)
and ADRs [0022](../decisions/0022-indexed-runtime-surface-representation.md),
[0023](../decisions/0023-array-authoritative-hot-path-state.md),
[0025](../decisions/0025-array-native-hillslope-day-frame.md),
[0030](../decisions/0030-r7-terminal-contract-and-compatibility-runtime-deletion.md),
[0031](../decisions/0031-delete-compatibility-runtime-single-authority-terminal.md);
this chapter is the story.

## 6.1 The symbol-map era, and the wall

The first working runtime represented simulation state as **symbol-keyed
maps**: every quantity addressed by its canonical WEPP name
(`BoundarySymbol`), phases exchanging `HillslopeKernelRequest` /
`KernelWritebackPayload` structures, a writeback evaluator validating each
value against its registered range before committing it. Maximally
introspectable, contract-friendly — and unusably slow: the H2637 endpoint ran
**~670 s vs 9.1 s legacy (~73×)**, against a viability gate of ≤10× (ideally
≤5×). This was existential: openWEPP is only viable as an engine if wepppy
can afford to run it.

The decisive experiment (PERFARCH03) ran one real water-balance branch two
ways on identical inputs: through the production symbol machinery
(**140.8 µs/OFE-day**) and as a plain array-native computation
(**0.96 µs/OFE-day, bit-identical outputs**). The arithmetic itself cost
0.075 µs. Conclusion, ratified as the program's premise: **the physics was
never the problem; the representation was.**

## 6.2 Incremental migration failed as a class

The obvious plan — migrate the hot path off the maps piece by piece — was
tried in many forms: dense mirrors for one phase, retiring hot symbols,
lane-owned dense state, caching layers, removing resynchronization
(PERFMIG01/02, PERFDEEP02/03/05/07). **Every one failed flat or negative**,
including runs at 2417 s and 1148 s — *worse* than the baseline they meant to
improve. The recurring mechanism: any partial migration keeps the old
representation alive alongside the new one, and the **synchronization between
the two representations costs more than the migration saves**.

Two program-level lessons were ratified from this arc and still bind work
today:

- **Type wholesale, never wrap.** A typed carrier that wraps a symbol-map
  computation is a *false single authority* — it adds a layer without
  deleting one. Replacement must be wholesale within a bounded surface.
- **The gate is the real endpoint.** Every one of the failed rungs had
  promising microbenchmarks. Acceptance is the full H2637 run, wall time and
  RSS, plus output identity — never a microbenchmark.

## 6.3 The rewrite (R0–R7) and the terminal deletions

The accepted response was a ground-up rewrite of the execution
representation — not of the science: physics, guards, units, process order,
and output schemas stayed contract-pinned, with byte/value identity of
protected outputs as the regression net. Staged as R0–R7 (the `rN` prefixes
still visible on phase names), it built the typed `DirectRunFrame` /
`DirectLaneFrame` / `DirectDayFrame` hierarchy and re-implemented the phase
sequence directly on it. The endpoint fell from ~670 s to **61–71 s (~7×)**,
inside the ≤10× viability gate.

Three completions made the new runtime *the* runtime rather than a fast mode:

1. **RSS arc (2026-06-30).** The direct path initially held ~1.13 GiB peak.
   Profile-first attribution found the dominant cause was a single
   run-length-scaled pre-allocation of per-day constructor inputs (~909 MiB)
   — not the state model. Publication became a streaming sink; memory is now
   run-length-flat (~80 MiB), byte-identical outputs.
2. **Typed seed authority.** Day-zero state (the last symbol-map computation)
   was re-implemented as `DirectProductionSeedAuthority` — wholesale, per the
   no-wrapping rule.
3. **Kernel-boundary deletion (2026-06-30).** With no production consumer
   left, the symbol-map scheduler, day-frame runtime, and carrier exports
   were deleted from the hillslope tier entirely; ADR-0031 removed the public
   `--compatibility-runtime` selector. Surviving `BoundarySymbol` references
   are I/O adapters, guard labels, and diagnostics — not executable runtime.
   The deletion also removed ~657 tests, whose contract assertions were then
   restored at the typed surfaces by a nine-package CQR burndown (closed
   2026-07-01).

Rollback for all of it is release/git history — deliberately not a runtime
flag, because keeping the old runtime reachable is exactly the dual-
representation trap of §6.2.

## 6.4 Where performance stands, and what's next

Measured 2026-07-01 (this repo, dual Xeon E5-2697 v2), a single day's arc:
the morning baseline was **71.4 s vs 9.65 s legacy = 7.40×**; the
mechanical sweep (guard-symbol deferral, curve hoisting — byte-identical
outputs) brought it to **46.7 s = 4.80×**, and the frost single-solve rewire
(one partition per day, applied at ingress; accepted by the frost
observation rubric rather than output identity) landed at
**32.8 s = 3.52×**, RSS ~77 MiB. **Both the ≤10× viability gate and the ≤5×
aspirational budget are met**, and the remaining profile is dominated by
physics for the first time in the program's history. The evidence trail is
[the sub-5× assessment](../backlog/20260701-hillslope-sub5x-performance-assessment.md)
and the two `20260701-hillperf-*` work packages.

The cost picture then gained a genuinely new term. Lane D — hourly
kinematic-wave routing along the OFE cascade — moved from shadow to
conditional **production default**
([rev-46 activation](../work-packages/20260708-laned-router-conditional-default-activation-001/),
2026-07-08): real added physics the daily numbers above never contained.
Its endpoint is a **different fixture** — the H2637 19-OFE geometry under a
2-year climate (`tests/fixtures/laned_shadow_h2637`), not the 34-year daily
run — so its times are not comparable to the 32.8 s above. On that fixture
the freshly active router cost **~37.5 s user**, dominated by the explicit
kinematic-wave solve. The buy-back was local numerics, not representation
work ([Tier-1, `SC-OFEROUTE-001` rev 47](../work-packages/20260708-laned-router-tier1-local-numerics-001/)):
closed-form rating evaluation on the selected friction branch, a bounded
log-Newton α/q solve, and `h·√h` replacing `h.powf(1.5)` in the hot
kinematic-wave step brought the endpoint to **11.90 s median (3.15×)** —
accepted by the Iwagaki oracle ladder and conservation gates rather than
output identity (rev 47 deliberately claims no bit identity; the recorded
hydrograph proxies landed bit-identical anyway). A
[post-Tier-1 hot-path sweep](../work-packages/20260708-laned-router-post-tier1-hotpath-sweep-001/)
settled it at **11.72 s median user, max RSS ~21 MiB** on this fixture. The
one recorded performance hold is the unratified Hirsch `Re^0.45`
approximation envelope.

One ratified caveat binds all H2637 numbers going forward:
[ADR-0037](../decisions/0037-abandon-hybrid-implicit-stepping.md) demoted
H2637 to a **synthetic stress case** — 19 uniform ~26 m OFEs is a
constructed stress configuration, not a fleet topology — and future
performance or promotion claims must not rest on H2637-only evidence. The same ADR is
the newest entry in the negative-results canon: a hybrid implicit-explicit
stepper was engineered to full working order — every named defect resolved,
all gates green — and abandoned anyway, because its only demonstrated win
was on the synthetic fixture while every real cohort member declined or
lost. A subsystem can be finished and still be net-negative carry.

An executional recheck on 2026-07-10
([audit](../audits/20260710_h2637_34yr_laned_active_endpoint_audit.md))
put both paths on the canonical 34-year endpoint for the first time since
Lane D activated. The daily path measured **39.2 s user / ~67 MiB** against
an interleaved same-core legacy anchor of **9.2 s** → **4.3×**, still
inside the ≤5× budget but ≈ +19% over the 2026-07-01 number with the
legacy control unchanged — real accumulated cost of the physics landed
since (erosion port, residue coupling, native intake, …), unattributed.
The active router **did not complete the run**: it fail-closed
deterministically at day 2621 (lane 8, `NegativeOutletBin`, a
zero-precipitation hard-freeze day on the recession tail of a ~46 mm
three-day rain spell — a span the 2-year fixture never reaches). The first
canonical-endpoint soak of the production default was a defect discovery,
not a timing datapoint; until it closes, the 2-year numbers above are the
only completed active measurements.

On the watershed tier the same playbook is repeating deliberately: measure
first (WSHEDPERF01 showed routing is ~0.08 s and hillslope fan-out is
everything), specify a ground-up target
([watershed runtime spec](../architecture/watershed-runtime-architecture-specification.md)),
rewrite wholesale, delete the old runtime at the end.

## 6.5 Instincts to carry into performance work

- Propose deletions of representation layers, not new caches over them. A
  package that adds a lookup layer without deleting a boundary meets a
  ratified stop-criterion.
- Claim endpoint numbers only from endpoint runs, and pair every performance
  claim with the identity evidence for protected outputs.
- Profile before attributing. The RSS arc's 909 MiB pre-alloc and the ~3%
  day-frame lifecycle (assumed dominant, measured marginal) are both recorded
  cases where the obvious suspect was innocent.
- Respect the negative results. If your idea resembles PERFMIG/PERFDEEP
  rungs or the hybrid stepper (ADR-0037), read those records first — the
  failure modes are documented in detail, and "this time it's different"
  needs to say *why*.
- Wins must generalize past the fixture that produced them. The hybrid
  stepper's one win was synthetic; performance claims are made on real
  cohort members, not H2637 alone (ADR-0037).
