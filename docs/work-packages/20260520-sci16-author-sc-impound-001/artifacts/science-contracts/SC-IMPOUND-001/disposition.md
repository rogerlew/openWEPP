# Disposition: SC-IMPOUND-001

Status: complete
Date: 2026-05-20 UTC
Canonical contract: `docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md`
Reviewed commit SHA: `b420ff46f08a2c401c7f88d09e5f747f894ff66d`
Review snapshot: `a29ca141d62a685c21203b6ad0852896cd9b1867de584b769bc11527e33c89d1`
Post-fix snapshot: `22906d4e190daf2b10839ef7739d7b03bb669f6657decba960b2e505840398c1`

Disposition table:

| finding_id | source | severity | decision | action_taken | artifact_ref | notes |
|---|---|---|---|---|---|---|
| `A-001` | `agent_a` | `high` | `accepted` | Added alias-map rows for `dDep/dt`, `dM/dt`, and `L`. | `docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md:138`, `:141` | Restores declared symbol continuity for externally relevant variables. |
| `A-002` | `agent_a` | `medium` | `accepted` | Added explicit evidence-tag column to `Allowed Degenerate States` and `Tolerance and Numeric Notes`. | `docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md:145`, `:192` | Restores explicit claim-level provenance in behavioral/numeric sections. |
| `A-003` | `agent_a` | `medium` | `accepted` | Normalized document-level evidence mode tokens to canonical `Static`. | `docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md:16`, `:26` | Aligns lifecycle evidence-mode semantics with procedure conventions. |
| `A-004` | `agent_a` | `medium` | `accepted` | Expanded `INV-IMPOUND-005` to explicitly lock signed stage-delta semantics from Eq. [14.5.3]. | `docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md:101` | Prevents divergent daily stage-update sign interpretations. |
| `A-005` | `agent_a` | `low` | `accepted` | Unified authority-anchor source-path style for Chapter-14 anchors. | `docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md:60` | Improves citation replay consistency without altering scientific content. |
| `B-001` | `agent_b` | `high` | `accepted` | Added missing variable-table symbols (`Tday`, `Vset`) and alias-map coverage for `dDep/dt`, `dM/dt`, `L`, `Tday`, `Vset`. | `docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md:85`, `:90`, `:136`, `:138`, `:140`, `:141` | Closes symbol continuity gaps for invariant-critical terms and boundary mapping. |
| `B-002` | `agent_b` | `medium` | `accepted` | Added evidence column/tags for degenerate-state and tolerance claims. | `docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md:145`, `:192` | Same fix as `A-002`; ensures provenance on comparator-governing claims. |
| `B-003` | `agent_b` | `medium` | `accepted` | Normalized evidence mode to `Static` in metadata and body. | `docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md:16`, `:26` | Same fix as `A-003`; keeps evidence-mode parsing/governance consistent. |
| `B-004` | `agent_b` | `low` | `accepted` | Standardized source-path style in authority anchors. | `docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md:61` | Same fix as `A-005`; improves deterministic provenance replay. |

Final disposition note:
- All reported findings were accepted and addressed in the post-fix snapshot.
- Contract lifecycle remains `in_review` because non-promotable gaps
  (`GAP-IMPOUND-001`, `GAP-IMPOUND-002`, `GAP-IMPOUND-003`) remain open and
  explicitly tracked.
