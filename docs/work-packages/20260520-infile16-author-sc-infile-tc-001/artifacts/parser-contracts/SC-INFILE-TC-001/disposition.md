# Disposition — SC-INFILE-TC-001

Evidence mode: `Static`

| finding_id | source | severity | decision | action_taken | artifact_ref | notes |
| --- | --- | --- | --- | --- | --- | --- |
| `TC-A-001` | `review_agent_a` | high | `amended_closed` | Reconciled strict non-ENOENT open-failure policy with `luntc` derivation: strict open-fail path is typed error/no normalized effective-mode state; missing/collapsed compatibility branches remain explicit. | `docs/specifications/science-contracts/contracts/SC-INFILE-TC-001.md:68`, `:73`, `:84`, `:89`, `:166` | Strict-mode fault masking via `luntc=0` collapse is removed. |
| `TC-A-002` | `review_agent_a` | medium | `amended_closed` | Added explicit run-context model surface and linked applicability enforcement to `run_context` in field/propagation/constraints/guards. | `docs/specifications/science-contracts/contracts/SC-INFILE-TC-001.md:74`, `:90`, `:109`, `:134`, `:169` | Watershed-only policy is now deterministic and data-driven. |
| `TC-A-003` | `review_agent_a` | medium | `amended_closed` | Added explicit content-insensitive warning trigger surfaces (`payload_nonempty`, `payload_ignored_warning_emitted`) with compatibility trigger rule and export mapping. | `docs/specifications/science-contracts/contracts/SC-INFILE-TC-001.md:71`, `:72`, `:87`, `:88`, `:144`, `:157`, `:171` | Warning semantics are now executable and auditable. |
| `TC-B-001` | `review_agent_b` | high | `amended_closed` | Closed strict IO policy inconsistency and added explicit `G-TC-003` linkage in `luntc` propagation row. | `docs/specifications/science-contracts/contracts/SC-INFILE-TC-001.md:84`, `:166` | Propagation now explicitly references strict open-error guard path. |
| `TC-B-002` | `review_agent_b` | medium | `amended_closed` | Added field-level warning trigger/export distinction for content-insensitive body handling and compat-warning emission. | `docs/specifications/science-contracts/contracts/SC-INFILE-TC-001.md:72`, `:88`, `:127`, `:144`, `:157`, `:171` | `TC-W-003` now has explicit trigger and observability surface. |

## Status
- Closed findings: `TC-A-001`, `TC-A-002`, `TC-A-003`, `TC-B-001`, `TC-B-002`.
- Open high-severity findings: none.
