# Finding Disposition

Status: `complete`

Evidence mode: `Static + Ran`

| Finding | Disposition | Correction and rationale |
| --- | --- | --- |
| A-01: active immediate-residue fields omitted | `accepted` with one factual refinement | Added `cf` and `diam` to the public guide and ledger, plus exact `initial_conditions[].inrcov`, `.rilcov`, `.sumsrm`, and derived runtime ratio lineage. Direct code inspection shows `diam` is passed to the helper but native forest `landuse=3` selects the fixed non-cropland factor; the guide therefore records current branch inactivity rather than the review's proposed positive sensitivity. |
| A-02: ledger lacks equation/effect columns | `accepted` | Rebuilt the ledger with explicit user label, equation location, effect direction, scale, minimum observations, and all original authority/calibration fields for 21 rows. |
| A-03: decomposition primer too thin | `accepted` | Added the surface/root first-order recurrence, temperature/moisture modulation, exponential cover, and separately derived depth chain. |
| A-04: fallback lacks reader-facing source | `accepted` | Added Olson (1963), Qualls (2016), a bounded citation statement, and an internal claim-map row. The exact 0.5 yr^-1 value remains labeled a contract-authorized fallback, not a literature-calibrated universal constant. |
| A-05: Lim reference shorthand | `accepted` | Replaced bibliography `et al.` with the published 19-author list. |
| B-01: public and ledger per-field contract incomplete | `accepted` | Added a public equation/minimum-evidence map, explicit required-input/default posture, and the expanded auditable ledger. |
| B-02: derived depth placeholder not actionable | `accepted` | Replaced the pseudo-field with `derived runtime residue_depth_conversion_m_per_kg_m2` and named every active initial-state/residue input and derivation. |
| B-03: OFE and `not_represented` undefined | `accepted` | Expanded overland-flow element at first use and identified `not_represented` as a YAML status token. |
| B-04: Lim reference shorthand | `accepted` | Same correction as A-05. |

No finding is deferred or assigned to follow-up. Both independent
verifications pass every accepted correction and report no remaining finding.
