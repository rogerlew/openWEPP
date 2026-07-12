# Eligibility Classification

Evidence class: **Ran** for metric/source identity; **Static** for classification.

| ID | Symbol / line | Hash | Class | Aggregate / floor / CRAP | Disposition |
| --- | --- | --- | --- | --- | --- |
| HA-01-R1 | `interpolate_unit_discharge`, 90 | start `90a218ed...5320d90`; current `574d98ab...b1d3fb` | `E-SCIENCE` | included / required / required | Actionable; selection reviews PASS; iteration CRAP 7 |
| HA-01-F1 | `sample_upstream_point`, current 174 | `574d98ab...b1d3fb` | `E-SCIENCE` | included / required / required | Private helper; both match arms directly characterized |

The helper performs numerical boundary handling and cannot receive a dead-code
or observability exclusion. Its point-sampling fallback is currently bypassed
when the conservative integral upstream closure exists; this limits consumer-
path claims but does not change eligibility.

No `R-*` or `X-*` treatment is requested. Review B rejected the provisional
control-flow-based `X-IMPOSSIBLE` proposal, so the point sampler was extracted
into HA-01-F1 and its `Some`/`None` behavior is directly tested.
