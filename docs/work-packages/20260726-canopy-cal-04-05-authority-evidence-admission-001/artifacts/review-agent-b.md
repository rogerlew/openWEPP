# Independent Review B

Evidence class: `Static source-object inspection plus ran extraction/hash checks`

Status: `HOLD / FINDINGS RETURNED`

The reviewer confirmed the original EML terms, CAL-05 pooled-material
limitations, missing-value handling, Plot 7 basket-loss semantics, conservative
Marcell/Santee exclusions, and the package/source hashes.

Findings:

- `HIGH`: CAL-04 lacked exact native endpoint mappings and aggregation rules;
  unobservable Harvard endpoints could not be scored.
- `MEDIUM`: the plausible 28-plot HF324 litter/stock join lacked a retained
  derived table and deterministic extractor.
- `HIGH`: queued package, gate, review-state, and disposition artifacts had to
  be reconciled before terminal acceptance.

Terminal verdict was `HOLD`. The reviewer agreed that CAL-05 is only partially
lifted and remains blocked for decomposition/source-adequacy claims.
