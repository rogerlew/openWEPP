# Disposition — SPEC-INFILE-SNOW-001

Evidence: Static

| finding_id | source | severity | decision | action_taken | artifact_ref | notes |
|---|---|---|---|---|---|---|
| `SNOW-A-001` | `review_agent_a.md` | medium | amend | Added provenance-tag column and row-level provenance labels in gap/conflict register. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/snow.spec.md:169` | Conflict provenance is now explicit per row. |
| `SNOW-A-002` | `review_agent_a.md` | medium | amend | Added strict/compat split for trailing tokens and surplus records with typed outcomes (`TrailingTokenError`, `InputRecordCountError`, compatibility warnings). | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/snow.spec.md:116` | Parser determinism for non-canonical trailing content is now explicit. |
| `SNOW-A-003` | `review_agent_a.md` | low | amend | Retargeted `FieldFiniteError` rationale to `snow.txt` parse semantics evidence (`E-WF-01`) rather than generic modern payload parsing. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/snow.spec.md:119` | Improves citation-to-claim traceability. |
| `SNOW-B1` | `review_agent_b.md` | medium | amend | Resolved by same provenance-tagging update as `SNOW-A-001`. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/snow.spec.md:169` | Duplicate finding closure. |
| `SNOW-B2` | `review_agent_b.md` | medium | amend | Resolved by explicit strict/compat grammar and typed behavior for trailing tokens. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/snow.spec.md:51` | Compatibility tolerance is now explicitly mode-gated. |
| `SNOW-B3` | `review_agent_b.md` | low | amend | Clarified policy boundary: baseline strict/compat invariant requires positive densities; broader bounds remain HOLD pending canonical policy. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/snow.spec.md:120` | Prevents ambiguity between enforced invariants and deferred policy ranges. |

## Unresolved / HOLD blockers
- `SNOW-GAP-001`: no usersum canonical `snow.txt` format table (`[DIRECT][E-US-02]`).
- `SNOW-GAP-002`: unit-label conflict (`g/cm^3` comments vs legacy `kg/m^3` semantics) (`[DIRECT][E-WF-02]`, `[DIRECT][E-WF-05]`, `[DIRECT][E-WP-01]`).
- `SNOW-GAP-003`: bounds-policy divergence between legacy defaults and modern guards (`[DIRECT][E-WF-01]`, `[DIRECT][E-WP-02]`).
- `SNOW-GAP-004`: unresolved `rst` bounds policy (`[DIRECT][E-WF-01]`, `[DIRECT][E-WP-02]`).
