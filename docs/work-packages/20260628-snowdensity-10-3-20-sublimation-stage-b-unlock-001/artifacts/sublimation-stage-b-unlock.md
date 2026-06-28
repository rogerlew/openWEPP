# SNOWDENSITY-10.3.20 Sublimation Stage B Unlock

Evidence mode: `Static + Ran`.

- Disposition: `NON-PROMOTION-GATE-NOT-MET`
- Current default robust fail/score: `15` / `179`
- Composition robust fail/score: `19` / `168`
- Stage B robust fail/score: `15` / `178`
- Stage B primary gate pass: `False`
- Stage B conservation gate pass: `True`
- Stage B bidirectional guardrail pass: `False`
- Activation authorized: `False`

## Candidate Gates

| Candidate | Primary | Guardrail | Conservation | Promotion | Better cells | Worse cells |
|---|---:|---:|---:|---:|---:|---:|
| `partition_sublimation_stage_a` | False | False | True | False | 1 | 8 |
| `stage_b_surface_layer` | False | False | True | False | 1 | 3 |

## Provenance

- libsnobal clone commit: `bf8b41c71e3e54ae654ae04005ddf72566c47ee6`
- `setup.py` CC0 declaration captured in JSON artifact.
- Default, rollback, fixtures, output schema, density cap, frost, parser/runfile/user CLI, and `.run` controls unchanged.
