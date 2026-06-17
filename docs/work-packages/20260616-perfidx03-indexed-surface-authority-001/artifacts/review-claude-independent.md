# PERFIDX03 — Independent Review (Claude Code)

Status: HOLD is correct on the flip — **but a net-new irrigation behavior change is
bundled in and should be extracted before anything lands.**
Evidence mode: **Static** (code/diff/history) + **Ran** (inert-path check)

## The flip HOLD is right

- The authority flip regressed OFE5 **+41.9%** (27.01→38.34 s, 3 interleaved
  samples), root-caused architecturally: the live path clones the sparse surface
  **then exports it back to a full `BTreeMap` per lane/day** for the kernel seam —
  the export dwarfs the clone win. Holding (disabling the flip; no-flip OFE5 back
  to 26.80 s) is the correct, disciplined call. Do not ship the flip.
- The blocker diagnosis + handoff (`PERFIDX03B`: stop exporting to a full BTreeMap
  on the hot path — have the kernel seam consume the indexed rep, or cache the
  export) is sound. **Deeper point:** this confirms the flip and the seam/lookup
  migration are *coupled* — the flip cannot win while the seam reads via a
  full-BTreeMap export, so part of Stage 4 (read-by-`SymbolId` at the seam) must
  come *with* the flip, not after. The PERFARCH01 staging assumed a clean
  flip-then-optimize; reality says otherwise.
- The diverse-management **registry-coverage gate succeeded** (0 unknowns across 5
  cases; frost fine-layer + the non-irrigation tightening). `clone_authoritative_writeback_surface`
  is **inert when the flip is off** (returns the plain `BTreeMap` clone when
  `indexed_writeback_surface` is `None`) — verified; bit-identical for the inactive
  path.

## The finding: PERFIDX03 silently activated irrigation

Pre-PERFIDX03 the hillslope **runner had zero irrigation wiring** (`git show
HEAD:…00_runner…` and `lane_setup_helpers` → 0 refs). The rest of the irrigation
pipeline already existed but was **dead**: the parsers (`irrigation_depletion`,
`irrigation_fixeddate`) exist in `openwepp-input-contract`; the hydrology kernel
has full irrigation logic (`hydrology/support_helpers_mod/irrigation.rs`, ~120
refs, **reads** `irrigation.fixeddate.event_*` / `irrigation.depletion.period_*`
symbols); the orchestrator has `build_hillslope_runtime_surface_from_irrigation_*`.
The only missing link was the runner's parse→build→seed wiring.

**PERFIDX03 added that wiring** (`00_runner` irrigation parsing + `lane_setup_helpers`
merging `irrigation_surface` into the static lane surface). Effect:

- **Non-irrigated runs** (no irrigation sidecars): empty surface → no-op →
  bit-identical. This is why the H2637 + 1–5-OFE cohort passed — and why the change
  is invisible to that cohort.
- **Irrigated runs**: irrigation symbols are now seeded, so the kernel's
  irrigation logic **activates** — irrigation is *applied* where it was previously
  inert. That is a **semantic behavior change** (process physics), not a
  behavior-preserving perf step.

Problems with shipping it as-is:
1. **Scope**: a behavior-activating feature inside a "behavior-preserving authority
   flip" HOLD; the disposition labels it "irrigation sidecar coverage," which
   undersells it.
2. **Contract-first (ADR-0011)**: activating irrigation application is process
   physics — it needs a governing `SC-*` and validation, not a side-effect of a
   perf package's registry-coverage gate.
3. **Unvalidated**: there is **no irrigated-run validation** — the anchor cohort is
   non-irrigated and the full anchor was skipped (speed gate failed first). Whether
   the now-active irrigation is physically correct is untested.
4. **Likely prompted by my scaffold** listing "irrigation" among the
   diverse-management cohort — the right response to "irrigation config produces
   uncovered symbols" was "irrigation isn't wired in the runner; scope it
   separately," not "wire it in here."

## Recommendation

1. **Extract the irrigation wiring out of PERFIDX03** (revert the `00_runner`
   irrigation parse/build + the `lane_setup_helpers` `irrigation_surface` merge).
   Keep the safe HOLD parts: inactive flip plumbing, `clone_authoritative` (inert),
   frost-fine + non-irrigation registry tightening, the artifacts/HOLD record.
2. **Scope irrigation activation as its own contract-first package** — grounded in
   the WEPP irrigation science contract, validated on an *irrigated* fixture
   (vs legacy/contract), reviewed as the feature it is. (It may well be a correct
   latent-feature completion — but it must be validated as one.)
3. **PERFIDX03B** (the flip blocker) proceeds without the irrigation entanglement;
   the diverse-cohort registry gate should drop irrigation until the separate
   irrigation package lands.

I have **not committed** the PERFIDX03 code, pending the decision on the irrigation
wiring. The HOLD verdict on the flip itself stands regardless.

## Decision taken (2026-06-17, operator-approved)

The operator's direction: irrigation is **eventually-planned** future work and must
run **only when the management declares it**. Given that, and that the held flip is
reworked from scratch in `PERFIDX03B` anyway, we took the *stronger* of the two
options in recommendation #1: **discard all PERFIDX03 working-tree code** (not just
the irrigation wiring — the held flip plumbing and the registry tightening too),
rather than land a surgical subset. The committed PERFIDX03 record is **docs-only**;
`crates/` returns to the PERFIDX02-complete state. The registry-coverage fixes
(frost fine-layer + the reachable `ncut`/`ncycle` tightening) are documented here and
in the evidence artifacts, and `PERFIDX03B` re-applies them clean — without
irrigation. Irrigation is captured as
`docs/backlog/20260617-irrigation-management-gated-activation.md` (management-gated;
out of scope for the perf migration).
