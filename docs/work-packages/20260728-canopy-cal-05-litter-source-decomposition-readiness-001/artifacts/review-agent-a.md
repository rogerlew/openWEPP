# Terminal Scientific Review A — Corrected Results

Status: `PASS`

Evidence class: `Static: complete corrected package/result/tool review; Ran:
read-only terminal validator and independent arithmetic checks`

## Finding closure

### TRA-001 — `CORRECTED`

The producer now retains raw forcing, surface/interrill/rill seeds, all frozen
decomposition factors and resulting pools, root/depth state, decomposition
publication, and residue-partition consumption. Interrill and rill state now
carry across every day and year as frozen.

The independent reconstructor derives temperature, standing-water,
flat-water, environmental-index, and decay factors from retained inputs and
checks the complete retained state/publication vector with first-divergence
reporting. All 16 candidates contain 7,300 ordered rows, report
`first_divergence=NONE`, and have maximum differences no larger than
`8.89e-16`, well inside `1e-12`. The strengthened validator also checks design
membership, chronology, source day, rate, role, raw forcing, all three carried
seeds, downstream publication, and partition consumption.

### TRA-002 — `CORRECTED`

Incident 002 records the method and unit correction. Local source sensitivity
uses the central difference around `S020` from `S010/S030`; local rate
sensitivity uses the central difference around `K050` from `K000/K100`. All
points and symmetric steps were in the prospectively frozen grid. The method
does not widen an axis, select a parameter, or change a model result.

Independent recomputation exactly reproduced all eight reported derivatives,
the five-pair ridge covariance `0.16261898422070004 kg m^-2 yr^-2`, and ridge
correlation `0.9993772023530584`. Sensitivity is nonzero in every reported
slice. The readiness matrix now separately records local sensitivity,
covariance/correlation, saturation, and equifinality. It also correctly states
that the ridge statistics describe confounding, not a probability model, and
that saturated water factors do not control the interior
`0.4976215945544323` temperature-limited decay modifier.

Within this narrow named source/rate direct-runtime operator,
`CALIBRATION_READY_DATA_LIMITED / PARTIALLY_IDENTIFIABLE` is supported. It does
not transfer to native source composition, identify an empirical decay value,
or lift the stock/material authority block.

### TRA-003 — `CORRECTED`

Every Harvard diagnostic now retains its unique project/plot key, flux period,
stock year, litter-row count, stock-replicate count, `stock_use_not=1`,
unit-bearing value fields, and the
`DESCRIPTIVE_NONCONTEMPORANEOUS_POOLED` interpretation. The validator rebinds
each row to the admitted join and checks the periods, counts, use flag, and
exact four source values.

Independent review confirmed 28 unique keys, exact source-value identity, and
zero discrepancy in the pooled-nonfoliar shares and descriptive flux/stock
ratios. The `1e-7 g C m^-2 yr^-1` Incident 001 arithmetic guard remains a
serialization tolerance only. No material allocation, carbon-to-dry-mass
conversion, modeled-pool equivalence, contemporaneous-turnover claim, or
empirical fit was introduced.

## Scientific disposition

The following claims remain correct:

- native leaf transfer is implemented, but frozen CAL-04B member-level native
  litter traces are unavailable, so leaf-source sufficiency remains
  `NOT_CALIBRATION_READY / NOT_ASSESSED`;
- recurring needle and fine-woody sources remain
  `AUTHORITY_MISSING / NOT_CALIBRATION_READY / NOT_ASSESSED`;
- complete daily synthetic stock recovers `S020-K050`, while a single
  year-20 endpoint is nonidentifying across the exact five-pair ridge;
- all prescribed valid-zero and invalid cases retain their expected typed
  dispositions; and
- the contract-first `CANOPY-LITTER-SOURCE-AUTHORITY-01` successor is bounded
  to determining configuration need, authority, and implementation before any
  new source law. It invents no magnitude, tissue allocation, recurrence,
  carbon conversion, or decay adjustment.

## Verdict

`PASS`. TRA-001 through TRA-003 are corrected. CAL-05 supports the named
direct-runtime data-limited readiness claim and bounded missing-physics
handoff, while empirical source composition and decomposition fitting remain
authority-blocked. This review does not replace the required independent
terminal review, verification, exact-diff reconciliation, or final gates.
