# Disposition — SPEC-INFILE-WATERSHED-CHANNEL-CHN-001

Evidence: Static

| finding_id | source | severity | decision | action_taken | artifact_ref | notes |
|---|---|---|---|---|---|---|
| `CHN-A-001` | `review_agent_a.md` | medium | amend | Added explicit claim-site evidence on each scope/applicability bullet (extension, run-mode applicability, hillslope exclusion). | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/watershed-channel-file.spec.md:17` | Non-trivial scope/applicability claims now carry per-bullet evidence binding at claim site. |
| `CHN-A-002` | `review_agent_a.md` | medium | amend | Added provenance-tag column and per-row provenance values in gap/conflict register. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/watershed-channel-file.spec.md:298` | Conflict-resolution authority is now explicit per item. |
| `CHN-A-003` | `review_agent_a.md` | low | amend | Normalized metadata timestamp to full UTC format. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/watershed-channel-file.spec.md:10` | Aligns metadata formatting with repo convention. |
| `B1` | `review_agent_b.md` | high | amend | Added normative strict/compat behavior for `ipeak > 2` `chan.inp` dependency with typed error and compatibility fallback warning. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/watershed-channel-file.spec.md:212` | High-severity closure: control-flow branch is now deterministic for parser contract mapping. |
| `B2` | `review_agent_b.md` | medium | amend | Updated minimal valid example to `ipeak=2` so the example is self-contained and does not require `chan.inp` sidecar context. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/watershed-channel-file.spec.md:224` | Sidecar coupling remains documented separately in Section 8.3. |
| `B3` | `review_agent_b.md` | low | amend | Resolved by same timestamp normalization as `CHN-A-003`. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/watershed-channel-file.spec.md:10` | Duplicate metadata issue across both reviews; single fix closes both. |

## Unresolved / Promotion blockers
- `CHN-GAP-001` and `CHN-GAP-002` were dispositioned in-spec via `CHN-POL-001/002` and are no longer high-severity blockers.
- `CHN-GAP-003` to `CHN-GAP-005` remain medium HOLD gaps pending policy disposition.
