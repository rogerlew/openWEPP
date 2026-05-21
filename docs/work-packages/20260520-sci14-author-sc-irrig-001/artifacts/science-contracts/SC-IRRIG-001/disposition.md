# Disposition: SC-IRRIG-001

Status: complete
Date: 2026-05-20 UTC
Canonical contract: `docs/specifications/science-contracts/contracts/SC-IRRIG-001.md`
Reviewed commit SHA: `2692ac59f67bed4094df62c5883f3826abe9f468`
Review snapshot: `b16aac05efcd96a68a5a84d7d9793c1e569bc3c097643c464abcf2d6798efb79`
Post-fix snapshot: `9585ff5106cfc403678448a4ef9d1cb715dd11c5bcb704d5ad8aa664b5a23d24`

Disposition table:

| finding_id | source | severity | decision | action_taken | artifact_ref | notes |
|---|---|---|---|---|---|---|
| `A-001` | `agent_a` | `high` | `accepted` | Added claim-level evidence tags in Purpose/Scientific Scope and added evidence tagging column to Allowed Degenerate States plus tolerance rows. | `docs/specifications/science-contracts/contracts/SC-IRRIG-001.md:34`, `:40`, `:151`, `:198` | Restores claim-level provenance typing required by procedure. |
| `A-002` | `agent_a` | `medium` | `accepted` | Added explicit peak-runoff alias coverage for canonical/legacy naming in Symbol Alias Map. | `docs/specifications/science-contracts/contracts/SC-IRRIG-001.md:146` | Closes symbol continuity gap for erosion coupling output. |
| `A-003` | `agent_a` | `low` | `accepted` | Normalized evidence-mode tokenization to `Static` in metadata and document header. | `docs/specifications/science-contracts/contracts/SC-IRRIG-001.md:16`, `:26` | Aligns with procedure-level evidence-mode convention. |
| `B-001` | `agent_b` | `high` | `accepted` | Standardized peak-runoff symbol to `qp` in variable/invariant/obligation text and added explicit `qp` (primary) / `Qp` (legacy) alias mapping. | `docs/specifications/science-contracts/contracts/SC-IRRIG-001.md:99`, `:114`, `:146`, `:174` | Removes symbol ambiguity and preserves canonical boundary continuity. |
| `B-002` | `agent_b` | `medium` | `accepted` | Added evidence-tag column for Allowed Degenerate States claims. | `docs/specifications/science-contracts/contracts/SC-IRRIG-001.md:151` | Resolves untagged claim rows in degenerate-state section. |
| `B-003` | `agent_b` | `medium` | `accepted` | Replaced broad Chapter-11 coupling citation with precise §11.2.2 Eq. [11.2.5] anchor and retained Chapter-12 coupling context. | `docs/specifications/science-contracts/contracts/SC-IRRIG-001.md:72`, `:114` | Improves citation precision for non-trivial coupling claims. |

Final disposition note:
- All findings from Agent A and Agent B were accepted and addressed in the
  post-fix snapshot.
- Contract lifecycle remains `in_review` because non-promotable cross-contract
  gaps (`GAP-IRRIG-002`, `GAP-IRRIG-003`) remain open.
