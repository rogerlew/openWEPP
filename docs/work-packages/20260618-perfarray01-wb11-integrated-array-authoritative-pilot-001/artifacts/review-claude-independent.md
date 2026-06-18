# PERFARRAY01 — Independent Review (Claude Code)

Verdict: **Correct, well-disciplined NO-GO — the structural proofs did exactly their job.**
Codex discovered statically that a WB11 pilot on the current request/scheduler seam *cannot*
satisfy "no per-day export" and "no dual-write," landed only the inert Stage A contract
shell, refused to fabricate perf evidence for a known-invalid path, and identified the real
prerequisite. The floor is still unmeasured (honestly), and — the important reframe — **it is
no longer cheap to measure: it now requires porting the request/accessor seam first.**

Evidence mode: **Static** (diff + the cited seam code) + **Ran** (verified inertness).

## The NO-GO is sound — I verified the crux

The blocker is real and code-cited: the kernel request reads scalars from logical
`BTreeMap`s (`core_types.rs:2453-2454`), built from `writeback_surface.state/flux_surface`
(`scheduler.rs:1606`), and the scheduler applies logical writeback then mirrors to the indexed
surface (`scheduler.rs:1676`/`:1714`). So a WB11 pilot that runs from `ArrayHotState` on the
**current** seam has only two paths, both invalid:

- materialize logical maps before the kernel → **per-day export** (PERFIDX03 trap, fails proof 1);
- keep logical as authority + mirror into the array → **dual-write** (PERFIDX05 trap, fails proof 2).

There is no honest third path without changing `HillslopeKernelRequest` so the kernel *reads*
from the array — which is Stage-C-scale authority work, explicitly out of PERFARRAY01's scope.
The conclusion ("Stage B cannot proceed honestly from the current request/scheduler
architecture") is correct. **Refusing to produce `perf` evidence for a path known to violate
both constraints is the right call** — that would have been theater.

## Stage A is good, inert groundwork

- New module `array_hot_state.rs` (674 lines) — `ArrayHotState`, `ArrayWritebackField/Payload`,
  `evaluate_array_writeback`, `apply_array_writeback`, `export_btreemap_surfaces`. Avoids
  bloating `core_types.rs` (line-count guardrail respected). Zero irrigation.
- **Genuinely inert:** the only wiring is `mod array_hot_state; pub use array_hot_state::*;`.
  I grepped — **zero references from any execution path**. Default path byte-identical by
  write-set. ✓
- **Well-tested,** including the property that matters: id-backed accept matches
  `evaluate_kernel_writeback`, reject preserves message-id class + subject, apply exports the
  same maps as logical apply, and a test asserts **zero new `BoundarySymbol` construction**
  during evaluation (the string-free success-path property). Focused `openwepp-kernel-contract`
  gates passed.
- Review-A is clean: Stage B not reclassified as complete, floor marked NOT RUN (not inferred),
  Stage A recorded as inert — no hollow green.

## Honestly not done (correctly stated)

Stage B, the floor, the two structural proofs (FAIL as scoped), bit-identity on H2637 — none
produced. 5× / ≤10× remain open; PERFIDX06's 73.12× stands; **ADR-0023 unratified**.

## The reframe the operator should weigh

The floor-first plan assumed the floor was a *cheap* measurement. PERFARRAY01 reveals it is
not: measuring the integrated WB11 floor **requires first porting the request/accessor authority
seam** (`HillslopeKernelRequest` + WB11 scalar accessors so the kernel reads from `ArrayHotState`
without export/dual-write). That is real production code (PERFARRAY02). So the choice is now:
invest in the seam port to get the real floor, trusting the PERFARCH02 synthetic ~49.9× as
directional evidence the investment is worth it — or decide the synthetic evidence alone is
enough to proceed/stop. Given the migration's size, measuring the real floor is worth the seam
port; but it is a real-code step, not a free measurement.

## Disposition

The NO-GO is correct and successful (it refined the dependency graph cheaply, before a doomed
pilot). Stage A is landable inert groundwork. Next: **PERFARRAY02** — request/accessor authority
split for WB11 runoff — which makes a valid array-authoritative request path possible **and**
runs the integrated floor measurement, so ADR-0023 ratification lands on the measured floor.
The operator decides whether to land the inert Stage A shell now or fold it into PERFARRAY02.
