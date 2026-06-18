# Backlog - Forest Lateral-Flow Absolute Magnitude Authority

Status: concept (deferred future work; not scheduled)
Date: 2026-06-18
Owner: maintainers
Origin: POST-BASECOND01-H2637-MAGNITUDE-DISPOSITION; FARPOINT01 H2637 71%
magnitude flag resolved as no-defect internally, with absolute magnitude left
as an external-authority `CONTRACT-GAP`.

## What

Define an external-authority suite for absolute forest lateral-flow magnitude on
wet, H2637-like hillslopes. The suite would answer whether an internally closed
and source-intent-correct openWEPP result such as H2637 `runvol_pct_precip =
71.0036550031206` is physically plausible or out of bounds.

## Governing Constraints

- This is not a legacy parity suite. Legacy remains an ADR-0017 flag, not a
  target.
- This is not a defect-closure package until an authority envelope exists and
  identifies an in-envelope contradiction.
- External authority must be explicit: field benchmark data, a validated forest
  hydrology study/model, or a ratified physical envelope with units,
  uncertainty, site conditions, and applicability limits.
- The suite must preserve the existing closure-first ordering: water balance,
  routing, export, and operand closure remain prerequisite gates before
  absolute magnitude is judged.

## Candidate Evidence

- Field-observed runoff and lateral/subsurface flow fractions for wet forest
  hillslopes with comparable precipitation, soil texture, conductivity,
  anisotropy, slope, and profile depth.
- A validated independent forest hydrology model with documented calibration,
  uncertainty, and applicability to H2637-like conditions.
- Literature-derived physical bounds specific enough to classify the H2637
  magnitude without relying on legacy binary behavior.

## Promotion Criteria

Before promotion to a work package:

1. Identify the proposed external authority and its provenance.
2. State the metric to judge, including units and temporal aggregation.
3. Define the acceptance envelope and uncertainty treatment.
4. Map H2637 input conditions to the authority's applicability domain.
5. Define required openWEPP evidence: closure gates, operand reconstruction, and
   no active source-intent defect in the judged path.

## Not This

This backlog item does not authorize:

- changing H2637 `ksat`, anisotropy, `ui_ssh`, or WB19 equations to match
  legacy;
- reopening BASECOND01's vertical `ssc` closure;
- reopening REFINTENT001's `ksatadj` closure for H2637;
- treating the H2637 `71%` value as an active blocker.
