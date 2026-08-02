# Review Agent A

Status: `PASS / PACKAGE HOLD WARRANTED`

Evidence mode: **Ran + Static**.

## Recommendation

The harness-closure diagnosis is reproducible, the albedo withdrawal is
necessary, and routing a bounded EB-04W2B contract/consumer-closure package is
warranted. The independently closing direct-production ledger is admissible as
descriptive calibration evidence. All Reviewer A findings are dispositioned;
W2A may close truthfully as `HOLD / PARTIAL ADMISSION`.

## Terminal Re-review

Evidence mode: **Ran + Static**.

- **RA-1 — RESOLVED.** The adjudicated JSON, CSV, synthesis, lineage,
  disposition, package, catalog, roadmap, and sidecars now publish per-window
  cold-content hits (`8/23`, `16/40`, `0/19`, `12/22`). They preserve the raw
  separate-median flag only with its exact limitation and make no blanket
  deflation claim.
- **RA-2 — RESOLVED.** The adjudicated products enforce negative modeled peak
  offsets, publish Niwot `0/27` and Snowbird `5/16`, and state that the screen
  neither proves nor rules out precipitation error. Independent reconstruction
  from the hash-bound raw JSON reproduces both populations and hit counts.
- **RA-3 — RESOLVED.** Human-facing products consistently identify empirical
  `b + c` versus `a + d` and explicitly prohibit interpreting the grouping as
  a pure turbulent-flux partition. The frozen raw field name remains only as
  provenance.
- **RA-4 — RESOLVED.** All four sidecars now give populations, water-year
  ranges, aggregation, missing-year treatment where applicable, spread limits,
  and interpretation boundaries. Terminal visual inspection confirms gray
  withdrawn harness traces/bars and a fully masked raw-flag panel with no
  leaked tick labels; all four SVGs parse.

The adjudicated machine view nulls every withdrawn harness chronology,
trajectory, and albedo result while binding the immutable raw JSON and CSV by
SHA-256. Independent recomputation confirms those hashes and every admitted
lane count/value. No new blocker was found.

## Ran Evidence

- `run_melt_chronology_diagnostic.py --self-check`: PASS.
- Freeze/tool/receipt identity audit: PASS. Tool SHA-256
  `fa5399db4a6c94e9d8bc1820828d9e7d82898a9493f9ca899b1ccc8eeaf70ad2`
  and freeze SHA-256
  `ff9f4595a640d4e5d33a41141ee842dcc1da2340f532cccc50bb3f9e937ffd73`
  match the receipt and all eight provenance records.
- Prospective-time audit: PASS. Every result-bearing provenance start follows
  the corrected freeze time.
- Retained-identity audit: PASS for 64 recorded provenance, anchor, fixture,
  run-file, and observation identities.
- Receipt inventory: PASS; four lanes by two models, eight unique successful
  cells.
- SVG XML parse and visual inspection: PASS for four figures. Withdrawal
  banners are conspicuous and the direct/harness roles are visually distinct.
- Independent JSON reconstruction confirms maximum direct mass closure
  `2.221e-15 m`, maximum Stage-3 energy residual `6.094e-08 J m^-2`, and
  harness non-closure `0.037672727273-0.070800000000 m`.

## Initial Findings (Dispositioned Above)

### RA-1 — Major — cold-content evidence is over-adjudicated

The artifacts say positive cold content is not a material concurrent-melt
explanation at all four sites. The emitted site flag applies the disjunction
*after* separately taking the median melt fraction and median melt depth. That
operation hides interannual windows that satisfy either prospectively frozen
threshold:

| Lane | Windows | Per-window threshold hits |
|---|---:|---:|
| Mica Creek | 23 | 8 |
| Niwot | 40 | 16 |
| Paradise | 19 | 0 |
| Snowbird | 22 | 12 |

