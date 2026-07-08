# Hold Legitimacy Audit

Evidence mode: Ran.

## Hold

Status: `EXECUTED-HOLD-SEDIMENT-METRIC-AUTHORITY`.

The `dx5` production mesh-policy promotion remains blocked because
`wa_cascades_forest_h1` refined-75 fine-reference annual pass-sediment adequacy
still fails the current `SC-OFEROUTE-001` rev-43 judged-surface rule:

- Surface: `tdep:4`
- Candidate/reference: `dx2p5_dt75` versus `dx1p25_dt75`
- Relative delta: `0.022131683796129127`
- One-third adequacy threshold: `0.006666666666666667`

## Evidence Proving the Blocker

Replay artifact:
`artifacts/wa-sediment-attribution.json`.

The replay confirms the prior comparator value from the pass parquets and
localizes it to one daily row:

- Year-4 candidate `tdep`: `0.6107069659777166 kg`
- Year-4 reference `tdep`: `0.5974836468326581 kg`
- Absolute delta: `0.013223319145058476 kg`
- Only nonzero daily `tdep` delta: sim day `1126`

## Why This Cannot Close In-Envelope

The package objective was attribution, not a production flip or tolerance
rewrite. The evidence supports a narrow classification: annual sediment
response to a sub-threshold routed-hydrograph shape perturbation. That evidence
does not by itself authorize replacing the existing annual sediment adequacy
rule:

- No alternate annual pass-sediment metric was predeclared in
  `SC-OFEROUTE-001`.
- A low-mass absolute floor, event-conditioned rule, or mass-weighted annual
  rule would change the mesh-policy acceptance surface and must be
  contract-first.
- Widening the existing threshold from the failing value would be
  tolerance-fitting and is explicitly forbidden by the handoff.

## In-Envelope Routes Considered

1. Promote `dx5` anyway because routed-water surfaces passed.
   - Rejected: annual pass-sediment is a named rev-43 judged surface.
2. Amend the threshold directly in this package.
   - Rejected: no predeclared authority or review for a replacement metric.
3. Classify as active-router numerics and scaffold another router correction.
   - Rejected: the implicated-day trace has zero clamp, zero tail fold, zero
     source delta, and no shape fallback; run-level counters do not increase
     between candidate and reference, and clamp remains roundoff-scale.

## First Actionable Follow-On

Scaffold a contract-first annual pass-sediment adequacy metric authority
package. Suggested package:

`20260708-laned-router-annual-sediment-adequacy-metric-authority-001`

First action:

- Amend or reaffirm the `SC-OFEROUTE-001` mesh-policy annual pass-sediment
  adequacy surface with explicit rationale over the WA day-1126 evidence.
- Candidate alternatives to adjudicate, without fitting the current miss:
  - retain strict relative annual sediment and leave `dx5` unpromoted,
  - add a predeclared absolute low-mass floor,
  - use a mass-weighted annual sediment adequacy surface,
  - use an event-conditioned annual sediment rule tied to erosive-day count.
- Rerun the selected cohort mesh-policy adjudication only after the metric
  authority is decided.
