# MOFE12 H2637 Defect Replication Diagnostic Report

Status: complete
Evidence mode: mixed (Static + Ran)

## Objective
Determine whether openWEPP reproduces the incident defect signature from:
`/home/workdir/wepp-forest/docs/ablation/20260430_uncapped-spectacular_h2637_hillslope_closure-spike/incident.md`.

## Incident Signature (Baseline Authority)
From incident artifacts/logs (`C000_baseline`):
- `day44_hillslope_error_mm_legacy = -180.31779`
- `day45_hillslope_error_mm_legacy = -0.0482`
- dominant OFE spike: `day44_ofe19_err_legacy = -180.4590`

Legacy diagnostic formula (reconstructed and validated):
- `err_legacy = (RM + Irr + UpStrmQ + SubRIn) - (QOFE + latqcc + Ep + Es + Er + Dp + Tile) - Δ(Total-Soil Water)`
- day-44 hillslope error is the sum of OFE-level `err_legacy` values.

## Candidate Execution Context
Ran openWEPP on staged H2637 inputs with compatibility policy, using:
- temp soil token normalization to satisfy current parser tokenization for this
  legacy 9002 file shape.
- bounded climate horizon (`p2637_60d.cli`) to capture day-44 behavior in a
  tractable execution lane.

Candidate publication geometry (manifest):
- `publication_ofe_policy = single-row-canonicalized-hillslope-aggregate`
- `contributor_ofe_count = 19`
- published rows carry only `OFE=1` keys.

## Candidate Results (Observed Surface)
Applying the same formula to the published candidate row stream:
- `day44_err_legacy = -194.75053419004115`
- `day45_err_legacy = -215.80856517617104`
- first-60-day residual band remains persistently large negative
  (`min=-229.8203322371234`, `max=-168.6910123056498`), with no isolated day-44
  spike pattern.

## Replication Verdict
Primary verdict: **indeterminate for strict incident signature replication**.

Reason:
- The incident signature is OFE-resolved (including `OFE19`) and hillslope-sum
  based.
- openWEPP WB13 publication currently canonicalizes to one aggregate row/day
  (`OFE=1`), so `OFE19` defect observability is not available on this surface.

Secondary observed-surface verdict:
- **not replicated on published aggregate signature shape**.
- Baseline has day-44 spike + near-zero day-45; candidate published-row signal
  shows persistent large negatives across neighboring days, not the same
  day-44/OFE19 defect fingerprint.

## Constraints and Caveats
- Full 34-year lane did not finish within the bounded execution window used in
  this package.
- Soil token normalization and climate clipping were applied only in `/tmp` for
  diagnostics; repo source/contracts were not changed.
- Candidate signal interpretation is constrained by WB13 aggregate publication
  policy.

## Recommended Follow-On
1. Add a diagnostics-authorized OFE-resolved publication mode (or equivalent
   replay extraction surface) for targeted defect replication/attribution.
2. Re-run H2637 on full-horizon climate with OFE-resolved output and apply the
   same `err_legacy` metric directly to OFE19 + hillslope sum.
