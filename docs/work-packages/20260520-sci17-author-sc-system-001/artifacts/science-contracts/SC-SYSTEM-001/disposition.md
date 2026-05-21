# Disposition: SC-SYSTEM-001

Status: complete
Date: 2026-05-20 UTC
Canonical contract: `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
Reviewed commit SHA: `2692ac59f67bed4094df62c5883f3826abe9f468`
Review snapshot: `2c73a0a58b48d1c202e03fadc66622b52d9dffdf272385530664c7296b0c7971`
Post-fix snapshot: `46780c0d7f914334c424e1ded3bfda03aeadc9cefb47d0fb6f20423d75e8d266`

Disposition table:

| finding_id | source | severity | decision | action_taken | artifact_ref | notes |
|---|---|---|---|---|---|---|
| `A-001` | `agent_a` | `high` | `accepted` | Normalized evidence metadata (`evidence_level`) and body evidence-mode token to canonical `Static` form. | `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md:16`, `:26` | Restores procedure-compliant evidence labeling and removes metadata parser ambiguity. |
| `A-002` | `agent_a` | `high` | `accepted` | Added explicit alias row for duration-family symbols (`durc`, `durrunon`, `durchan`, `durirrig`). | `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md:143` | Closes symbol-continuity gap for `INV-SYSTEM-003`. |
| `A-003` | `agent_a` | `medium` | `accepted` | Added explicit evidence column/tags to all allowed-degenerate-state rows. | `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md:152`-`:158` | Aligns claim-level provenance with other normative sections. |
| `A-004` | `agent_a` | `medium` | `accepted` | Added evidence column/tags to all tolerance rows. | `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md:221`-`:227` | Makes gate-threshold provenance explicit and auditable. |
| `B-001` | `agent_b` | `high` | `accepted` | Same corrective metadata/evidence-mode normalization as `A-001`. | `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md:16`, `:26` | Shared fix closes both reviewer findings. |
| `B-002` | `agent_b` | `high` | `accepted` | Same duration-family alias-map amendment as `A-002`. | `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md:143` | Shared fix closes both reviewer findings. |
| `B-003` | `agent_b` | `medium` | `accepted` | Same evidence-label augmentation for allowed degenerate states as `A-003`. | `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md:152`-`:158` | Shared fix closes both reviewer findings. |
| `B-004` | `agent_b` | `medium` | `accepted` | Same tolerance-table evidence-label augmentation as `A-004`. | `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md:221`-`:227` | Shared fix closes both reviewer findings. |
| `B-005` | `agent_b` | `low` | `accepted` | Clarified `GAP-SYSTEM-004` with explicit chapter-cited CREAMS dataset range (`70 ha` to `6200 ha`). | `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md:236` | Converts broad applicability caveat into explicit replayable risk boundary. |

Final disposition note:
- All reported findings were accepted and addressed in the post-fix snapshot.
- Contract lifecycle remains `in_review` because non-promotable gaps
  (`GAP-SYSTEM-001`, `GAP-SYSTEM-002`) remain open.
