# Mesh-Policy Final Adjudication

Status: `EXECUTED-HOLD-MN-CORN-H4-SHAPE-NONCONVERGED`
Evidence mode: Ran.

## Decision

Do not promote `dx5` as the opt-in active production mesh default in this
package.

The operator's fidelity-first posture removes runtime cost as a promotion
blocker, but the predeclared reference adequacy gate still fails on the
`mn_corn_h4` routed hourly shape surface. Because the evidence did not close
the reference basis, candidate promotion cannot proceed even though the rev-41
package had `dx5` as the best provisional candidate.

## Candidate Consequences

The new `mn_corn_h4` candidate comparison against `dx1p25` would keep `dx5`
inside the production candidate shape tolerance:

| Candidate | Reference | Shape max L1 | Shape > `0.05` | Outlet L1 rel | Annual sed rel | Verdict if reference were adequate |
|---|---|---:|---:|---:|---:|---|
| fixed10 | `dx1p25` | `0.061780185` | `1` | `0.000139554` | `0` | FAIL |
| `dx20` | `dx1p25` | `0.061780185` | `1` | `0.000139554` | `0` | FAIL |
| `dx10` | `dx1p25` | `0.061780185` | `1` | `0.000139554` | `0` | FAIL |
| `dx5` | `dx1p25` | `0.043488592` | `0` | `0.000097541` | `0` | PASS on `mn_corn_h4` candidate surfaces |

This table is informational only because the adequate-reference precondition
does not close. The other real selected-cohort PASS adequacy verdicts from the
rev-41 package stand; no new evidence implicated another member.

## Production Default

The active production mesh default remains fixed `10 cells/OFE`.

`SC-OFEROUTE-001` is not amended and no Rust production policy is changed.
Shadow mesh policy remains unchanged and out of scope; no shadow mesh decision
is needed because the active production promotion did not land.

## Runtime Cost

The rev-41 selected-cohort cost record remains priced evidence:

- `dx5`: `84.70 s` aggregate real-cohort user time.
- fixed10: `17.46 s` aggregate real-cohort user time.
- Ratio: about `4.85x`.

Under the operator posture, this cost would not block a fidelity-backed `dx5`
promotion. The blocker here is strictly the unclosed reference adequacy gate.
