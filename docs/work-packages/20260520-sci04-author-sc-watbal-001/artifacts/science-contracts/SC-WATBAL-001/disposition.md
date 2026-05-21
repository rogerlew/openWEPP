# Disposition: SC-WATBAL-001

Status: complete
Date: 2026-05-20 UTC
Canonical contract: `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
Reviewed commit SHA: `4c6c504eb79619a3d602943021590e3b25113194`
Review snapshot: `96bcaafb9fb294cb9193031e28c8e0fb8f24e3e4ba6143151e52bcf24966bd5a`
Post-fix snapshot: `da1e4ed533ef318743a02d966198dc54bbd66c7c4e6a99f61b515f6abfd08fd6`

Disposition table:

| finding_id | source | severity | decision | action_taken | artifact_ref | notes |
|---|---|---|---|---|---|---|
| `A-001` | `agent_a` | `medium` | `accepted` | Added explicit `Etp = 0` branch semantics (`Σ Ui = 0`, `Ws = 1`) in invariants, invalid-states list, guard map, and tolerance table. | `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md:102`, `:115`, `:152`, `:160`, `:199` | Resolves divide-by-zero ambiguity and preserves explicit branch behavior. |
| `A-002` | `agent_a` | `low` | `accepted` | Expanded symbol alias map with explicit rows for `Θin` and `Θc`. | `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md:131`, `:138` | Restores variable-symbol continuity coverage for externally relevant symbols. |
| `B-001` | `agent_b` | `medium` | `accepted` | Clarified closure invariant text to require per-daily-step residual evaluation. | `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md:99`, `:195` | Aligns Tier-A enforcement semantics with daily gate expectations. |
| `B-002` | `agent_b` | `low` | `accepted` | Added validation caveat gap entry for deeper-profile agreement limits from Chapter 5 validation section. | `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md:208` | Prevents overstatement of confidence for deep-profile interpretation. |

Final disposition note:
- All reported findings were accepted and addressed in the post-fix snapshot.
- Contract lifecycle remains `in_review` because non-promotable cross-contract
  gaps (`GAP-WATBAL-002`, `GAP-WATBAL-003`) are still open.
