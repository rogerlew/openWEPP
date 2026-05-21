# Disposition — SC-INFILE-CHANINP-001

Evidence mode: `Static`

| finding_id | source | severity | decision | action_taken | artifact_ref | notes |
| --- | --- | --- | --- | --- | --- | --- |
| `CHAN-A-001` | `review_agent_a` | high | `amended_closed` | Added explicit topology dependency surfaces (`nchan`, `valid_channel_element_ids`) to field and propagation maps and exported them as dedicated cross-file boundary surfaces tied to guard enforcement. | `docs/specifications/science-contracts/contracts/SC-INFILE-CHANINP-001.md:85`, `:86`, `:108`, `:109`, `:170`, `:181`, `:208`, `:209` | Cross-file guard closure for `G-CHN-007`/`G-CHN-008` is now explicit and executable. |
| `CHAN-A-002` | `review_agent_a` | medium | `amended_closed` | Implemented explicit compat unknown-ID retention warning surface and taxonomy/guard mapping (`unknown_ichnum_retained_warning_emitted`, `CHN-W-005`, `G-CHN-008`). | `docs/specifications/science-contracts/contracts/SC-INFILE-CHANINP-001.md:96`, `:118`, `:163`, `:180`, `:195`, `:209` | Compat retention behavior is now deterministic and observable instead of error-only implied behavior. |
| `CHAN-B-001` | `review_agent_b` | high | `amended_closed` | Split strict required-surface missing vs strict open-failure into distinct typed errors (`CHN-E-009` vs `CHN-E-000`) and encoded this distinction in guard behavior. | `docs/specifications/science-contracts/contracts/SC-INFILE-CHANINP-001.md:149`, `:150`, `:202` | High-severity strict missing-vs-open distinction now matches matrix/spec intent. |
| `CHAN-B-002` | `review_agent_b` | medium | `amended_closed` | Closed unknown-ID compat-path gap by wiring warning-based branch semantics through policy text, taxonomy, guard map, and observability exports. | `docs/specifications/science-contracts/contracts/SC-INFILE-CHANINP-001.md:163`, `:180`, `:195`, `:209` | Aligns contract execution model with compatibility expectations in paired spec. |

## Status
- Closed findings: `CHAN-A-001`, `CHAN-A-002`, `CHAN-B-001`, `CHAN-B-002`.
- Open high-severity findings: none.
