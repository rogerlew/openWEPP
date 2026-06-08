# SCSTRUCT03 Batch 2 Review — Claude Code (reviewer)

Reviewer: Claude Code
Subject: Batch 2 (snow/melt comparator arc, HPHYS0298–0308) — relocation, stopped before WB13/WB16/WB12
Evidence mode: **Ran + Static** — read the `SC-WATBAL-001` diff, INV-073..081
statements, the batch-2 adjudication/crosswalk/size-delta artifacts; **ran**
`--strict` lint.

## Verdict: sound — the cleanest mapping so far

This was the high-scrutiny batch (snow arc = where comparator surface artifacts
historically produced false root-causes). It is the strongest-evidenced relocation
to date.

### Conservation (Ran/Static)
- Diff: invariant table untouched (no `INV-*`/`OBL-*` row deleted); INV-073..081
  and INV-087 all present in core.
- Nine HPHYS0298–0308 sections relocated to the sidecar; ADR0017 Comparator-Flag
  addendum **retained in core** (line ~368) — no comparator re-tiering.
- `--strict` lint: `PASS-DEFERRED … 56 rows`. Deferred fell **64 → 56** (−8, not
  −9: HPHYS0298 already carried an INV-087 mapping and was never deferred). 75
  rows conserved.
- Core size: batch-2 **−6,319 bytes / −835 tokens**; cumulative (b1+b2)
  **−1,414 tokens** (38,444 → 37,030, ~3.7%).

### Mapping is 1:1 and verifiable
Each relocated section maps to its own dedicated numbered invariant:
INV-073=HPHYS0298, 074=0299, 075=0300, 076=0301, 077=0302, 078=0305, 079=0306,
080=0307, 081=0308 — plus INV-087 (ADR0017 comparator verdict) on the three
comparator-governance rows. This is the soundest possible conservation evidence: a
named invariant per section, not a coverage argument.

### Key insight (validates the framework)
These obligations were **already promoted** to invariants (INV-073..081).
SCSTRUCT02's mechanical pass had false-negatived them as `unpromoted-binding`
because the section *body* lacked the INV string while the invariant lived in the
table. The science review recovered the real mappings rather than inventing them —
the conservative mechanical pass erred safe, exactly as designed.

## Findings

### B2-F1 — INFO — yield outlook revised upward
- If a meaningful share of the remaining 56 deferred rows are likewise
  already-promoted-but-not-string-matched (the WB19 HPHYS0218–0259 family is a
  candidate), consolidation will be more productive than batch 1 implied. Two
  batches have reclaimed ~1,414 tokens (~3.7%) cleanly; the trajectory now looks
  worth continuing, with the genuinely-ambiguous promote-vs-map calls concentrated
  in the WB19 cohort.

### B2-F2 — TRIVIAL — stale artifact header
- `watbal-core-size-delta.md` still says `Status: batch 1 measured` though it now
  contains batch 2. Cosmetic.

## Recommendation

**Accept batch 2.** Conservation is airtight (1:1 named invariants), ADR-0017 is
preserved, reduction is real. The stop before WB13/WB16/WB12 is correct — those
are active closure/publication obligations (Chapter-5 closure + WB13 publication
authority), a different character than retired comparator narrative, and warrant
their own per-row adjudication.

Findings surface issues + evidence; disposition and domain calls remain with Codex
and the operator.
