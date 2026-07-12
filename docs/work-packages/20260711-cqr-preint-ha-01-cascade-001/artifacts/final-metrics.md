# Supplemental Workspace Metric Provenance

Evidence class: **Ran**

Under the revised campaign cadence this is supplemental evidence, not a module-
checkpoint requirement. High-A final will create the one authoritative coherent
tranche-final workspace metric set.

## Source

- HEAD: `c0a75d8e4e0975ac0bfc9e3896af6b8539453cf7`
- `cascade.rs`: 709 lines, SHA-256
  `574d98ab6708c9332a6ddef3adc35df843f6cec3a00a05c80c1f5f042ab1d3fb`
- The delegated runner made no tracked edit.

## Commands

```text
cargo llvm-cov --workspace --ignore-run-fail --json --output-path /tmp/openwepp-cqr-preint-ha01-final.json
cargo llvm-cov clean --workspace
/usr/bin/time -v -o /tmp/openwepp-cqr-preint-ha01-final-retry1-lcov.time cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path /tmp/openwepp-cqr-preint-ha01-final-retry1.lcov
/usr/bin/time -v -o /tmp/openwepp-cqr-preint-ha01-final-retry1-crap.time cargo crap --workspace --lcov /tmp/openwepp-cqr-preint-ha01-final-retry1.lcov --min 0 --format json --output /tmp/openwepp-cqr-preint-ha01-final-retry1-crap.json
jq '[.entries[] | select(.file | startswith("/home/workdir/openWEPP/crates/")) | select(.file | contains("/src/")) | select((.file | contains("/src/tests/")) | not) | select(.crap > 30) | {file:(.file | sub("^/home/workdir/openWEPP/"; "")), function, line, cyclomatic, coverage, crap}] | unique_by([.file,.function,.line,.cyclomatic,.coverage,.crap]) | sort_by(.file,.line,.function)' /tmp/openwepp-cqr-preint-ha01-final-retry1-crap.json > /tmp/openwepp-cqr-preint-ha01-final-retry1-production-over30.json
```

All outer commands exited 0.

## Artifacts

| Artifact | Bytes | SHA-256 | Elapsed |
| --- | ---: | --- | ---: |
| `/tmp/openwepp-cqr-preint-ha01-final.json` | 19,118,762 | `eaabd76ed3adcf48dc7d73040ecc8feba88f004a7629b7225ea1286ee723f599` | 34:44.30 |
| `/tmp/openwepp-cqr-preint-ha01-final-retry1.lcov` | 4,378,173 | `5e143335d8e3bb410d208ddad502b905db675556812faf4121aa47196a4bc8ae` | 34:24.11 |
| `/tmp/openwepp-cqr-preint-ha01-final-retry1-crap.json` | 2,870,530 | `74468bd2fb75ad625caf43fb69b4014905a8680bbf94ff0bd27187b2966b9e1b` | 1.11 s |
| `/tmp/openwepp-cqr-preint-ha01-final-retry1-production-over30.json` | 15,311 | `e831466be9ba5ce0744dcdca25f5d318d1439cb55bc086dc445f0885acc8d312` | included above |

## Results And Failure Attribution

Production `cascade.rs` is 207/211 lines (`98.104%`) and 283/288 regions
(`98.264%`); its lowest function floor is 11/12 (`91.667%`). Interpolation
CRAP is 7 and every target row is at most 11.001. The filtered ranking is 66
rows/44 modules; after removing the HA-01 start row, all 66 non-target records
exactly equal High-A start.

The retry's underlying tests returned 101 only for
`-p openwepp --test laned_shadow_h2637`: five passed, three failed, two ignored.
The failures exactly match High-A start:
`h2637_active_fails_closed_without_routing_coefficients`,
`h2637_active_and_disable_are_mutually_exclusive`, and
`h2637_active_and_shadow_are_mutually_exclusive`. This is baseline-known
shared-process environment interference. No other retry target failed and no
non-target CRAP record changed.

The separate supplemental JSON capture had additional shared-process variants:
`active_trace_selector_requires_active_before_outputs`, the legacy-shadow
missing-coefficients case, and the unrelated R4B audit-counter test. Implicated
sources were unchanged; the R4B test passed 1/1 in an exact isolated rerun.
Because this JSON and retry LCOV are not one coherent capture, they are not
tranche-final acceptance evidence. The revised strategy intentionally defers
one coherent exact workspace set and complete failure attribution to High-A
final instead of repeating module-level workspace runs.
