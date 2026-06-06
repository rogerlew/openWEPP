# Pre-Implementation Contract Gate

Status: executed-hold

Evidence mode: mixed `Static:` and `Ran:`

Ran:

- Current release build:
  `cargo build --release -p openwepp-runner --bin openwepp-cli-hill` passed.
- Current J-95 reproduction command pattern:

  ```text
  target/release/openwepp-cli-hill \
    --run-dir /wc1/runs/in/indispensable-presenter/wepp/runs \
    --run-file /tmp/wbval01_rocky_mountain_20260606T000000Z/generated_runfiles_nodiscovery/<p>.toml \
    --output-dir /tmp/wbval03_repro_current/<p> \
    --policy compat
  ```

- Current J-95 target results:

  | Hillslope | Current RC | Current blocker |
  |---|---:|---|
  | `p7` | 1 | `CLIM-RUNTIME-E-017`, `radly=486` |
  | `p11` | 1 | `CLIM-RUNTIME-E-017`, `radly=486` |
  | `p18` | 1 | `CLIM-RUNTIME-E-017`, `radly=486` |
  | `p20` | 1 | `CLIM-RUNTIME-E-017`, `radly=486` |

Static:

- Historical WBVAL01 evidence anchors the original J-95 failures:
  `HKERNEL-WB11-PERC-E-003`, `last_phase=percolation_deep_seepage`,
  `sim_day_index=95`, calendar year `1990`, Julian day `95`.
- Current WBVAL03 implementation gate fails before production-code authority:
  the required validation surface is blocked upstream by WBVAL04.

Gate conclusion:

- Do not amend WBVAL03 contracts, add production fixes, loosen percolation
  guards, or compensate WAT publication in this run.
