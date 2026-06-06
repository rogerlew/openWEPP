# WBVAL03 Validation Ledger

Status: executed-hold

Evidence mode: mixed `Static:` and `Ran:`

Ran:

- Current validation used release binary `target/release/openwepp-cli-hill` at
  repository commit `57eed35`.
- Current J-95 targets:

  | Prefix | Current RC | Current result |
  |---|---:|---|
  | `p7` | 1 | `CLIM-RUNTIME-E-017`, `radly=486` |
  | `p11` | 1 | `CLIM-RUNTIME-E-017`, `radly=486` |
  | `p18` | 1 | `CLIM-RUNTIME-E-017`, `radly=486` |
  | `p20` | 1 | `CLIM-RUNTIME-E-017`, `radly=486` |

- Current prior-WAT-emitter targets:

  | Prefixes | Current result |
  |---|---|
  | `p1`, `p3`, `p5`, `p8`, `p10`, `p12`, `p13`, `p15`, `p16`, `p19`, `p21`, `p22` | all fail before WAT publication with `CLIM-RUNTIME-E-017`, `radly=486` |

Static:

- Historical WBVAL01 WAT files remain usable for static balance auditing, but
  they are pre-WBVAL02 artifacts and cannot validate a new WBVAL03 production
  correction.

Acceptance status:

- Four J-95 blockers: not closed; current runs are preempted by upstream
  invalid climate source input.
- Twelve WAT residuals: complete-identity residuals remain real in saved
  WBVAL01 outputs; current reruns cannot regenerate WAT for validation.
- Package outcome: legitimate `HOLD` behind WBVAL04.
