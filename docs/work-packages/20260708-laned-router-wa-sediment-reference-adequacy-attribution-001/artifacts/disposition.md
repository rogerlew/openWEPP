# Disposition

Evidence mode: Ran.

## Outcome

Status: `EXECUTED-HOLD-SEDIMENT-METRIC-AUTHORITY`.

The package successfully attributed the WA refined-75 annual pass-sediment
fine-reference adequacy miss. It did not promote `dx5`, amend tolerances, or
change production code.

## Findings

1. Confirmed the failing surface as `tdep:4` for
   `wa_cascades_forest_h1` `dx2p5_dt75` versus `dx1p25_dt75`.
2. Recomputed the prior relative delta exactly:
   `0.022131683796129127`.
3. Localized the whole annual miss to sim day `1126`, julian `30`.
4. Rejected active-router numerics as the mechanism because the day-level and
   run-level trace surfaces are clean.
5. Rejected daily water-magnitude drift as the primary mechanism because
   pass-row water magnitude operands are identical on the implicated day and
   the terminal routed outlet delta is sub-0.004 m3 on a 4594 m3 event.
6. Classified the miss as annual sediment response to a sub-threshold
   routed-hydrograph shape perturbation: the routed shape is a real consumed
   water-timing input, but the prior rev-43 routed-water mesh-policy surfaces
   for the same rung pair all passed.

## Contract Posture

`SC-OFEROUTE-001` rev 43 is unchanged. The annual pass-sediment adequacy rule
remains binding, so `dx5` production mesh-policy promotion remains blocked.

The next package must decide the annual pass-sediment adequacy metric
contract-first; this package provides mechanism evidence but does not settle
metric authority.

## Review And Verification

- `review-linnaeus.md`: initial HOLD findings accepted. Classification wording
  was narrowed from erosion-consumer sensitivity to annual sediment response to
  a sub-threshold routed-hydrograph shape perturbation; package-local heuristic
  thresholds were removed from the analyzer; clean-counter wording was narrowed;
  and the follow-on package was named in the human artifact.
- `verification-carson.md`: numeric/provenance PASS. Initial governance HOLD
  due missing `gate-results.md` and review/verification artifacts is resolved
  by this final artifact set and the gate results recorded here.
