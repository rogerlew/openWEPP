# Disposition: SC-SOIL-001

Status: complete
Date: 2026-05-20 UTC
Canonical contract: `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
Reviewed commit SHA: `2692ac59f67bed4094df62c5883f3826abe9f468`
Review snapshot: `887f02651b28840130e7ce8ba79266ea64ea0ca7a5ee41907090e419b7318530`
Post-fix snapshot: `65db321373a45a4103f887638cf01ef8a4fff23e6bbd37b7b82a612e29d6c3d8`

Disposition table:

| finding_id | source | severity | decision | action_taken | artifact_ref | notes |
|---|---|---|---|---|---|---|
| `A-001` | `agent_a` | `high` | `accepted` | Normalized evidence-mode token casing to canonical `Static` in metadata and body header. | `docs/specifications/science-contracts/contracts/SC-SOIL-001.md:16`, `:26` | Restores canonical evidence-mode semantics required by procedure. |
| `A-002` | `agent_a` | `medium` | `accepted` | Added explicit `τcadj` row coverage in Symbol Alias Map entry. | `docs/specifications/science-contracts/contracts/SC-SOIL-001.md:133` | Re-establishes symbol continuity for erosion-threshold consumer variable. |
| `A-003` | `agent_a` | `medium` | `accepted` | Expanded `Allowed Degenerate States` to include per-row evidence tags. | `docs/specifications/science-contracts/contracts/SC-SOIL-001.md:140`, `:142`, `:146` | Satisfies claim-level provenance annotation requirement. |
| `A-004` | `agent_a` | `low` | `accepted` | Reworked freeze-thaw anchor naming and source-path form for citation specificity/consistency. | `docs/specifications/science-contracts/contracts/SC-SOIL-001.md:62`, `:92` | Removes mixed-anchor ambiguity for freeze-thaw provenance. |
| `B-001` | `agent_b` | `high` | `accepted` | Added missing `τcadj` alias coverage. | `docs/specifications/science-contracts/contracts/SC-SOIL-001.md:133` | Same corrective action as `A-002`; finding closed by shared amendment. |
| `B-002` | `agent_b` | `medium` | `accepted` | Normalized evidence-mode token casing to `Static`. | `docs/specifications/science-contracts/contracts/SC-SOIL-001.md:16`, `:26` | Aligns document-level evidence token with normative values. |
| `B-003` | `agent_b` | `medium` | `accepted` | Added explicit evidence tags for each degenerate-state claim row. | `docs/specifications/science-contracts/contracts/SC-SOIL-001.md:140`, `:142`, `:146` | Completes claim-level evidence labeling on degenerate-state assertions. |
| `B-004` | `agent_b` | `low` | `accepted` | Standardized freeze-thaw authority row to rooted path form and specific chapter-section composition. | `docs/specifications/science-contracts/contracts/SC-SOIL-001.md:62` | Improves citation hygiene and replay determinism. |

Final disposition note:
- All reported findings were accepted and addressed in the post-fix snapshot.
- Contract lifecycle remains `in_review` because non-promotable gaps
  (`GAP-SOIL-002`, `GAP-SOIL-003`) remain open.
