# Disposition — SPEC-INFILE-TC-001

| finding_id | source | severity | decision | action_taken | artifact_ref | notes |
| --- | --- | --- | --- | --- | --- | --- |
| `TC-A-001` | `review_agent_a.md` | `medium` | `accepted-fixed` | Added explicit row-level provenance tags to the gap/conflict register. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/tc.spec.md:139` | Satisfies provenance-tag requirement for arbitration/verifier traceability. |
| `TC-A-002` | `review_agent_a.md` | `medium` | `accepted-fixed` | Codified strict and compat typed behavior for open-failure in both applicability matrix and typed-expectation table. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/tc.spec.md:39`; `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/tc.spec.md:108` | Compat now explicitly preserves legacy collapsed branch with warning. |
| `TC-A-003` | `review_agent_a.md` | `low` | `amended-fixed` | Reclassified naming/UX governance item from blocking `HOLD` to non-blocking `NOTE`. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/tc.spec.md:144` | Parser-correctness gate is no longer blocked by omni naming debt. |
| `TC-B1` | `review_agent_b.md` | `medium` | `accepted-fixed` | Same closure as `TC-A-001`: provenance-tagged gap rows are now present. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/tc.spec.md:139` | Duplicate finding resolved by shared edit. |
| `TC-B2` | `review_agent_b.md` | `medium` | `accepted-fixed` | Same closure as `TC-A-002`: compat branch explicitly typed for open failure. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/tc.spec.md:39`; `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/tc.spec.md:108` | Duplicate finding resolved by shared edit. |
| `TC-B3` | `review_agent_b.md` | `medium` | `accepted-fixed` | Added interim authority-resolution rule to bound retirement-snapshot evidence as legacy-compat provenance. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/tc.spec.md:30` | Clarifies snapshot-vs-active-source authority semantics. |

## Unresolved HOLD Blockers
- `TC-GAP-001`, `TC-GAP-002`, `TC-GAP-003` remain open in spec Section 10 and continue to justify `draft-HOLD` status.
