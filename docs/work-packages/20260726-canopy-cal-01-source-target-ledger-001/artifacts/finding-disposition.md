# Finding Disposition

## Reviewer A

| Finding | Disposition | Evidence |
| --- | --- | --- |
| Missing Gresham material rows | `FIXED` | `target-ledger.csv` contains aggregate, needle, foliage, and combined wood/fruit rows with standard errors. |
| Hubbard LAI overclassified | `FIXED` | `HB-FIELD-003` is `CONTEXT_ONLY`; the primary-source range remains separate. |
| Gosz/Coates measurement basis | `FIXED` | Ledger notes and comparison rules distinguish loss-on-ignition organic mass from oven-dry bulk mass. |
| Aggregate SE and printed-page details | `FIXED` | Aggregate SEs are `0.501` and `0.217 Mg/ha/yr`; all Gresham table locators use printed p.227. |

Reviewer A independently confirmed `PASS`.

## Reviewer B

| Finding | Disposition | Evidence |
| --- | --- | --- |
| Bounded substitutes unspecified | `FIXED` | Admission prescribes one exact fixture, required files, allowed transformations, arms, period, hash rule, and claim limits. |
| Typed ledger incomplete | `FIXED` | CSV contains 140 unique rows including shared management operands, reported means, and all 60 return-period values. |
| Third-party redistribution unresolved | `SUPERSEDED BY OPERATOR CONFIRMATION` | Review correctly held publication; the operator later explicitly confirmed redistribution permission for the retained set and directed commit/push. |
| Hubbard LAI overclassified | `FIXED` | Same disposition as Reviewer A. |
| Deciduous baseline mislabeled constant cover | `FIXED` | Admission now calls `p10.man` a seasonal-deciduous fixture baseline and blocks all constant-cover claims/arms absent a superseding record. |
| WEPPcloud return values crossed lateral-flow boundary | `FIXED` | All 12 rows are `daily_hill_streamflow_return_level` with hill-streamflow-including-lateral-flow boundaries. |
| Stale gate record | `FIXED` | Final selected gates cover 140 rows and 23 Markdown files plus hashes, LFS, PDF readability, credentials, placeholders, and diff hygiene. |

No review finding is waived as scientifically immaterial. The review-time
publication hold is preserved in the review artifacts and superseded by the
operator authority recorded in `publication-rights-register.md`.
