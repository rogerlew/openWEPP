# Disposition: SC-CLIMATE-001

Status: complete
Date: 2026-05-20 UTC
Evidence mode: `Static`

Canonical contract under disposition:
- path: `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- commit_sha: `4c6c504eb79619a3d602943021590e3b25113194`
- reviewed_snapshot_pre_fix: `498460d4fc3828cae543af6988a794d7d366f5888636334c3368dbb3bb36d12d`
- reviewed_snapshot_post_fix: `88bc3b9d70b8db6efdd5f5726ce28e4d2fc583fb6cb1bd3a66f4ec851a6d4073`

Disposition table:

| finding_id | source | severity | decision | action_taken | artifact_ref | notes |
|---|---|---|---|---|---|---|
| A-001 | agent_a | medium | amended | Promoted canonical symbol IDs to Greek-primary form (`α`, `β`) while keeping textual aliases. | `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md:68`; `:69`; `:131`; `:132` | Closed in amendment pass. |
| A-002 | agent_a | medium | amended | Added missing accumulated-frequency symbols (`Ak`, `Nk`, `N`) to variable table and alias map. | `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md:80`; `:81`; `:82`; `:139` | Closed in amendment pass. |
| B-001 | agent_b | medium | amended | Scoped breakpoint start/end convention invariant to generated storm events (`P > 0`). | `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md:102` | Closed in amendment pass. |
| B-002 | agent_b | low | amended | Added explicit alias-map coverage for `X`, `Dp`, and `De` (and retained explicit identity coverage). | `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md:135`; `:138` | Closed in amendment pass. |

Disposition summary:
- Medium/high findings: all closed.
- Final promotion state for this revision: `HOLD` (open `non-promotable` gaps `GAP-CLIMATE-003` through `GAP-CLIMATE-005` remain explicit; `GAP-CLIMATE-002` is now `promotable-with-risk`).
