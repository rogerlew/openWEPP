# Disposition — SPEC-INFILE-SLOPE-SLP-001

Evidence: Static

| finding_id | source | severity | decision | action_taken | artifact_ref | notes |
|---|---|---|---|---|---|---|
| `SLOPE-A-001` | `review_agent_a.md` | low | amend | Added explicit provenance-tag column to the gap/conflict register rows. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/slope-file.spec.md:210` | Satisfies required provenance tagging for conflict governance. |
| `B1` | `review_agent_b.md` | medium | amend | Replaced ambiguous mixed-mode invalid example with unambiguous mixed encoding (`0.5` normalized interior point with `100.0` meter endpoint). | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/slope-file.spec.md:178` | Tightens guard-fixture semantics for `DistanceModeMixError`. |
| `B2` | `review_agent_b.md` | medium | amend | Added strict-mode contract text for no-version legacy form and explicit typed error (`MissingDatverHeaderError`). | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/slope-file.spec.md:43` | Also reflected in typed error table at `:134`. |
| `B3` | `review_agent_b.md` | low | amend | Reclassified non-blocking provenance item from HOLD row to `SLOPE-NOTE-001` with explicit `NOTE` disposition. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/slope-file.spec.md:215` | Removes ambiguity between blockers and provenance notes. |

## Unresolved / Promotion blockers
- `SLOPE-GAP-001` through `SLOPE-GAP-003` remain active HOLD items in the canonical spec and continue to block promotion to non-HOLD status.
