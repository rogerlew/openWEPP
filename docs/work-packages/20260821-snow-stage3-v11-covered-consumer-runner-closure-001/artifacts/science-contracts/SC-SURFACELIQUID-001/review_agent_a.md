# Independent contract review A

Static: Reviewed `SC-SURFACELIQUID-001@8` at candidate commit
`89b1e288df9d5beaf4e17f215ab5416acc6f95ed` without access to Agent B's
findings.

Ran: strict Binding Exposure lint passed with two rows; focused contract-derived
suite passed 11/11.

| Finding | Severity | Finding and impact | Recommendation |
|---|---|---|---|
| `A-001` | Critical | The all-OFE/all-owner scope conflicted with a deferred unkeyed multi-lane Stage-3 energy ledger. Promotion could authorize incompatible OFE-ground aggregation. | Close with a lane-keyed ledger and two-lane fixture, or add an exact single-lane guard. |
| `A-002` | High | `last_reviewed` and closed/reviewed gap language outran the incomplete v8 review and verification cycle. | Use pending/candidate posture until verification and promotion. |
| `A-003` | High | Proposed/accepted support rules lacked exact `SC-COUPLEDTIME-001` and Stage-3 cadence anchors. | Cite the governing invariant IDs. |
| `A-004` | High | Calibration/readiness dispositions did not use the required `PASS/BLOCKED/NOT_APPLICABLE` schema. | Supply the complete ADR-0042 matrix and clarify non-calibratable custody. |
| `A-005` | Medium | Producer/consumer obligations preceded invariants, and active details followed the gap register. | Restore required core order and eliminate parallel normative authority. |
| `A-006` | Medium | OFE/lane/config/model/K/psi/storage identities lacked complete aliases, units, and provenance. | Add exact runtime names, units, digest ownership, and parameter authority. |

Final recommendation at the review candidate: `HOLD` pending amendments and
independent verification.
