# Mesh-Policy Adjudication

Status: `EXECUTED-HOLD-DX5-PROMOTION-UNRATIFIED`
Evidence mode: Ran.

## Decision

Do not promote a production target-`dx` mesh policy in this package.

Operational default remains fixed `10 cells/OFE`, but this package does not
ratify fixed10 as fidelity-adequate. The rev-41 evidence shows fixed10 is
cheap and currently operational, while `dx5` has the best provisional
candidate-vs-`dx2p5` deltas among tested target-`dx` rungs. That comparison is
not promotional because the fine-reference adequacy gate remains unclosed.

## Fine-Reference Adequacy

Strict one-third adequacy result:

| Member | Verdict | Blocking surface |
|--------|---------|------------------|
| `h2637` | PASS | n/a |
| `mn_corn_h4` | FAIL | shape max L1 `0.02018051100943346 > 0.0166667` |
| `n_idaho_forest_h1` | PASS | n/a |
| `wa_cascades_forest_h1` | PASS | n/a |

The old WA fine-reference closure blocker is lifted. The remaining
fine-reference blocker is the `mn_corn_h4` routed-hourly-shape adequacy delta.

## Candidate Verdicts

Real selected-cohort candidate comparisons against `dx2p5`, treated as
provisional because the reference basis is not fully adequate:

| Candidate | Provisional comparison verdict | Blocking surface |
|-----------|-------------------------------|------------------|
| fixed `10 cells/OFE` | FAIL | WA annual sediment max relative `0.05799962982635423` on `tdep:4` |
| `dx20` | FAIL | WA annual sediment max relative `0.05799962982635423` on `tdep:4` |
| `dx10` | FAIL | WA annual sediment max relative `0.02189085377404068` on `tdep:3` |
| `dx5` | CONDITIONAL-PASS | none on the candidate table; not promotional until the fine reference is adequate |

H2637 is synthetic stress only and no longer blocks the rev-41 decision
surface; its recorded candidate deltas are informational and carry no
fleet-general authority.

## Cost Posture

`dx5` costs `84.70 s` aggregate real-cohort user time versus `17.46 s` for
fixed10, about `4.85x`. The cost is concentrated in long forest members:

- `n_idaho_forest_h1`: fixed10 `0.96 s`; `dx5` `21.17 s`.
- `wa_cascades_forest_h1`: fixed10 `15.93 s`; `dx5` `62.79 s`.

Because `dx5` is the only tested target-`dx` rung without a provisional
candidate-table blocker and carries this runtime cost, production promotion
needs a follow-on cost/fidelity decision or optimization package after the
fine-reference adequacy blocker is closed.

## Final Verdict

`EXECUTED-HOLD-DX5-PROMOTION-UNRATIFIED`.

The package lifts the WA rev-39 reference-run closure blocker and narrows the
observed target-`dx` search toward `dx5`, but it cannot safely flip production
because:

- the predeclared fine-reference adequacy rule is still not closed for
  `mn_corn_h4` shape;
- fixed10/`dx20`/`dx10` fail WA annual sediment against the provisional
  `dx2p5` comparison surface;
- `dx5` has a large runtime cost and no production-activation cost
  ratification.
