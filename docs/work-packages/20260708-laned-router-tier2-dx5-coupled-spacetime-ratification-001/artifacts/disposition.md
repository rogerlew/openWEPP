# Disposition

Status: `EXECUTED-HOLD-DX5-UNRATIFIED`
Evidence mode: Ran.

## Decision

Hold. Do not promote `dx5` as the active production mesh default.

## Rationale

The selected real-cohort coupled ladder completed without active runtime
failures, and `dx5` candidate-vs-reference comparisons pass production
tolerance class at both production and refined max-substep settings. However,
the fine-reference adequacy basis remains open:

- `mn_corn_h4` production-cap shape one-third miss:
  `0.020180511 > 0.016666667`.
- `wa_cascades_forest_h1` refined-75 annual pass-sediment one-third miss:
  `0.022131684 > 0.0066666667`.

Per `SC-OFEROUTE-001` rev 43, a target-`dx` production default change requires
candidate-vs-adequate-fine-reference evidence. That precondition is unmet.

## Disposition

- `SC-OFEROUTE-001`: no amendment.
- Active production mesh default: unchanged.
- Shadow mesh: unchanged and out of scope.
- Runtime cost: recorded, not blocking.
- Follow-on: WA annual pass-sediment fine-reference adequacy attribution.

## Review Finding Disposition

| Finding | Disposition | Evidence |
|---|---|---|
| A-HIGH / B-HIGH stale gate and line-count artifacts | accepted, fixed | `gate-results.md` now classifies every gate as `PASS`, `FAIL`, or `NOT RUN`; `line-count-governance.md` is final `PASS` |
| B-HIGH missing review/verification artifacts | accepted, fixed | Dual reviews are present; dual verification is being completed after review-response fixes |
| B-MEDIUM timestep-control analyzer semantics | accepted, fixed | `analyze_coupled_spacetime.py` now treats `timestep_control_dx5`, `timestep_control_dx2p5`, and `timestep_control_dx1p25` as gate-class comparisons; replay preserved the same `EXECUTED-HOLD-DX5-UNRATIFIED` verdict |

No finding rejects the technical hold/no-flip disposition.
