# Finding Disposition

Status: `complete`

Evidence class: `Ran + Static`

| Finding | Source | Disposition | Resolution |
| --- | --- | --- | --- |
| WAT `year` was initially treated as calendar year. | Execution incident 001 | accepted | Discarded the unpublished attempt; the executor now binds dates from the protected CLIGEN calendar and checks WAT Julian/day identity. |
| Harvard SWE metadata conflicts with the row-level depth-density identity by approximately tenfold. | Execution inspection; review A/B | accepted | Excluded bound Harvard SWE as `INVALID_SOURCE_UNIT_IDENTITY_CONTRADICTION`; no fixture value or unit was rewritten. |
| Harvard vertical density-profile layers were pooled against modeled bulk snow density. | CAL06-RA-001; CAL06-RB-001 | accepted | The operator now uses HF237-01 daily bulk density only; profile layers are `NOT_EVALUATED_SCALE_MISMATCH`. |
| `CAL06-SNOW-001` used a global bound-snow match count although the cell is Marcell-only. | CAL06-RB-002 | accepted | Scoped the count to Marcell and added a terminal assertion for the exact 31,542 count. |
| Snow and litter/frost figures omitted required explicit distributions/chronologies. | CAL06-RA-003 | accepted | Added peak SWE, peak bulk density, melt-out, residue depth, and explicit frost onset/thaw distributions. |
| Litter/downstream null and advancement labels were hardcoded and not digest-bound to verdict tables. | CAL06-RA-003 | accepted | The plot-only follow-on removes ancillary labels from SVGs. Same-basename Markdown sidecars link the verdict/source tables and retain null/advancement semantics; every SVG embeds the SHA-256 identity of its plotted source table. |
| Ephemeral raw objects did not leave enough retained operands to reconstruct annual/water-year summaries. | CAL06-RA-002 | accepted | Re-executed the full matrix and retained per-run ALL/calendar-year/water-year operands; the validator independently reconstructs every `run-results.csv` numeric summary. |
| Derived ensemble and observation summaries were not independently reconstructed, allowing mutually stale summary/figure pairs. | CAL06-RA-004 | accepted | The terminal validator now rebuilds and checks every lane/group inventory, count, minimum, median, maximum, and score-group verdict from the retained run and score rows. |
| Package-tool line counts were stale after correction tooling was added. | CAL06-RB-003 | accepted | Reconciled all five package-local Python tools; no Rust file changed and the Rust line-count gate remains passed. |
| Terminal review completion | Review A/B | accepted | Both independent reviews finalized `PASS`; all seven named review findings are corrected and no finding remains undispositioned. |
| Ancillary captions and scientific interpretation were embedded inside SVG figures. | User-directed plot/sidecar follow-on | accepted | SVGs now contain plots, axes, legends, and plotted categorical data only. Six same-basename Markdown sidecars carry captions, units, source links, and ancillary scientific boundaries. Both independent follow-on re-reviews pass. |
| The Harvard downstream plot used a generic four-stratum legend that advertised an unavailable conifer lane. | User visual review | accepted | The renderer now accepts a figure-specific legend inventory; the Harvard downstream figure lists only open, deciduous, and mixed. The validator rejects a downstream conifer legend and requires all three plotted strata. |
