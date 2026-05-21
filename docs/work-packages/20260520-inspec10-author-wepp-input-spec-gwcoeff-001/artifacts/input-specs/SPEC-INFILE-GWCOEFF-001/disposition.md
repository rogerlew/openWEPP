# Disposition — SPEC-INFILE-GWCOEFF-001

Evidence: Static

| finding_id | source | severity | decision | action_taken | artifact_ref | notes |
|---|---|---|---|---|---|---|
| `GWCOEFF-A-001` | `review_agent_a.md` | medium | amend | Added provenance-tag column and per-row provenance tags in gap/conflict register. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/gwcoeff.spec.md:180` | Required conflict provenance structure is now explicit. |
| `GWCOEFF-A-002` | `review_agent_a.md` | medium | amend | Added explicit typed rejection for version/datver-prefixed first-line variants. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/gwcoeff.spec.md:119` | Rejection branch now maps to executable error taxonomy. |
| `GWCOEFF-A-003` | `review_agent_a.md` | low | amend | Reclassified provenance ownership row to non-blocking note (`GWCOEFF-NOTE-001`). | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/gwcoeff.spec.md:186` | Promotion blockers now reflect correctness-impact items. |
| `GWCOEFF-B1` | `review_agent_b.md` | medium | amend | Resolved by same provenance-tagging update as `GWCOEFF-A-001`. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/gwcoeff.spec.md:180` | Duplicate finding closure. |
| `GWCOEFF-B2` | `review_agent_b.md` | medium | amend | Resolved by explicit typed `FormatVersionLineUnsupportedError` branch. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/gwcoeff.spec.md:119` | Deterministic reject behavior implemented. |
| `GWCOEFF-B3` | `review_agent_b.md` | low | amend | Added explicit strict/compat typed policy for trailing token handling. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/gwcoeff.spec.md:121` | Prevents parser divergence on labeled/annotated lines. |

## Unresolved / HOLD blockers
- `GWCOEFF-GAP-001`: no authoritative usersum format section for `gwcoeff.txt`; legacy-code authority ratification unresolved (`[DIRECT][E-US-01]`, `[DIRECT][E-US-02]`).
- `GWCOEFF-GAP-002`: unresolved namespace collision safeguards between `chan.inp` baseflow coefficient and `gwcoeff.txt` `bfcoeff` (`[DIRECT][E-US-03]`, `[DIRECT][E-WF-03]`).
- `GWCOEFF-GAP-003`: unresolved policy for explicit value defaults when optional sidecar is absent (`[DIRECT][E-WF-01]`, `[DIRECT][E-WF-03]`, `[DIRECT][E-WP-01]`).
- `GWCOEFF-GAP-004`: unresolved compat policy for malformed present-file read failures beyond typed strict errors (`[DIRECT][E-WF-01]`).
