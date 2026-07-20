# Independent Review B

Evidence mode: `Static`

Reviewed range: `49ff3138..53f47dba` (withdrawn prototype)

Status: `CHANGES REQUIRED`

Review B was completed independently without Review A's findings.

## Findings

| ID | Severity | Finding | Required disposition |
|---|---|---|---|
| B-01 | high | The machine intent artifact admitted documentation only, so implementation lacked valid pre-edit admission. | Withdraw the prototype, restore the scaffolded state, and generate a corrected intent before reimplementation. |
| B-02 | high | Accepted native forest schedule gaps could skip GSI dates and fail chronology on later reactivation. | Require continuous year-round native phenology scheduling or define an authorized gap evolution; add a multi-year guard test. |
| B-03 | high | Schema/projection/kernel accepted zero `bb` contrary to the contract. | Enforce strict positivity at all native boundaries and test it. |
| B-04 | high | Real-consumer and conservation artifacts overclaimed source-only and execution-only evidence. | Add real-run runtime-value checks for every claimed consumer and revise evidence. |
| B-05 | medium | VPD values down to `-1e-9 kPa` were silently clamped without named contract authority. | Fail every negative derived VPD or ratify an explicit bounded normalization. |
| B-06 | medium | SH and repeated-cycle tests did not meet one-day transformed-calendar and bit-identical endpoint claims. | Strengthen both tests to the stated contract. |
| B-07 | high | Exact terminal receipt failed A0 admission and workspace Clippy; dependent regression, doctest, and CRAP nodes were blocked. | Fix both direct failures and rerun every selected node at the corrected exact head. |

Recommendation: `HOLD` / not closure-eligible until all findings are fixed and
verified.
