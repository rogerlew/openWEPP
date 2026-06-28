# SNOWDENSITY-10.3.14 Policy-B No-Regression And Cap Authority

Evidence mode: Static/Ran.

- Disposition: `READY-FOR-ACTIVATION-PACKAGE-UNDER-ACTIVE-CAP`
- Activation policy: `POLICY-B`
- Activation package ready under active cap: `True`
- Active cap: `522.0 kg m^-3`
- Density cap changed: `False`
- 550 cap disposition: `MIXED-FOLLOW-UP-DYNAMIC-RERUN-REQUIRED`
- 550 cap required for activation: `False`
- Cap-pinned paired rows: `248`
- Cap-pinned current/projected failures: `105 -> 102`
- Cap-pinned projected net fail delta: `-3`
- Workspace regression status: `pass`
- Composite trace identity max residual: `1.11022302463e-16 m`
- Trace density cap exceed count: `0`
- Frost attribution blocker: `SNOW-CONTROL-RESIDUALS-REMAIN`
- Next recommended package: `SNOWDENSITY-10.3.15-DEFAULT-ACTIVATION-UNDER-ACTIVE-CAP`

## Policy-B Matrix

| Scope | Status | Evidence |
|---|---|---|
| direct bundle trace proof | `PASS` | SNOWDENSITY-10.3.12 trace proof selected both bundle members. |
| gate-eligible paired-snow improvement versus current default | `PASS` | default 1147 -> bundle 498 |
| paired surface no-worse guard versus holding-only | `PASS` | worse paired surfaces: 0 |
| composite melt-density trace identity | `PASS` | max SWE-depth-density residual 1.11022302463e-16 m; cap exceed count 0 |
| workspace regression/identity with bundle selectors | `PASS` | Recorded from package gate-results. |
| non-snow climate, erosion/WB, and watershed routing suite | `PASS` | Covered by the full workspace test run under package-bound bundle selectors. |
| 550 kg m^-3 cap re-anchor | `FOLLOW-UP` | Projection only; no dynamic runtime cap mutation. Projected cap-pinned fail delta -3. |

## Cap-Pinned Surface Results

| Surface | Rows | Current fail | Projected 550 fail | Pass->Fail | Fail->Pass |
|---|---:|---:|---:|---:|---:|
| `hjandrews_conifer` | 0 | 0 | 0 | 0 | 0 |
| `sleepers_south_field` | 63 | 29 | 26 | 0 | 3 |
| `sleepers_w9_hardwood` | 15 | 9 | 8 | 0 | 1 |
| `harvard_hardwood` | 91 | 33 | 35 | 2 | 0 |
| `harvard_open` | 79 | 34 | 33 | 1 | 2 |
| `hubbardbrook_deciduous` | 0 | 0 | 0 | 0 | 0 |
| `hubbardbrook_mixed` | 0 | 0 | 0 | 0 | 0 |

## Boundary Disposition

- Default activation changed: `false`.
- Production physics changed: `false`.
- Density cap changed: `false`.
- Public output schema changed: `false`.
- Parser/runfile/user selector added: `false`.
- Fixture inputs changed: `false`.
- Frost attribution authorized: `false`.
