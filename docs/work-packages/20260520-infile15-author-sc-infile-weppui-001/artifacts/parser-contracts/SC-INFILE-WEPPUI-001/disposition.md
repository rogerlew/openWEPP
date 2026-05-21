# Disposition — SC-INFILE-WEPPUI-001

Evidence mode: `Static`

| finding_id | source | severity | decision | action_taken | artifact_ref | notes |
| --- | --- | --- | --- | --- | --- | --- |
| `WUI-A-001` | `review_agent_a` | high | `amended_closed` | Reconciled strict non-ENOENT open-failure policy with effective-mode derivation: strict open-failure is typed error/no normalized state; compatibility collapse is explicit via `open_result`. | `docs/specifications/science-contracts/contracts/SC-INFILE-WEPPUI-001.md:71`, `:75`, `:90`, `:122`, `:173` | Strict IO faults no longer collapse silently into daily-mode state. |
| `WUI-A-002` | `review_agent_a` | high | `amended_closed` | Added requested/effective/divergence observability surfaces in field, propagation, and boundary mappings. | `docs/specifications/science-contracts/contracts/SC-INFILE-WEPPUI-001.md:70`, `:79`, `:85`, `:94`, `:145`, `:146`, `:174` | Requested-vs-effective drift is now executable and externally visible. |
| `WUI-A-003` | `review_agent_a` | medium | `amended_closed` | Codified deterministic multi-soil reduction rule (`solwpv_reduced_min=min(solwpv[1..n])`) and guard-linked compatibility evaluation. | `docs/specifications/science-contracts/contracts/SC-INFILE-WEPPUI-001.md:77`, `:92`, `:109`, `:137`, `:170` | Multi-soil compatibility decisions are now deterministic. |
| `WUI-B-001` | `review_agent_b` | high | `amended_closed` | Closed same strict IO collapse issue by decoupling strict failure path from normalized `ui_run=0` branch and adding explicit open-branch provenance. | `docs/specifications/science-contracts/contracts/SC-INFILE-WEPPUI-001.md:71`, `:75`, `:90`, `:122`, `:173` | Branch semantics now preserve strict/compat separation. |
| `WUI-B-002` | `review_agent_b` | medium | `amended_closed` | Added explicit model and export fields for requested/effective mode and divergence closure. | `docs/specifications/science-contracts/contracts/SC-INFILE-WEPPUI-001.md:70`, `:79`, `:94`, `:145`, `:174` | Observability requirement is fully materialized. |

## Status
- Closed findings: `WUI-A-001`, `WUI-A-002`, `WUI-A-003`, `WUI-B-001`, `WUI-B-002`.
- Open high-severity findings: none.
