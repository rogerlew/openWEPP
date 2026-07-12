# High-B Start Metrics

Evidence class: **Ran**

## Source State

- Repository: `/home/workdir/openWEPP`.
- Commit: `34a3f1abdf131bf1d7bcff450b8b62629adcf045`.
- Branch: `main`.
- `git status --short` was empty before and after measurement.
- The delegated `comparator_suite_runner` made no edit and performed no retry.

## Exact Commands And Results

The binding protocol was expanded with `slug=hb` and `phase=start`: clean the
workspace coverage profile, run separately timed workspace LCOV and JSON with
`--ignore-run-fail`, generate CRAP from LCOV, hash/size the artifacts, and apply
the exact production-over-30 filter. Logs and time reports remain under
`/tmp/openwepp-cqr-preint-hb-start-{lcov,json,crap}.{log,time}`.

| Step | Exit | Elapsed | Max RSS |
| --- | ---: | ---: | ---: |
| Clean | 0 | included before LCOV | not recorded |
| LCOV | 0 | 35:26.51 | 830,072 KB |
| JSON | 0 | 35:17.54 | 826,912 KB |
| CRAP | 0 | 1.15 s | 205,052 KB |
| Hash, size, exact filters | 0 | negligible | not recorded |

The LCOV run reported 1,830 passed, five failed, and three ignored; JSON
reported 1,831 passed, four failed, and three ignored. Both had 476 filtered
tests across 175 result sets.

## Artifact Integrity

| Artifact | Bytes | SHA-256 |
| --- | ---: | --- |
| `/tmp/openwepp-cqr-preint-hb-start.lcov` | 4,444,305 | `39eb4d51a4d594ab693b43525dfe9d7950d9d2e65d7558e82429dfbbf01eda47` |
| `/tmp/openwepp-cqr-preint-hb-start.json` | 19,494,803 | `11e1149f9d6fed1d02cec3d2ae56f5b63c1daa3919cf1b146922e851164f5a28` |
| `/tmp/openwepp-cqr-preint-hb-start-crap.json` | 2,905,724 | `2b6a9f169b59365408facfc4f5fe22bf9ae2d4eff3d103a07d616ed7661f0079` |
| `/tmp/openwepp-cqr-preint-hb-start-production-over30.json` | 12,261 | `df4163e492b733cd1b1722186991bd0c5afbe0017cea564015cf1c7ce55c3d7c` |
| `/tmp/openwepp-cqr-preint-hb-start-hb-target-rows.json` | 5,194 | `eec587b652f7aa5710f76be69d06f6bdde96ad84b10ef142f32b660cf40c750f` |

The exact production census is 54 rows across 35 modules. The exact High-B
filter contains 21 rows across all ten fixed target files.

## Ignored-Run Failure Attribution

Both formats reproduced the known shared-environment H2637 active/conflict
family and the known parallel R3C audit-counter assertion. LCOV additionally
observed `h2637_legacy_shadow_fails_closed_without_routing_coefficients`; this
is the same shared-environment family and was not rerun. The H2637 test source
SHA-256 remains `e6a8b65fe0fe02951a0751fbc6c187dbd75bec189ac049e5a39deea917edcd9d`;
the R3C test source remains
`9117d2ff4e0a0d9ecc5f30ae1fe1dfd2aecee28574fbe3dea5aed034a9ddaf7c`.
Neither failure family is in a High-B target source, and no target-related or
unattributed failure occurred.
