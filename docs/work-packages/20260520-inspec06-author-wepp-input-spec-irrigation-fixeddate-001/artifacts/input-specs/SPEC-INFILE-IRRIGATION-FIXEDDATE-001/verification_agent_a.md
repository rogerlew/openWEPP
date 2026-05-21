# Verification Agent A — SPEC-INFILE-IRRIGATION-FIXEDDATE-001

Evidence: Static

## Per-Finding Verification

| finding_id | source | severity | disposition decision | verdict | evidence (file:line) | verification note |
|---|---|---|---|---|---|---|
| `FDIRR-A-001` | `review_agent_a.md` | high | amend | closed | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-fixeddate-file.spec.md:67`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-fixeddate-file.spec.md:83`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-fixeddate-file.spec.md:139` | Furrow line-5 arity conflict now has explicit strict/compat grammar and typed outcomes. |
| `FDIRR-A-002` | `review_agent_a.md` | medium | amend | closed | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-fixeddate-file.spec.md:131` | Omitted-datver strict-mode rejection is now typed. |
| `FDIRR-A-003` | `review_agent_a.md` | medium | amend | closed | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-fixeddate-file.spec.md:197` | Gap register now includes explicit provenance tags per row. |
| `FDIRR-B1` | `review_agent_b.md` | high | amend | closed | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-fixeddate-file.spec.md:67`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-fixeddate-file.spec.md:139` | High-severity duplicate closed by same arity-policy fix. |
| `FDIRR-B2` | `review_agent_b.md` | medium | amend | closed | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-fixeddate-file.spec.md:131` | No-version strict/compat behavior now explicitly enforced. |
| `FDIRR-B3` | `review_agent_b.md` | medium | amend | closed | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-fixeddate-file.spec.md:197` | Provenance-tag structure requirement satisfied. |

## Package Verdict
PASS

## Remaining High-Severity Open Items
- None from reviewed findings.

Notes:
- HOLD blockers remain in Section 10 (`FDIRR-GAP-001..004`) and should be resolved in subsequent contract/disposition cycles.
