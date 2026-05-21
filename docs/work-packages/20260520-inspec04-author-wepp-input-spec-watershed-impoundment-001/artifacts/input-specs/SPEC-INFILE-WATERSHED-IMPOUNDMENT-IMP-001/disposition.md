# Disposition — SPEC-INFILE-WATERSHED-IMPOUNDMENT-IMP-001

Evidence: Static

| finding_id | source | severity | decision | action_taken | artifact_ref | notes |
|---|---|---|---|---|---|---|
| `IMP-A-001` | `review_agent_a.md` | high | amend | Updated lifecycle metadata status to `draft-HOLD` to match unresolved HOLD conditions. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/watershed-impoundment-file.spec.md:6` | Governance alignment restored. |
| `IMP-A-002` | `review_agent_a.md` | medium | amend | Added claim-site evidence anchor IDs for surface-scope/applicability and legacy staged-consumption claims. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/watershed-impoundment-file.spec.md:13` | Tightens claim-to-source traceability without changing semantic content. |
| `IMP-A-003` | `review_agent_a.md` | medium | amend | Added provenance-tag column and per-row provenance labels in gap/conflict register. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/watershed-impoundment-file.spec.md:302` | Conflict provenance is now explicit for disposition/verifier workflows. |
| `B1` | `review_agent_b.md` | high | amend | Resolved by same status correction as `IMP-A-001`. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/watershed-impoundment-file.spec.md:6` | Duplicate high-severity governance issue; single fix closes both. |
| `B2` | `review_agent_b.md` | medium | amend | Reconciled datver policy with grammar by adding explicit `legacy_compat_preamble` variant and strict-mode rejection note. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/watershed-impoundment-file.spec.md:40` | Removes matrix/grammar inconsistency for no-version legacy branch. |
| `B3` | `review_agent_b.md` | medium | amend | Added normative `jpond > npond` behavior: strict equality in strict mode, compatibility-mode warning path, and typed warning surface. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/watershed-impoundment-file.spec.md:206` | Cross-file closure behavior is now explicit instead of implicit legacy side-effect. |

## Unresolved / Promotion blockers
- `G-IMP-001` and `G-IMP-002` remain HOLD-relevant unresolved conflicts.
- `G-IMP-003` and `G-IMP-004` remain open provenance/coverage notes with lower severity.
