# Hold Legitimacy Audit

Status: `EXECUTED-HOLD-DX5-PROMOTION-UNRATIFIED`
Evidence mode: Static + Ran.

## Hold Condition

`EXECUTED-HOLD-DX5-PROMOTION-UNRATIFIED`

The rev-41 full ladder completes and identifies `dx5` as the only tested
target-`dx` rung without a provisional candidate-vs-`dx2p5` table blocker, but
the package cannot promote it.

## Evidence

- Full rev-41 ladder: 24/24 rungs passed runtime execution.
- WA fine-reference rungs now close:
  - `dx2p5`: wall `4:20.62`, user `260.53`, solver steps `8548214`.
  - `dx1p25`: wall `19:14.96`, user `1154.78`, solver steps `17051717`.
- Fine-reference adequacy remains unclosed:
  - `mn_corn_h4` `dx2p5` versus `dx1p25` shape max L1 is
    `0.02018051100943346`, above the one-third threshold `0.0166667`.
- Candidate comparison:
  - fixed10 and `dx20` fail WA annual sediment at `0.05799962982635423`.
  - `dx10` fails WA annual sediment at `0.02189085377404068`.
  - `dx5` has no blocker in the provisional candidate-vs-`dx2p5` table.
- Runtime:
  - fixed10 aggregate real-cohort user time: `17.46 s`.
  - `dx5` aggregate real-cohort user time: `84.70 s` (`4.85x` fixed10).

Exact evidence is in `artifacts/mesh-ladder-summary.md` and
`artifacts/mesh-ladder-summary.json`.

## Why Not Closed In-Envelope

The package cannot change production default to `dx5` because a current-scope
fine-reference adequacy gate remains failed, so the candidate table is
provisional, and `dx5` also has a large unratified runtime cost. Relaxing the
one-third reference rule or treating fixed10 as fidelity-adequate would be a
contract/policy change not supported by this package's predeclared gates.

## First Actionable Follow-On

Scaffold a narrow `dx5` promotion hold-lift package only if the operator wants
to continue target-`dx` promotion:

1. Resolve `mn_corn_h4` fine-reference adequacy by running a further `dx0p625`
   reference or amending the adequacy rule under explicit contract review.
2. Price and optimize the `dx5` cost burden, especially on
   `n_idaho_forest_h1` and `wa_cascades_forest_h1`.
3. Re-adjudicate whether the fidelity improvement over fixed10 justifies a
   production default cost increase.

Until that package closes, keep active production fixed at `10 cells/OFE`.
