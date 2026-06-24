# Blocker Ledger

Evidence class: Static/Ran.

## R7H-003: H2637 direct performance and protected parity

State: closed `OPT-IN` by operator decision on 2026-06-24.

Inherited evidence:

- Prior package:
  `docs/work-packages/20260624-r7h-closure-activation-gates-001/`.
- Direct default-candidate endpoint after no-material frost fixes:
  `113.53 s / 1083636 KiB`.
- Budget: `<=91.2 s`.
- Manifest: `compatibility_edge_invocations=0`,
  `scheduler_kernel_executed=false`,
  `publication_source=direct-publication-frame`.
- Protected parity was red/not current-matrix green.

Current iterations:

- Reproduced current-code direct default-candidate timing failure:
  `112.99 s / 1083024 KiB`, exit `0`,
  `compatibility_edge_invocations=0`.
- Profiled direct default-candidate with `perf record -F 99 -g`: `115.52 s`,
  `12606` samples. `require_shadow_fine_state_domains` was the dominant
  in-envelope hot path due per-fine-layer symbol formatting/allocation.
- Corrected the guard path in
  `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost.rs`.
- Focused R7G orchestrator tests passed.
- Direct default-candidate now passes timing at `61.40 s`; explicit direct
  passes timing at `64.19 s`. Direct manifests report zero compatibility edges
  and direct publication provenance.
- Current-code default compatibility completed:
  `590.23 s / 227924 KiB`, `selected=compatibility`,
  `publication_source=scheduler-kernel`.
- Current-code explicit rollback completed:
  `600.20 s / 229164 KiB`, `selected=compatibility`,
  `publication_source=scheduler-kernel`.
- Default compatibility and explicit rollback are stable references:
  HBP/loss/PASS/plot/WAT checksums match exactly; WAT `EXCEPT ALL` row delta
  is `0/0`.
- Direct default-candidate and explicit direct match each other exactly and both
  report `compatibility_edge_invocations=0`.
- Direct-vs-compatibility parity remains red:
  HBP, WAT, and PASS differ; loss and plot match.
- WAT reduction shows first material divergence on Julian day 6. The first
  affected fields are frost/water-state fields: direct under-freezes relative
  to compatibility (`frozwt`, `frdp`) and retains the corresponding water in
  `Total-Soil`/`SoilWaterTotal`. Runoff (`Q`, `QOFE`, `UpStrmQ`, `SubRIn`,
  `latqcc`) diverges downstream from that frost split.
- PASS reduction is limited to hydrology-derived event fields: `runvol`,
  `sbrunv`, and `peakro`; sediment fields are clean.
- Existing `OPENWEPP_R7G_FROST_TRACE_*` instrumentation does not cover the typed
  direct frost branch; two filtered direct trace attempts completed with no
  trace file. This is an instrumentation limitation, not evidence of no frost
  execution.

Operator closure:

- The former `HOLD-R7H-TYPED-FROST-FREEZE-PARITY` is reclassified. Typed direct
  frost under-freezes relative to compatibility from the first active freezing
  day, but compatibility frost is not validated to frost-depth magnitude.
- R7H direct stays opt-in. The green performance evidence, zero compatibility
  counters, compatibility rollback path, and direct shadow path are retained.
- Default activation is not approved. The next work is frost-depth heat-flow
  fidelity against historic observations under reopened `GAP-SNOWFREEZE-002`,
  not direct-vs-compatibility frost bit-parity.
