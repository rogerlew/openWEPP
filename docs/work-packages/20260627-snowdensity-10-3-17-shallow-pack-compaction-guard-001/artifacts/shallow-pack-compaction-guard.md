# SNOWDENSITY-10.3.17 Shallow-Pack Compaction Guard

Evidence mode: Static/Ran.

- Disposition: `NON-PROMOTION-SHALLOW-GUARD-GATE-NOT-MET`
- Promotion eligible: `False`
- Activation authorized: `False`
- Candidate trace ok: `True`
- Induced under-persistence reduced: `True`
- Over-persistence not worse: `False`
- Threshold authority ok: `True`
- Snow-state conservation ok: `False`
- Snow-control failures: `498 -> 500`
- Induced under-persistence: `177 -> 176`
- Under-persistence total: `234 -> 233`
- Over-persistence total: `264 -> 267`
- Max SWE-depth-density residual: `1.110e-16 m`
- Max snow-state closure residual: `5.551e-17 m`
- Max mass-term delta: `3.342e-03 m`

## Surface Results

| Surface | Scope | Baseline fails | Candidate fails | Baseline induced under | Candidate induced under | Baseline over | Candidate over |
|---|---|---:|---:|---:|---:|---:|---:|
| `hjandrews_conifer` | observation_blocked | 0 | 0 | 0 | 0 | 0 | 0 |
| `sleepers_south_field` | paired_observation | 150 | 152 | 21 | 21 | 123 | 125 |
| `sleepers_w9_hardwood` | paired_observation | 57 | 57 | 22 | 21 | 30 | 31 |
| `harvard_hardwood` | paired_observation | 153 | 153 | 73 | 73 | 47 | 47 |
| `harvard_open` | paired_observation | 138 | 138 | 61 | 61 | 64 | 64 |
| `hubbardbrook_deciduous` | observation_blocked | 0 | 0 | 0 | 0 | 0 | 0 |
| `hubbardbrook_mixed` | observation_blocked | 0 | 0 | 0 | 0 | 0 | 0 |

## Boundary Disposition

- Candidate remains opt-in diagnostic only.
- Default activation, density cap, public output schema, fixtures, parser/runfile/user CLI, compatibility runtime, Qwet/frzftp, sublimation, two-layer structure, and frost attribution remain unchanged.