Snowbird is especially important: 12 of 22 windows meet at least one arm of
the disjunctive rule even though the two separately aggregated medians are
`4.23%` and `0.00856 m`. The valid conclusion is that the separately medianed
site screen is false, not that cold-content coincidence is immaterial or
deflated at every site. Paradise is consistently negative; Mica is mostly
negative; Niwot and especially Snowbird retain material interannual
coincidence. None of these associations establishes a causal melt gate.

Revise `scientific-disposition.md`, `scientific-synthesis.md`,
`operand-lineage.md`, the package/catalog/roadmap summaries, and the relevant
figure sidecars to report the distribution and remove the blanket deflation
claim. Preserve the frozen emitted flag as raw evidence if desired, but label
its exact aggregation semantics.

### RA-2 — Major — the late-input implementation exceeds the frozen early-gap scope

The prospective rule is limited to an **early** modeled peak gap. The runner
computes `observed_swe_gain_m` and increments `late_input_window_count` for
every peak window after sorting modeled and observed dates, including zero and
late modeled peaks. It therefore reports 40 Niwot and 22 Snowbird windows,
where the rule-admissible early-gap populations are 27 and 16.

The present Boolean outcome happens to survive a corrected reconstruction:
Niwot has 0 hits among 27 early gaps and Snowbird has 5 among 16. Snowbird's
screen is therefore supported only as a one-sided, interannual screen, not as a
general site-wide diagnosis. A false screen also cannot rule out input error,
because observed SWE gain is net of contemporaneous losses.

Record a corrected rule reconstruction and prevalence in the scientific
artifacts. Do not present the emitted `late_input_window_count` as faithful to
the frozen rule. This correction can be an adjudication of immutable output;
it does not justify mutating the frozen tool or rerunning the withdrawn harness
inside W2A.

### RA-3 — Moderate — `b + c` is mislabeled as turbulent physics

The frozen machine flag calls `bmelt + cmelt` “turbulent empirical terms,” but
the figure and lineage identify `bmelt` as temperature/cloud and `cmelt` as
wind/dew point. The sum is not a pure turbulent-flux partition. Replace human-
facing “turbulent terms dominate” claims with “non-radiative empirical
`b + c` terms exceed empirical `a + d` terms” (or equally explicit language).
The frozen field name may remain for provenance, provided its non-physical
grouping is stated. This affects the snow roadmap and pathways figure as well
as the prose disposition.

### RA-4 — Moderate — figure sidecars do not yet meet the frozen population/uncertainty requirement

The figures are legible and their withdrawal notices are effective, but the
sidecars generally omit lane-specific water-year counts/ranges and do not
state the aggregation uncertainty exposed by RA-1 and RA-2. Add population
counts, median/operator construction, missing-year treatment where applicable,
and the pertinent interannual limitation. Rename the pathways “Turbulent
terms” row per RA-3. The invalid harness values may remain pictured as failure
evidence because the in-figure and sidecar warnings are unambiguous.

### RA-5 — Information — harness mechanism and successor routing are sound

At frozen source HEAD
`6be622ccfbef6bd563228c02d61095b8e05787c8`, the public partition predicate at
`runoff_reconciliation.rs:297-305` ignores typed hourly snowfall when choosing
the inactive path. The inactive outcome at lines 434-485 records no
accumulation, while the bypassed active hourly path at
`infiltration_reconciliation.rs:905-915,1313-1357` would initialize and sum new
snow. The four exact dropped inputs repeat under both model labels. Because
the lost input changes subsequent state, common-mode bias does not rescue the
albedo contrast. Contract reconciliation, a mixed-event zero-pack regression,
daily SWE closure, and proof through the real snowbench consumer are
appropriate EB-04W2B hold-lift conditions.

## Final Review Disposition

`PASS` for truthful closure as `HOLD / PARTIAL ADMISSION`. EB-04W2B is the
correct next prerequisite. Reviewer A supports admission of the closed direct
ledger with the corrected, explicitly interannual claims and supports
withdrawal of every harness chronology/albedo conclusion.
