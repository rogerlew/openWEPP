# Review Disposition

Status: EXECUTED
Evidence mode: Static + Ran.

## Faraday Review

Review artifact: `artifacts/review-faraday.md`.

| Finding | Disposition | Fix / rationale |
|---|---|---|
| Medium: final TVD positivity scaling was contract-bound but lacked direct branch evidence. | Accepted. | Added private helper `tvd_positivity_scale` and focused test `final_tvd_scaling_preserves_positivity_and_total`. The test forces `tvd_scale = 0.5` on a telescoping TVD correction, verifies non-negative committed depths, and verifies exact total preservation. Reran focused tests: branch tests `2/2`; D10B/Case-4 focused gate `19/19`; full workspace `1420/1420`. |

## Hooke QA Review

Review artifact: `artifacts/review-hooke.md`.

| Finding | Disposition | Fix / rationale |
|---|---|---|
| High: package closure artifacts incomplete. | Accepted. | Added disposition, verification, worker handoff, final disposition, and final gate/results updates. |
| Medium: package and catalog statuses stale/inconsistent. | Accepted. | Updated `package.md`, package artifact README, `gate-results.md`, and `docs/work-packages/README.md` to `EXECUTED-COMPLETE`. |
| Medium: BEI gate used non-standard `PASS-DEFERRED` closure status. | Accepted. | Gate table now classifies the criterion as `PASS` and records the checker output (`PASS-DEFERRED`) as evidence, with rationale that standing BEI consolidation posture is not a current-scope blocker. |
| Low: required-reading map downgraded Tier-2 package context to on-demand. | Accepted. | Required-reading map now records Tier-2 mesh-policy context as read for boundary context. |

## Residual Risks

- This package does not promote target-`dx`. It removes the WA active solver
  clamp blocker that prevented target-`dx` adjudication from producing valid
  WA evidence.
- Active-mode erosion water-magnitude policy and default-promotion gates remain
  outside this package.
