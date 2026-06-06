# Pre-Implementation Contract Gate

Status: complete

Evidence mode: mixed `Static:` and `Ran:`

Ran:

- Before-state reproduction command pattern:

  ```text
  target/release/openwepp-cli-hill \
    --run-dir /wc1/runs/in/indispensable-presenter/wepp/runs \
    --run-file /tmp/wbval01_rocky_mountain_20260606T000000Z/generated_runfiles_nodiscovery/<p>.toml \
    --output-dir /tmp/wbval02_repro_before/<p> \
    --policy compat
  ```

- Before-state results:

  | Hillslope | RC | Error symbol | Value |
  |---|---:|---|---:|
  | `p2` | 1 | `winter.hourly.rad_mj_m2_0012` | 4.908100451183912 |
  | `p4` | 1 | `winter.hourly.rad_mj_m2_0012` | 4.915834085837891 |
  | `p6` | 1 | `winter.hourly.rad_mj_m2_0012` | 4.833738717329369 |
  | `p9` | 1 | `winter.hourly.rad_mj_m2_0012` | 4.912717106932223 |
  | `p14` | 1 | `winter.hourly.rad_mj_m2_0012` | 4.829383053400764 |
  | `p17` | 1 | `winter.hourly.rad_mj_m2_0012` | 4.857151679545786 |

- Source radiation ledger from `/wc1/runs/in/indispensable-presenter/wepp/runs/p2.cli`:
  - All six wrappers reference identical DRIGGS climate records.
  - First source-bound violation: `1990-02-18`, day-of-year `49`,
    `radly=486 Ly d^-1`.
  - Baseline `sunmap` horizontal daily potential at latitude `43.73`:
    `r3=453.068716 Ly d^-1`.
  - Ratio: `radly/r3=1.072685`.
  - The file contains `53` rows with `radly > r3`; maximum observed ratio in
    the local scan was `1.1096425162356847`.

Static:

- Mechanism: invalid upstream daily radiation exceeds baseline horizontal
  daily potential before SIMIMPL28 hourly synthesis.
- Ownership: input generation is upstream of this package, but typed
  SIMIMPL28 source-bound evidence is in the package write set.
- Authority: `SC-CLIMATE-001#INV-CLIMATE-006`,
  `SC-CLIMATE-001#INV-CLIMATE-013`, and pinned baseline `sunmap.for`.
- Gate decision: proceed to contract amendment, red tests, and production
  typed-evidence correction; do not loosen the HPHYS0277 hourly guard.
