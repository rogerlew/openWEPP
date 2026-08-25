# Terminal chronology coordinated contract finding disposition

Evidence: `Static`

Current disposition: `GO to mandatory gate stage`

| finding_id | source | severity | decision | action_taken | artifact_ref | rationale |
|---|---|---|---|---|---|---|
| CR-A-001 | agent_a | critical | accepted | Define accepted two-half/root carrier integration and terminal snow--soil receipt equations. | coordinated SC successors | Reproducibility is mandatory. |
| CR-A-002 | agent_a | critical | accepted | Add zero-support event-at-start ledger and no-snow--soil-receipt branch. | coordinated SC successors | Removes contradictory positive-support requirement. |
| CR-A-003 | agent_a | critical | accepted | Separate coupled chronology mutation from physical complete-owner mutation. | SC-COUPLEDTIME / SC-SNOWFREEZE | Clock receipt and snow owner are distinct authorities. |
| CR-A-004 | agent_a | major | accepted | Move/add valid active BEI entries in all four contracts. | four BEI tables | Binding residue must be indexed. |
| CR-A-005 | agent_a | major | accepted | Expand invariant rows with authority/evidence and add branch/state/wire/guard/test/gap mappings. | four contracts | Kernel profile is binding. |
| CR-A-006 | agent_a | major | accepted | Specify V4 domain, encoding, ordering, widths, tags, digest and parcel fields. | SC-SNOWFREEZE | Independent byte reconstruction is required. |
| CR-A-007 | agent_a | major | accepted | Preserve released predecessor lifecycle explicitly while successor remains in review. | front matter / registry / BEI | Existing qualified authority cannot be silently downgraded. |
| CR-A-008 | agent_a | major | accepted | Add exact typed terminal owner error IDs. | SC-SNOWFREEZE guard map | Generic failures are insufficient. |
| CR-B-001 | agent_b | critical | accepted | Require terminal unallocated energy within tolerance before acceptance. | SC-SNOWENERGY | Prevents energy deletion. |
| CR-B-002 | agent_b | critical | accepted | Classify cumulative unresolved liquid as cumulative diagnostic and add lane/tile parcel mass equation. | SC-SNOWENERGY / SC-SNOWFREEZE | Prevents double custody. |
| CR-B-003 | agent_b | major | accepted | Same action as CR-A-005. | four contracts | Duplicate independent finding. |
| CR-B-004 | agent_b | major | accepted | Same action as CR-A-004. | four BEI tables | Duplicate independent finding. |
| CR-B-005 | agent_b | critical | accepted | Same action as CR-A-001. | SC-LANDSURFACEENERGY | Duplicate independent finding. |
| CR-B-006 | agent_b | major | accepted | Add explicit execution-mode selector and version-scoped supersession priority. | SC-SNOWENERGY / SC-SNOWFREEZE | Avoids two live receiver chronologies. |
| CR-B-007 | agent_b | major | accepted | Define separate pre-acceptance group digest and post-acceptance accepted-group receipt digest. | SC-COUPLEDTIME | Removes circular preimage. |

All CR-A-001 through CR-A-008 findings were closed by reviewer A's final static
re-review. All CR-B-001 through CR-B-007 science blockers were closed by
reviewer B's final static `GO-WITH-AMENDMENTS`, and verifier B's subsequent
regression pass returned `GO`. Verifier A's post-disposition pass identified
two residuals: dimensionally ambiguous tolerance-family references and missing
retained final-review evidence. Both corrections are present, and verifier A's
final narrow pass returned `GO` after confirming the exact predicates are
bound by the structural test. This disposition is static contract evidence
only. It does not claim runtime implementation, exact-head qualification, or
terminal chronology PASS.
