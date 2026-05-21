# Disposition: SC-PERC-001

Status: complete
Date: 2026-05-20 UTC
Canonical contract: `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
Reviewed commit SHA: `2692ac59f67bed4094df62c5883f3826abe9f468`
Review snapshot: `5ed9c61a3ca632cceaeeca572e41ccfec7a36310bbe1e9bbbc0f00eab27bb07b`
Post-fix snapshot: `9a5572193bb35eff3d7352b7044bcb34761d6fdc9aaa916aecc19d41d00b20e0`

Disposition table:

| finding_id | source | severity | decision | action_taken | artifact_ref | notes |
|---|---|---|---|---|---|---|
| `A-001` | `agent_a` | `high` | `accepted` | Added explicit Promotion Readiness section that locks this revision in `HOLD` while alias and cross-contract gaps remain open. | `docs/specifications/science-contracts/contracts/SC-PERC-001.md:190` | Finding is valid governance signal; no claim of promotion-readiness is made in v2. |
| `A-002` | `agent_a` | `high` | `accepted` | Replaced indirect Chapter-7 reference with direct Chapter-7 authority anchors and propagated them into conductivity invariant coverage. | `docs/specifications/science-contracts/contracts/SC-PERC-001.md:61`, `:62`, `:90` | Strengthens provenance for conductivity-domain invariants. |
| `A-003` | `agent_a` | `medium` | `accepted` | Normalized evidence-mode tokens from `static` to `Static` in metadata/body. | `docs/specifications/science-contracts/contracts/SC-PERC-001.md:16`, `:26` | Aligns this contract with procedure evidence-mode token form. |
| `A-004` | `agent_a` | `medium` | `accepted` | Expanded tolerance table with explicit Evidence column and per-row labels. | `docs/specifications/science-contracts/contracts/SC-PERC-001.md:174` | Improves traceability for threshold-governed gates. |
| `A-005` | `agent_a` | `low` | `accepted` | Added evidence tags to all Allowed Degenerate States rationales. | `docs/specifications/science-contracts/contracts/SC-PERC-001.md:130` | Restores in-section evidence-label consistency. |
| `B-001` | `agent_b` | `high` | `accepted` | Clarified identity-alias placeholder state and explicit non-promotable posture while alias finalization is pending. | `docs/specifications/science-contracts/contracts/SC-PERC-001.md:113`, `:186`, `:190` | Alias continuity remains pending by design and is now explicitly governance-held. |
| `B-002` | `agent_b` | `high` | `accepted` | Retained explicit governance `HOLD` posture tied to unresolved `SC-SUBHYD-001` dependency; no contradictory promotion language remains. | `docs/specifications/science-contracts/contracts/SC-PERC-001.md:95`, `:187`, `:193` | No corrective physics change required; governance assertion confirmed. |
| `B-003` | `agent_b` | `medium` | `accepted` | Normalized evidence-mode tokens to `Static`. | `docs/specifications/science-contracts/contracts/SC-PERC-001.md:16`, `:26` | Same fix closes both A-003 and B-003. |
| `B-004` | `agent_b` | `medium` | `accepted` | Clarified that `TOL-PERC-003` is comparator-only and does not weaken runtime hard-fail semantics for invalid restriction domain. | `docs/specifications/science-contracts/contracts/SC-PERC-001.md:91`, `:105`, `:178` | Removes clamp-vs-fail ambiguity. |
| `B-005` | `agent_b` | `low` | `accepted` | Added direct Chapter-7 anchors (`§7.8`, `§7.9.7`) and updated conductivity invariant authority mapping. | `docs/specifications/science-contracts/contracts/SC-PERC-001.md:61`, `:62`, `:90` | Same anchor fix also closes A-002. |

Final disposition note:
- All reported findings were accepted and addressed in the post-fix snapshot.
- Contract lifecycle remains `in_review` and non-promotable because `GAP-PERC-002`
  and `GAP-PERC-003` remain open by explicit governance rule.
