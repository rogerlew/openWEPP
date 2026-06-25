# Review Disposition

Evidence mode: Static + Ran.

## Scope Reviewed

- WAT `Snow-Depth` schema and row serialization.
- Direct and compatibility publication lineage for `snow.runtime_depth_m`.
- Unit registry aliases for `hillslope_wat.Snow-Depth:mm`.
- Observed harness snow-depth control calculation.
- Classifier behavior for missing, unmatched, failed, and passed snow control.
- Source scan for accidental Qwet/frzftp production coupling.

## Findings

No unresolved implementation findings.

Resolved during execution:

- Initial workspace run failed synthetic WB13 publication probes because
  `snow.runtime_depth_m` became a required publication input. The shared probe
  seed was updated with neutral snow depth, preserving the production
  fail-closed requirement while allowing unrelated guard tests to exercise their
  intended symbols.

## Disposition

- `Snow-Water` remains SWE and is not used as snow-depth control.
- `Snow-Depth` is populated from existing runtime depth only; no snow/frost
  physics math changed.
- The five-site rerun clears the missing modeled-depth diagnostic blocker but
  does not authorize frost-physics tuning because no site passes paired
  snow-depth control.
