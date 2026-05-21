# Disposition — SPEC-INFILE-WATERSHED-STRUCTURE-STR-001

Evidence: Static

| finding_id | source | severity | decision | action_taken | artifact_ref | notes |
|---|---|---|---|---|---|---|
| `STR-A-001` | `review_agent_a.md` | medium | amend | Added explicit provenance-tag column for each gap/conflict row. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/watershed-structure-file.spec.md:171` | Conflict provenance is now explicit and auditable per row. |
| `B1` | `review_agent_b.md` | medium | amend | Added explicit normative structure record-count formula (`1 + (nchan + npond)` logical records). | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/watershed-structure-file.spec.md:71` | Enables deterministic truncated/extra-row guard mapping. |
| `B2` | `review_agent_b.md` | medium | amend | Added explicit typed outcomes for Case B (`InputLegacyNoDatverDisallowed` in strict mode; compatibility warning when enabled). | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/watershed-structure-file.spec.md:128` | Removes branch-enforcement ambiguity for datver compatibility path. |
| `B3` | `review_agent_b.md` | low | amend | Expanded `G1` with concrete constraint bullets and intended staging scope. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/watershed-structure-file.spec.md:173` | Makes downstream verifier closure criteria machine-actionable. |

## Unresolved / Promotion blockers
- `G1`–`G4` remain explicit HOLD items in the spec and still block promotion out of `draft-HOLD`.
