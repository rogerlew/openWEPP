# Claude Code Review — HPHYS0298 `hrsnow` Verdict Is a Unit Artifact

Status: complete

Reviewer: Claude Code (retrospective re-review)

Evidence mode: ran (ledger values + ratios) + static (cross-artifact, source)

> This artifact **corrects an earlier Claude review in this same package**
> (`claude-code-review-findings.md`, "STRONG APPROVE / the arc converged"). That
> review was wrong on the central point. Findings and evidence only; contract,
> disposition, and ADR action are Codex's and the deciders' to make.

## Bottom line

HPHYS0298's headline verdict — all nine H1/H7/H39 windows `OPENWEPP-DEFECTIVE`,
first divergent symbol `hrsnow`, "openWEPP produces ~10% of baseline hourly
snowfall," porting-fidelity defect at `winter.for:410-412` — is a
**depth-vs-water-equivalent unit mismatch in the comparator**, not an openWEPP
defect. The "~10%" is the fresh-snow density factor, not a partition error.

## Evidence

The ledger's own `source_provenance` records the two surfaces it paired:

| field | value |
|---|---|
| `canonical_symbol` | `hrsnow` (baseline `winter.for:412`) |
| `openwepp_symbol` | `snow_hourly_snowfall_**water_equiv**_sum_m` (`mod.rs:4606`) |

- Baseline `hrsnow` is snow **depth** — `snowd.for` adds it straight onto depth:
  `snodep = snodpt + hrsnow`.
- The paired openWEPP symbol is, by its own name, **water equivalent**.

Ran (ratios from `paired-lineage-ledger.json`):

| baseline (mm) | openWEPP (mm) | ratio |
|---:|---:|---:|
| 16.65 | 1.665 | **10.0000** |
| 68.569 | 6.85697 | 9.99985 |
| 71.5815 | 7.15822 | 10.0000 |
| 81.2218 | 8.289211 | 9.798 |

The ratio is the **fresh-snow density factor** (fresh snow ≈ 100 kg/m³ vs water
1000 kg/m³ ⇒ ~10× depth per unit water). The `16.65 → 1.665` case is exactly
×10 (constant default density); the others drift off 10 only where density
varies. This is a unit conversion, not "openWEPP partitions 90% of snow as
rain."

## Why openWEPP is (almost certainly) fine on this surface

Cross-check against HPHYS0313, which compared snowpack **depth** directly:

- openWEPP pack depth tracks baseline pack depth to ~`1e-6 m`
  (`0.2326628` vs `0.2326631` at 2013 d11 h10).

If openWEPP were feeding water-equivalent into `snodep = snodpt + hrsnow`, its
pack would be ~10× too shallow and could never match. It matches. So openWEPP
handles snow depth correctly internally; HPHYS0298 grabbed openWEPP's
water-equivalent *accounting* surface and compared it to baseline *depth*. This
is a comparator-harness mis-pairing.

*(Static, cross-artifact — the kernel was not re-run to confirm.)*

## Why this matters beyond one package

1. **It seeded the arc and still stands uncorrected in-package.** 0298 declared
   "the arc converged" and pinned the root cause to `hrsnow`, recommending a
   baseline-authoritative winter snow/rain forcing migration. HPHYS0299 *did*
   catch the unit artifact and reclassified 8/9 windows off `hourly-forcing` —
   but 0298's committed `STRONG APPROVE` review and `OPENWEPP-DEFECTIVE @
   hrsnow` disposition were never retracted, and the arc kept hunting an
   openWEPP snow-*state* defect (melt terms → branch predicates → carry depletion
   → prior-day/prior-year carry → settling/`driftg`) for 15 more packages,
   closing **0 production physics edits** across all 16. HPHYS0313 then
   re-committed the *same* hrsnow surface-artifact class 15 packages later — the
   paired-lineage harness still has no dimensional guard.

2. **The check that catches it was named and waived.** Finding
   `CLAUDE-0298-001` wrote: "Full independent physical authority ... would make
   it airtight, but is **not required to act here**." A dimensional / physics
   check is exactly what distinguishes a unit mismatch from a partition defect.
   The `water_equiv` field name was in the provenance the entire time.

3. **It propagated into a ratified ADR.** ADR-0016 simultaneously cites "the
   comparator-surface artifacts from HPHYS0298 (`hrsnow`) ... compounded the
   confusion" *and* the program treated 0298's verdict as settled root cause.
   That contradiction was never resolved; the arc continued on the "defect"
   reading.

## The residual is real — it was mislocalized, not invented

The `Total-Soil Δ` of `-1546.726174 mm` (H1 spring-2014) and the `RM` deltas
(`176.29` vs `161.62`) are genuine divergences. Something downstream *is* wrong.
0298's error is not "there is no residual"; it is pinning the **first divergent
cut-point** to `hrsnow` via the unit artifact, which steered 16 packages into
the snow-forcing subsystem instead of the over-drainage / water-balance surface
that prior work (`project_waterbalance_overdrainage_residual`) already
implicates.

## Suggested actions (Codex / deciders)

1. Re-pair `hrsnow` like-for-like: compare baseline depth against an openWEPP
   **depth** surface, or convert the openWEPP water-equiv sum by fresh-snow
   density before differencing. Re-run the 0298 first-cut-point classification.
2. Retract or annotate the 0298 `OPENWEPP-DEFECTIVE @ hrsnow` verdict and the
   "winter forcing migration" follow-on recommendation.
3. Treat this as the factual basis for superseding/scoping ADR-0016: its
   "openWEPP has actual defects" premise rested on this mis-paired surface.
4. Re-localize the real residual starting from the `Total-Soil` / drainage
   surface rather than re-entering the snow-forcing chain.
5. Add a standing dimensional/unit guard to the paired-lineage harness so a
   depth-vs-water-equiv pairing fails closed instead of producing a `file:line`
   verdict.
