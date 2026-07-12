# Eligibility Classification

Evidence class: **Ran** for metric/source identity; **Static** for classification.

| ID | Symbol / line | Hash | Class | Aggregate / floor / CRAP | Disposition |
| --- | --- | --- | --- | --- | --- |
| HA-01-R1 | `interpolate_unit_discharge`, 90 | `90a218ed...5320d90` | `E-SCIENCE` | included / required / required | Actionable; dual selection review PASS |

The helper performs numerical boundary handling and cannot receive a dead-code
or observability exclusion. Its point-sampling fallback is currently bypassed
when the conservative integral upstream closure exists; this limits consumer-
path claims but does not change eligibility.
