# Disposition — SPEC-INFILE-WEPPUI-001

Evidence: Static

| finding_id | source | severity | decision | action_taken | artifact_ref | notes |
|---|---|---|---|---|---|---|
| `WEPPUI-A-001` | `review_agent_a.md` | medium | amend | Added explicit mode-gated typed outcomes for non-empty sentinel payload (`SentinelPayloadNotEmptyError` vs `SentinelPayloadIgnoredWarning`). | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/wepp-ui.spec.md:113` | Presence-only sentinel behavior is now deterministic for strict/compat modes. |
| `WEPPUI-A-002` | `review_agent_a.md` | medium | amend | Added provenance-tag column and per-row provenance tags in gap/conflict register. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/wepp-ui.spec.md:147` | Governance traceability requirement satisfied. |
| `WEPPUI-A-003` | `review_agent_a.md` | low | amend | Reclassified cross-repo ownership concern to non-blocking note (`WEPPUI-NOTE-002`). | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/wepp-ui.spec.md:152` | Separates governance/provenance notes from correctness blockers. |
| `WEPPUI-B1` | `review_agent_b.md` | medium | amend | Resolved by same provenance-tagged gap table update as `WEPPUI-A-002`. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/wepp-ui.spec.md:147` | Duplicate finding closure. |
| `WEPPUI-B2` | `review_agent_b.md` | medium | amend | Resolved by explicit strict/compat typed behavior for non-empty sentinel content. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/wepp-ui.spec.md:113` | Removes parser ambiguity for content-bearing sentinel files. |
| `WEPPUI-B3` | `review_agent_b.md` | low | amend | Resolved by non-blocking note classification for interoperability ownership/provenance concern. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/wepp-ui.spec.md:152` | Promotion gate logic now cleaner. |

## Unresolved / HOLD blockers
- `WEPPUI-GAP-001`: unresolved enforcement severity for 7778-soil compatibility recommendation (`[DIRECT][E-US-02]`, `[DIRECT][E-WF-01]`, `[DIRECT][E-WF-05]`).
- `WEPPUI-GAP-002`: unresolved policy for legacy collapse of non-not-found open errors into disabled state (`[DIRECT][E-WF-01]`).
