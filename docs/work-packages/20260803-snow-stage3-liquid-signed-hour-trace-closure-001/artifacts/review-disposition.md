# Review Disposition

Status: `complete / all findings remediated`

Evidence mode: `Static + Ran`

| Finding | Disposition | Resolution |
|---|---|---|
| Post-v4 evidence predated diagnostic-carrier refactor | `accepted / remediated` | Rebuilt release binary and ran fresh non-overwriting `post_v4_terminal`; canonical report binds binary `464c87e1...`. |
| WAT-only protected-output predicate omitted HBP/PASS | `accepted / remediated` | Independent verifier now requires, hashes, reports, and gates both WAT and HBP/PASS byte identity. |
| Mixed-sign rows were not required to overlap Stage-3 operands | `accepted / remediated` | Pass predicate requires all four individual mixed-row counts and a joint all-nonzero population; `227` joint rows observed. |
| Active/lower arrays mislabeled end-of-hour | `accepted / remediated` | Contract and lineage now bind duration-weighted substep semantics. |
| Lower-state full-hour weighting lacked presence interpretation | `accepted / remediated` | Additive v4 publishes exact existing `lower_layer_present_fraction`; contract and tests require it. |
| Disabled-row semantics and v4 size cost were implicit | `accepted / remediated` | Verifier requires zero disabled-row liquid violations and reports `1.7311x` trace growth. |
| Contract edit left the generated assurance identity stale | `accepted / remediated` | The package write set was amended, then the typed `adopt-report-source` transaction generated receipt `ac9ae76f...`; no active authority was invalidated and assurance validation passes. |

No finding was rejected, deferred, or left open. Both reviewers returned fresh
`PASS` after remediation.
