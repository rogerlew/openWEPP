# CQR05 Function Length Before

Evidence: Static.

Target file line count before refactor:

- `hydrology_phase_erod14.rs`: `648` lines

Function length baseline:

| Function | Start line | End line | Lines |
| --- | ---: | ---: | ---: |
| `Wb11HydrologyKernel::run_erod14_wave2` | 6 | 648 | 643 |

Baseline issue:

- One high-complexity function owned activation, input loading, validation,
  case matching, class transport, reproportioning, enrichment, and writebacks.
