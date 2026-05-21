# Disposition: SC-RESIDUE-001

Status: complete
Date: 2026-05-20 UTC
Canonical contract: `docs/specifications/science-contracts/contracts/SC-RESIDUE-001.md`
Reviewed commit SHA: `2692ac59f67bed4094df62c5883f3826abe9f468`
Review snapshot: `0c85de74bd8bb3b71e1cc43036e3751717ea161120f641d79e23bfa1753e923b`
Post-fix snapshot: `8516dc2a445556cdf5422b72d7ca2db08f1de887a62f9fd97f8294c98ac30ae2`

Disposition table:

| finding_id | source | severity | decision | action_taken | artifact_ref | notes |
|---|---|---|---|---|---|---|
| `A-001` | `agent_a` | `high` | `accepted` | Retained governance `HOLD` posture with explicit non-promotable gaps in contract and registry entry. | `docs/specifications/science-contracts/contracts/SC-RESIDUE-001.md:211`, `:212`; `docs/specifications/science-contracts/index.md:34` | Finding is a valid gate observation; no corrective edit removes this while gaps remain open. |
| `A-002` | `agent_a` | `medium` | `accepted` | Replaced mixed-unit variable rows with explicit unit rows for climate/soil/rangeland driver symbols. | `docs/specifications/science-contracts/contracts/SC-RESIDUE-001.md:71`, `:76`, `:77`, `:78`, `:87`, `:88` | Resolves per-symbol unit clarity requirement. |
| `A-003` | `agent_a` | `medium` | `accepted` | Replaced identity-only alias rows with explicit legacy Chapter-9 variable-token mappings and updated gap language accordingly. | `docs/specifications/science-contracts/contracts/SC-RESIDUE-001.md:130`, `:136`, `:142`, `:146`, `:147`, `:211` | Alias closure remains non-promotable until openWEPP runtime field bindings are fixed. |
| `B-001` | `agent_b` | `high` | `accepted` | Added `Cr` to Variables and Units table with ET interface unit declaration. | `docs/specifications/science-contracts/contracts/SC-RESIDUE-001.md:71` | Closes missing externally relevant symbol coverage. |
| `B-002` | `agent_b` | `high` | `accepted` | Added explicit alias mappings to legacy variable tokens and removed identity-placeholder-only map behavior. | `docs/specifications/science-contracts/contracts/SC-RESIDUE-001.md:130`, `:136`, `:142`, `:146`, `:147`, `:211` | Same corrective action as `A-003`; finding closed by shared amendment. |
| `B-003` | `agent_b` | `medium` | `accepted` | Split coarse unit-bucket rows into explicit unit declarations. | `docs/specifications/science-contracts/contracts/SC-RESIDUE-001.md:76`, `:77`, `:78`, `:87`, `:88` | Same corrective action as `A-002`; finding closed by shared amendment. |

Final disposition note:
- All amend findings were accepted and resolved in the post-fix snapshot.
- Contract lifecycle remains `in_review` and promotion remains `HOLD` while
  `GAP-RESIDUE-002` and `GAP-RESIDUE-003` remain non-promotable.
