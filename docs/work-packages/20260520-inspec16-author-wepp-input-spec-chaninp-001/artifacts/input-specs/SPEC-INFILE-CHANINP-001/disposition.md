# Disposition — SPEC-INFILE-CHANINP-001

| finding_id | source | severity | decision | action_taken | artifact_ref | notes |
| --- | --- | --- | --- | --- | --- | --- |
| `CHANINP-A-001` | `review_agent_a.md` | `high` | `accepted-fixed` | Added explicit compat branch for strict missing/open-failure behavior and codified mode-paired taxonomy for core applicability/default branches. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/chaninp.spec.md:46`; `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/chaninp.spec.md:147` | High-severity strict-vs-compat fork ambiguity closed. |
| `CHANINP-A-002` | `review_agent_a.md` | `medium` | `accepted-fixed` | Expanded Section 3 matrix to include missing/open-error/malformed/truncated branches aligned with Section 8 typed outcomes. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/chaninp.spec.md:42`; `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/chaninp.spec.md:48` | Matrix now executable and policy-aligned. |
| `CHANINP-A-003` | `review_agent_a.md` | `medium` | `accepted-fixed` | Replaced generic gap IDs with scoped IDs and added row-level provenance tags in the gap/conflict register. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/chaninp.spec.md:203` | Governance traceability normalized to corpus conventions. |
| `CHANINP-B1` | `review_agent_b.md` | `high` | `accepted-fixed` | Added complete strict-vs-compat typed taxonomy for normalization/default branches (`ichout`, `nchnum`, `dtchr`, parse/open paths). | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/chaninp.spec.md:147`; `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/chaninp.spec.md:156` | High-severity executable-clarity gap closed. |
| `CHANINP-B2` | `review_agent_b.md` | `medium` | `accepted-fixed` | Same closure as `CHANINP-A-003`: provenance tags and scoped gap IDs added. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/chaninp.spec.md:203` | Duplicate finding resolved by shared edit. |
| `CHANINP-B3` | `review_agent_b.md` | `medium` | `accepted-fixed` | Normalized `last_updated_utc` to full UTC timestamp format. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/chaninp.spec.md:10` | Metadata now consistent with corpus audit expectations. |

## Unresolved HOLD Blockers
- `CHANINP-GAP-001` through `CHANINP-GAP-004` remain open in spec Section 10 and continue to gate promotion.
