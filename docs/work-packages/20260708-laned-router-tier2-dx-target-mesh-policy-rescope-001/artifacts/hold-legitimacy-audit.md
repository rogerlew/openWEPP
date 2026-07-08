# Hold Legitimacy Audit

Status: EXECUTED-HOLD-DX-REFERENCE-ADEQUACY
Evidence mode: Static + Ran.

## Hold Condition

`EXECUTED-HOLD-DX-REFERENCE-ADEQUACY`

Fine-reference adequacy and real-cohort target-`dx` evidence did not satisfy
the package's required tolerance surface. The package cannot promote a
production target-`dx` mesh policy.

## Evidence

- `wa_cascades_forest_h1` `dx2p5` fails active day cascade closure at day 1122:
  residual `-0.0001100301742553711 m3`, relative
  `2.2504181899264942e-8 > 1e-9`.
- `wa_cascades_forest_h1` `dx1p25` fails active day cascade closure at day
  1122: residual `0.000011086463928222656 m3`, relative
  `2.2674852834578698e-9 > 1e-9`.
- H2637 remains synthetic stress evidence and fails the shape/sediment
  adequacy surfaces.
- `mn_corn_h4` and `n_idaho_forest_h1` pass their declared comparisons, but
  they do not override the real-cohort WA failure.

Exact adjudication evidence is in:
- `artifacts/mesh-fidelity-adjudication.md`
- `artifacts/selected-cohort-mesh-timing.md`
- `artifacts/mesh-ladder-summary.md`
- `artifacts/mesh-ladder-summary.json`

## Why Not Closed In-Envelope

The next step is an attribution investigation, not a safe production policy
change. The failing rungs are the same fine-reference rungs that the package
requires before accepting any target-`dx` promotion, and the WA `dx10/dx5`
candidate behavior shows large storage/outlet/clamp effects that need a
narrow numerics attribution before changing policy.

## First Actionable Follow-On

Scaffold `20260708-laned-router-wa-day1122-high-resolution-closure-investigation-001`.

Initial action:
- Re-run only `wa_cascades_forest_h1` day-1122-focused diagnostics at `dx2p5`
  and `dx1p25`.
- Attribute whether the closure failure is roundoff amplification, TVD
  high-resolution stress, route-coefficient/geometry magnitude, or a defect in
  active ledger booking.
- Keep production active mesh fixed `10 cells/OFE` unless a later
  contract-backed package passes candidate-vs-adequate-reference evidence.
