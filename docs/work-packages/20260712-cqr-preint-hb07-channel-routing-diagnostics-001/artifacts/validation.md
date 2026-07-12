# Terminal Validation

Evidence class: **Ran + Static**

| Gate | Result | Evidence |
| --- | --- | --- |
| DC red | PASS as reproduction | `cx < -10` was admitted by the clamp; focused nextest exited `100`. |
| HB07 focused | PASS | Five selected HB07/DC tests passed. |
| Full orchestrator | PASS | `126/126` |
| W11C real consumer | PASS | `7/7` in `mt3_hbp_hourly_consumer_contract` |
| Module coverage | PASS | Lines `496/595` (`83.361%`), regions `529/610` (`86.721%`), functions `9/9`. |
| Function floor/CRAP | PASS | Zero diagnostics functions below 75%; zero above CRAP 30. |
| Format/Clippy/diff | PASS | All-target orchestrator/runner Clippy with `-D warnings`; format and diff checks clean. |

| Function | CC | Coverage | CRAP |
| --- | ---: | ---: | ---: |
| `ws11_muskingum_geometry_from_depth` | 15 | 92.308% | 15.102 |
| `compute_variable_muskingum_cunge_state` | 25 | 83.750% | 27.682 |
| `ws11_dynamic_muskingum_celerity` | 9 | 92.857% | 9.030 |

## Hashes And Line Governance

| Item | SHA-256 |
| --- | --- |
| `diagnostics.rs` | `7588a8f35961c63e3af5c5dff42f5d3f2d1e06b8844d666a20040e4990a175e5` |
| `direct_tests.rs` | `e29510b2595627b139b058cac40e1426c49194a188fadcd7bc4e0fcf42b4c2e5` |
| Red log | `62e393c060492941edcc534e45111b2003bd68ac67dd4159a831a5bc62d06d80` |
| Coverage JSON | `1bb0ac3f1616a5766d6b2e8f04a447f1bc3edbf5568237146c653deee56770a1` |
| Coverage LCOV | `5bde76f8d7ed579eb2eba1578d40bcc1d77bd1ea53acb1ac6f444565e779f7b8` |
| CRAP JSON | `5c0c9458ce7cabdcac66c91ab8c963c0cbd32add3f4ca9b71eb2f64f1465a794` |

The production target is 648 lines, below WARN. The shared private test module
is 2,297 lines, WARN but below the 3,000-line blocker; it remains the nearest
cohesive kernel test surface.

