# Disposition — SPEC-INFILE-TCR-001

| finding_id | source | severity | decision | action_taken | artifact_ref | notes |
| --- | --- | --- | --- | --- | --- | --- |
| `TCR-A-001` | `review_agent_a.md` | `medium` | `accepted-fixed` | Added row-level provenance tags to all gap/conflict rows. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/tcr.spec.md:179` | Restores required governance metadata. |
| `TCR-A-002` | `review_agent_a.md` | `medium` | `accepted-fixed` | Converted `taumin <= taumax` from recommendation-only language to enforceable relational guard and added strict/compat typed handling for violations. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/tcr.spec.md:74`; `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/tcr.spec.md:125` | Invariant now executable in contract form. |
| `TCR-A-003` | `review_agent_a.md` | `medium` | `accepted-fixed` | Added explicit compat behavior for non-missing open failures alongside strict typed error path. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/tcr.spec.md:43`; `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/tcr.spec.md:124` | Strict-vs-compat split is now explicit. |
| `TCR-B1` | `review_agent_b.md` | `medium` | `accepted-fixed` | Same closure as `TCR-A-001`: provenance tags added per row. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/tcr.spec.md:179` | Duplicate finding resolved by shared edit. |
| `TCR-B2` | `review_agent_b.md` | `medium` | `accepted-fixed` | Same closure as `TCR-A-002`: typed relational guard for `taumin`/`taumax` added. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/tcr.spec.md:74`; `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/tcr.spec.md:125` | Duplicate finding resolved by shared edit. |
| `TCR-B3` | `review_agent_b.md` | `medium` | `accepted-fixed` | Added explicit strict-vs-compat trailing-token policy with typed strict error path. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/tcr.spec.md:61`; `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/tcr.spec.md:123` | Keeps wepppy producer compatibility while constraining strict mode. |

## Unresolved HOLD Blockers
- `TCR-GAP-001` through `TCR-GAP-005` remain open in spec Section 10 and continue to justify `draft-HOLD` status.
