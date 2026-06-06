# ADR-0017: Re-pin operational distrust — the fixed comparator is a flag, not a target

**Status:** Accepted
**Date:** 2026-06-05
**Deciders:** Roger Lew, Codex
**Author of draft:** Claude Code (drafted at decider Roger Lew's direction; ratified by Codex package `20260605-adr0017-comparator-distrust-ratification-001`)
**Amends:** [ADR-0016](0016-promote-260430-baseline-as-canonical-comparator-and-abandon-kernel-rewrite.md)
**Reaffirms:** [ADR-0011](0011-architecture-first-top-down-science-contracts.md)

## Context

ADR-0016's intent was narrow and correct: fold the negative-melt fix into the
comparator so reviewers stop carrying the "is this residual an openWEPP defect or
openWEPP correctly diverging from a buggy baseline?" question on every comparison.
Its **text** preserved distrust — it cites ADR-0011 as still governing, states it
"does not make legacy behavior a universal correctness oracle," and requires
"contract/source/conservation evidence" before any divergence is labeled an
openWEPP defect.

The **operating effect** diverged from that text. Three rationale phrases —
"Restores 39/39 as a legitimate goal," "openWEPP must match it," "Narrows the
acceptance escape hatch" — were scoped in the ADR to the negative-melt class, but
that scope boundary lived only in the reader's head. Across the HPHYS0298→0313
arc it generalized into a working prior that inverts ADR-0011's burden of proof:
a baseline-vs-openWEPP divergence became a presumptive **openWEPP defect to close**
rather than a like-for-like question to adjudicate. The inversion was hard-coded
into the verdict taxonomy — `OPENWEPP-DEFECTIVE` as the open default class,
`LEGACY-DEFECTIVE` "reserved" for the negative-melt case — with no verdict class
for a harness/surface mismatch.

The cost is now evidenced:

- **The keystone verdict is a unit artifact.** HPHYS0298's "all nine windows
  `OPENWEPP-DEFECTIVE @ hrsnow`" compared baseline snow **depth** against
  openWEPP **water-equivalent** (`snow_hourly_snowfall_water_equiv_sum_m`); the
  "~10×" is the fresh-snow density factor. The package's own criterion C
  (independent correctness authority) was recorded "partially met" and waived
  ("not required to act here"). That verdict has since been retracted
  in-package and HPHYS0299 now supplies the corrected depth-vs-depth continuation
  authority. Evidence:
  `docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/artifacts/review_claude_hrsnow_unit_artifact.md`,
  `docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/artifacts/disposition.md`, and
  `docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/artifacts/review-disposition.md`.
- **The arc closed nothing.** HPHYS0298→0313 = 16 packages, **0 authorized
  production physics edits** (the only `.rs` change is HPHYS0305 trace
  instrumentation). Evidence command:
  `git diff --name-status bcbec46^..30f20aa -- '*.rs' ':!tests/**' ':!docs/**'`
  returns only `crates/openwepp-runner/src/hillslope/mod.rs`; inspecting that
  diff shows trace-schema additions for hourly snow/rain/depth/density fields,
  not process-physics edits.
- **The predicted recurrence happened.** ADR-0016 precondition #8 warned that
  without like-for-like rigor "a third comparator artifact after `hrsnow` and H39
  is likely." HPHYS0313 is that artifact — a re-misattributed `hrsnow`/`driftg`
  surface, 15 packages later. The guardrail existed as prose and had no teeth.
  Evidence:
  `docs/work-packages/20260605-hphys0313-snowpack-settling-carry-recursion-closure-001/artifacts/review_claude_settling_route_misattribution.md`.

The erosion is in the telos and the taxonomy, not the constitution — which is why
it has to be re-pinned explicitly. ADR-0011's distrust was a standing principle
with no forcing function to keep practice aligned to it.

## Decision

1. **Preserve ADR-0016's correct core.** The negative-melt confound collapse
   stands. The fixed comparator branch
   `wepp_260430_negmeltfix_comparator`, tag
   `wepp_260430_negmeltfix_comparator_47ac4c32faee`, commit
   `47ac4c32faeea81bb99081f955a14c38b815ef4d` remains the single active
   H1..H39 comparator artifact, and the 260430 lineage anchor (ADR-0012) is
   unchanged. This ADR reverses none of that.

2. **The comparator is a flag, not a target.** Retire "39/39 against the
   comparator" as an objective function. Matching the fixed baseline is **not** a
   goal; satisfying the SC-* contracts, conservation/closure invariants, and
   external physics authority is the goal (per ADR-0011 and the correctness
   re-anchoring direction). Comparator agreement is a sanity flag, never an
   acceptance oracle.

3. **Restore the ADR-0011 burden of proof.** A baseline-vs-openWEPP divergence is,
   by default, a **like-for-like adjudication question** — not a presumptive
   openWEPP defect. It may not be labeled `OPENWEPP-DEFECTIVE` until both: (a) the
   paired surfaces are proven identical in units and lineage stage, and (b)
   independent correctness authority (acceptance criterion C) is **met, not
   waived**. "Not required to act here" is no longer an acceptable disposition of
   criterion C for an openWEPP-defect verdict. Criterion C is not waivable for
   an `OPENWEPP-DEFECTIVE` finding.

4. **Symmetric verdict taxonomy with a first-class harness verdict.** Replace the
   asymmetric `{OPENWEPP-DEFECTIVE default, LEGACY-DEFECTIVE reserved}` scheme.
   Comparator/ledger packages must support, as peer verdicts:
   `HARNESS-SURFACE-MISMATCH`, `LEGACY-DEFECTIVE`, `OPENWEPP-DEFECTIVE`, and
   `UNRESOLVED`. A depth-vs-water-equivalent, raw-vs-released, or
   lineage-stage-mismatched pairing resolves to `HARNESS-SURFACE-MISMATCH`, not to
   an openWEPP defect.

5. **Make like-for-like a gate, not a guideline.** The paired-lineage harness must
   verify unit and lineage-stage identity at the point each paired surface is
   added, and **fail closed** when it cannot. ADR-0016 precondition #8 becomes an
   executable gate. A suspiciously round ratio (≈10×, ≈1000×) or large delta is
   treated as a surface-mismatch hypothesis first.

6. **Findings must resolve to a decision (anti-rot teeth).** A comparator finding
   may not linger indefinitely as an ownerless or unscoped `HOLD`. Truthful
   `HOLD` remains valid when it records an owner, next evidence gate, and
   continuation artifact. Every finding resolves to one of: *fixed*,
   *deliberately kept in HOLD with cited reason, owner, and next gate*, or
   *harness-corrected*. A finding that invalidates a prior verdict must
   **retract that verdict in-package**, not leave it standing. HPHYS0298 is now
   the precedent: its `STRONG APPROVE` / `OPENWEPP-DEFECTIVE @ hrsnow` record was
   superseded in-package by the accepted unit-artifact review.

7. **Re-open the snow/`RM` arc under the restored burden of proof.** The open
   `OPENWEPP-DEFECTIVE` verdicts across HPHYS0298→0313 are re-classified under
   §3–§4. The real residuals, including `Total-Soil` over-drainage and corrected
   depth-vs-depth snow/`RM` surfaces, are re-localized from unit-consistent
   ledgers. This rejects the stale water-equivalent `hrsnow` route and the
   misattributed snow-state/`driftg` route; it does not preclude a corrected
   hourly snowfall input/phase-partition investigation where HPHYS0299 or later
   evidence still supports it.

## Consequences

- Acceptance authority returns cleanly to ADR-0011 + SC-* contracts +
  conservation/physics-law suites; comparator agreement is demoted to a flag. The
  "cannot reach 39/39 without reintroducing a legacy bug" bind that ADR-0016
  dissolved stays dissolved — because matching the baseline is no longer the bar.
- The 16-package/0-edit arc is reframed: not a failure to fix openWEPP, but
  largely a harness/premise artifact plus a genuine downstream residual that was
  mislocalized. Re-classification work is required; some `OPENWEPP-DEFECTIVE`
  verdicts are expected to become `HARNESS-SURFACE-MISMATCH` or `UNRESOLVED`.
- The paired-lineage harness gains a dimensional/lineage gate; comparator packages
  gain a symmetric verdict schema and a retraction obligation.
- Does **not** change: the negative-melt collapse, the 260430 lineage anchor
  (ADR-0012), the fixed comparator identity, or SC-* governance. openWEPP's
  negative-melt implementation remains matched on both sides.

## Citations

- ADR-0016 (intent and preconditions, esp. #8).
- ADR-0011 (confidence-tiered comparator evidence; correctness authority is the
  contract, not the binary).
- HPHYS0298 unit-artifact review, HPHYS0298 superseding disposition, HPHYS0299
  corrected depth-vs-depth disposition, and HPHYS0313 misattribution review
  (paths above).
