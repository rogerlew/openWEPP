# Disposition: SC-HYDRAULICS-001

Status: complete
Date: 2026-05-20 UTC
Canonical contract: `docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md`
Reviewed commit SHA: `2692ac59f67bed4094df62c5883f3826abe9f468`
Review snapshot: `a45968d831c040fa30714d5756e60c8e07a33aee887524ab874b24daf0f982b0`
Post-fix snapshot: `aac85308fbb9766c1063f2d7379b4f6ab12c9c6b9eababbb98c3e49e230938f2`

Disposition table:

| finding_id | source | severity | decision | action_taken | artifact_ref | notes |
|---|---|---|---|---|---|---|
| `A-001` | `agent_a` | `high` | `accepted` | Normalized document evidence-mode tokens to canonical `Static` in metadata and body header. | `docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md:16`, `:26` | Aligns evidence-mode encoding with procedure tokens. |
| `A-002` | `agent_a` | `medium` | `accepted` | Added claim-level evidence tagging to `Allowed Degenerate States` and tolerance narrative/table surfaces. | `docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md:146`, `:190`, `:193` | Restores per-claim provenance tags required by procedure. |
| `A-003` | `agent_a` | `medium` | `accepted` | Added missing `τfe` alias coverage in the Symbol Alias Map. | `docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md:142` | Completes symbol-trace continuity for erosion-coupling variable set. |
| `A-004` | `agent_a` | `low` | `accepted` | Standardized Chapter-10 anchor source-path formatting to rooted `references/50201000/...` form. | `docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md:63`, `:64` | Improves citation-path consistency for audit replay. |
| `B-001` | `agent_b` | `high` | `accepted` | Added `τfe` alias row coverage in canonical symbol mapping. | `docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md:142` | Same corrective action as `A-003`; closed by shared amendment. |
| `B-002` | `agent_b` | `medium` | `accepted` | Normalized evidence-mode token casing (`Static`) in front matter and body evidence header. | `docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md:16`, `:26` | Satisfies normative evidence-mode requirement. |
| `B-003` | `agent_b` | `medium` | `accepted` | Added evidence tags for degenerate-state and tolerance claims. | `docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md:146`, `:190`, `:193` | Restores deterministic provenance labeling on review-critical claims. |
| `B-004` | `agent_b` | `low` | `accepted` | Replaced shortened chapter paths with rooted source paths in authority-anchor table. | `docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md:63`, `:64` | Closes citation-style inconsistency noted by review. |

Final disposition note:
- All reported findings were accepted and addressed in the post-fix snapshot.
- Contract lifecycle remains `in_review` due open non-promotable cross-contract
  gap `GAP-HYD-003`.
