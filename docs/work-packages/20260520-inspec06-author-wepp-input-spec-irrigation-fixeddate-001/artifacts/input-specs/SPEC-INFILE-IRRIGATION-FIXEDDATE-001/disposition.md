# Disposition — SPEC-INFILE-IRRIGATION-FIXEDDATE-001

Evidence: Static

| finding_id | source | severity | decision | action_taken | artifact_ref | notes |
|---|---|---|---|---|---|---|
| `FDIRR-A-001` | `review_agent_a.md` | high | amend | Added explicit strict/compat parser policy for furrow line-5 arity conflict and typed outcomes (`RecordArityError` vs compatibility warning). | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-fixeddate-file.spec.md:67` | High-severity closure for executable parser determinism. |
| `FDIRR-A-002` | `review_agent_a.md` | medium | amend | Added strict-mode typed rejection for omitted `datver` header in Section 8. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-fixeddate-file.spec.md:131` | Aligns matrix/branch narrative with typed contract outcomes. |
| `FDIRR-A-003` | `review_agent_a.md` | medium | amend | Added provenance-tag column and per-row provenance values to gap/conflict register. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-fixeddate-file.spec.md:197` | Governance/disposition traceability requirement met. |
| `FDIRR-B1` | `review_agent_b.md` | high | amend | Resolved by same strict/compat furrow line-5 arity policy and typed error/warning outcomes. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-fixeddate-file.spec.md:139` | High-severity duplicate closure. |
| `FDIRR-B2` | `review_agent_b.md` | medium | amend | Resolved by explicit strict-mode no-datver rejection path. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-fixeddate-file.spec.md:131` | Compatibility behavior remains explicitly gated. |
| `FDIRR-B3` | `review_agent_b.md` | medium | amend | Resolved by provenance-tagged conflict rows. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-fixeddate-file.spec.md:197` | Row-level provenance now explicit. |

## Unresolved / HOLD blockers
- `FDIRR-GAP-001`: canonical semantic ratification for `tdepl` vs legacy 3-field furrow shape remains open (`[DIRECT][E-US-02]`, `[DIRECT][E-WF-09]`).
- `FDIRR-GAP-002`: `iryr` semantic conflict (calendar vs simulation-relative year) remains unresolved (`[DIRECT][E-US-02]`, `[DIRECT][E-WF-10]`).
- `FDIRR-GAP-003`: fixed-date compatibility-floor enforcement policy unresolved (`[DIRECT][E-WF-05]`, `[DIRECT][E-WF-06]`).
- `FDIRR-GAP-004`: strict chronology policy vs legacy warning-only behavior unresolved (`[DIRECT][E-US-02]`, `[DIRECT][E-WF-04]`).
