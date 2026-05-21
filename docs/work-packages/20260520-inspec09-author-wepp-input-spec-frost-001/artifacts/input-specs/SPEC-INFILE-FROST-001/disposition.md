# Disposition — SPEC-INFILE-FROST-001

Evidence: Static

| finding_id | source | severity | decision | action_taken | artifact_ref | notes |
|---|---|---|---|---|---|---|
| `FROST-A-001` | `review_agent_a.md` | high | amend | Added explicit strict/compat typed outcomes for all line-2 malformed shapes (missing line, wrong arity, non-numeric token), and updated Case C policy text. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/frost.spec.md:40` | High-severity parser determinism closure. |
| `FROST-A-002` | `review_agent_a.md` | medium | amend | Added row-level provenance tags in gap/conflict register. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/frost.spec.md:185` | Governance traceability requirement satisfied. |
| `FROST-A-003` | `review_agent_a.md` | low | amend | Reclassified delimiter/comment completeness item from HOLD blocker to non-blocking provenance note (`FROST-NOTE-001`). | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/frost.spec.md:190` | Keeps promotion blockers focused on correctness impact. |
| `FROST-B1` | `review_agent_b.md` | medium | amend | Resolved by same provenance-tagged gap table update as `FROST-A-002`. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/frost.spec.md:185` | Duplicate finding closure. |
| `FROST-B2` | `review_agent_b.md` | medium | amend | Reworded `kfactor(1..3)` field dictionary semantics to runtime class slots and retained unresolved class-mapping conflict in HOLD register. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/frost.spec.md:87` | Removes internal contradiction between field dictionary and gap register. |
| `FROST-B3` | `review_agent_b.md` | medium | amend | Ratified explicit strict/compat typed taxonomy for malformed line-2 and out-of-range numeric handling; closed `FROST-GAP-002` as a resolved policy note. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/frost.spec.md:133` | Mode-gated behavior is now fully executable for both shape and range branches. |

## Unresolved / HOLD blockers
- `FROST-GAP-001`: unresolved canonical mapping of `kfactor(1..3)` class semantics across conflicting legacy/comments (`[DIRECT][E-US-02]`, `[DIRECT][E-WF-01]`, `[DIRECT][E-WF-04]`, `[DIRECT][E-WF-05]`).
- `FROST-GAP-003`: unresolved reject/accept policy for hypothetical version-prefixed variants (`[DIRECT][E-WF-01]`, `[DIRECT][E-US-02]`).
