# Verification Agent A — SPEC-INFILE-CHANINP-001

Evidence: Static

## Per-Finding Verification

| finding_id | source | severity | disposition decision | verdict | evidence (file:line) | verification note |
| --- | --- | --- | --- | --- | --- | --- |
| `CHANINP-A-001` | `review_agent_a.md` | high | accepted-fixed | closed | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/chaninp.spec.md:46`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/chaninp.spec.md:47`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/chaninp.spec.md:150`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/chaninp.spec.md:151` | Missing/open-failure branches now have explicit strict-vs-compat paired outcomes consistent with legacy-default compatibility behavior. |
| `CHANINP-A-002` | `review_agent_a.md` | medium | accepted-fixed | closed | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/chaninp.spec.md:42`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/chaninp.spec.md:48` | Version/applicability matrix now includes missing/open-error/malformed/truncated branches aligned with typed outcomes. |
| `CHANINP-A-003` | `review_agent_a.md` | medium | accepted-fixed | closed | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/chaninp.spec.md:203` | Gap register now uses scoped IDs and row-level provenance tags. |
| `CHANINP-B1` | `review_agent_b.md` | high | accepted-fixed | closed | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/chaninp.spec.md:147`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/chaninp.spec.md:154`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/chaninp.spec.md:155`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/chaninp.spec.md:156` | Normalization/default branches (`ichout`, `nchnum`, `dtchr`) now have complete strict/compat typed taxonomy. |
| `CHANINP-B2` | `review_agent_b.md` | medium | accepted-fixed | closed | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/chaninp.spec.md:203` | Duplicate governance/provenance finding closed by same gap-register normalization. |
| `CHANINP-B3` | `review_agent_b.md` | medium | accepted-fixed | closed | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/chaninp.spec.md:10` | `last_updated_utc` normalized to full UTC timestamp format. |

## Package Verdict
PASS

## Remaining High-Severity Open Items
- None from reviewed findings.

Notes:
- Spec remains `draft-HOLD` due to unresolved Section 10 blockers (`CHANINP-GAP-001..004`).
