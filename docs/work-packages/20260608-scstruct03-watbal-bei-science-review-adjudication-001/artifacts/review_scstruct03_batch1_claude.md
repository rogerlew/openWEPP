# SCSTRUCT03 Batch 1 Review — Claude Code (reviewer)

Reviewer: Claude Code
Subject: Batch 1 (ProfileFC/WP family) — first narrative relocation, stopped at the snow-arc boundary
Evidence mode: **Ran + Static** — read the `SC-WATBAL-001` diff, the batch-1
adjudication / crosswalk / size-delta artifacts, the cited invariants, and the
retained HPARITY02 section; **ran** `--strict` lint.

## Verdict: sound — conservation verified, method validated

This is the first batch to relocate narrative out of the core, so it carried the
highest drop-an-obligation risk. It holds up.

### Mechanics (Ran/Static)
- `SC-WATBAL-001` diff: **5 ins / 103 del** — five ProfileFC/WP sections removed,
  five BEI rows updated. Invariant table untouched (no `INV-*`/`OBL-*` row
  deleted); INV-041/042 still present.
- Sidecar grew from a 16-line stub to 200 lines holding the relocated narrative.
- `--strict` lint: `PASS-DEFERRED … 64 science-review-follow-on rows`, exit 1.
  Deferred count fell exactly **69 → 64** (5 resolved); 75 total rows conserved.
- Core size: **−4,925 bytes / ~−579 tokens** (batch-1 size-delta artifact).

### Conservation argument (verified, not just asserted)
The live ProfileFC/WP obligation is held in core by three retained authorities:
- **INV-WATBAL-041** — corrected, full-profile-depth, non-truncated seed grid.
- **INV-WATBAL-042** — explicitly names `ProfileFCStore`/`ProfileWPStore`,
  requiring runtime WB11/WB13 storage-lineage traceability and forbidding static
  synthesis.
- **HPARITY02 Profile-Capacity Publication Lineage Closure** — confirmed present
  at core line ~1528, **untouched** by this batch, with guard codes
  `HKERNEL-WB13-HWAT-E-001..003`.
The five relocated sections (HPHYS0202/0205/0206/0216/0216D) are historical
evolution narrative of that rule. INV-042 naming the stores and HPARITY02 owning
the publication lineage make this a genuine semantic mapping, not a token scrape.

### Discipline
Relocated only 5 of the ~7 ProfileFC/WP-area rows — HPHYS0207 and HPHYS0209 were
left routed. Genuine per-row adjudication, not a blanket family call. The stop
before the snow/melt-term arc is correct per ADR-0017 (retired investigation ≠ no
obligation).

## Findings

### B1-F1 — INFO — competence-calibrated soundness caveat
- The mechanics and citations are clean and the cited authorities are topically
  on-point and verified retained. But the final call — that INV-041 + INV-042 +
  HPARITY02 capture **every** live sub-obligation of HPHYS0202/0205/0206 (notably
  the "corrected, not raw, FC/WP layer values" residue) — is a water-balance
  domain judgment at the edge of a non-specialist reviewer's competence.
- Mitigation already in place: the relocation is fully **reversible** (narrative
  is in the sidecar and git; the BEI mapping is explicit). This is exactly the
  low-risk batch chosen to validate the method.
- Recommended: operator/scientist concurrence on the ProfileFC/WP coverage before
  the method is treated as proven for the map/historical-heavy batches.

### B1-F2 — INFO — per-batch yield expectation
- Batch 1 reclaimed ~579 tokens (~1.5% of the ~37.9k-token core). The method
  works, but short historical sections yield little. Meaningful reduction depends
  on the **large** sections (EROD12 ~290 lines, HPHYS0260 ~180, HPHYS0308 ~196)
  resolving as map/historical (which relocate) rather than promote/narrower-HOLD
  (which retain). Net WATBAL shrink is therefore uncertain until the big cohorts
  are adjudicated — worth setting expectations, not a defect.

## Recommendation

**Accept batch 1.** Conservation is verified and the method is sound. The stop is
correct. Two gates before continuing:
1. Operator concurrence on the ProfileFC/WP coverage (B1-F1) to confirm the method.
2. The snow-arc and WB19 cohorts remain genuinely science-gated — proceed only
   with cited authority per row, not momentum.

Findings surface issues + evidence; disposition and the domain calls remain with
Codex and the operator.
