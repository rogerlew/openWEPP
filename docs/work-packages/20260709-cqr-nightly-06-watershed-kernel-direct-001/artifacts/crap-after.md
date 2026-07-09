# CRAP After

Evidence label: Static/Ran.

Status: `TARGETED-PASS`

Target module:
`crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/direct.rs`

Targeted after commands:

- `cargo llvm-cov clean --workspace` - exit `0`.
- `cargo llvm-cov -p openwepp-watershed-orchestrator --lib --no-report` -
  exit `0`.
- `cargo llvm-cov --workspace --test wshedw5_typed_watershed_runtime_contract --lcov --output-path /tmp/openwepp-cqr-nightly-06-direct-targeted-final7.lcov --no-clean` -
  exit `0`.
- `cargo crap --workspace --lcov /tmp/openwepp-cqr-nightly-06-direct-targeted-final7.lcov --min 0 --format json --output /tmp/openwepp-cqr-nightly-06-direct-targeted-final7-crap.json` -
  exit `0`.

Summary:

- Deduplicated target functions above CRAP `30`: `0`.
- Max target CRAP after: `23.069544598035826`.
- Baseline deduplicated rows above CRAP `30`: `7`.
- Baseline deduplicated total excess above `30`: `504.9508413968741`.

Top target rows after:

| Function | Line | CC | Coverage | CRAP |
|---|---:|---:|---:|---:|
| `Ws10ChannelImpoundmentKernel::compute_direct_channel_runon` | `313` | `23.0` | `94.91525423728814` | `23.069544598035826` |
| `Ws10ChannelImpoundmentKernel::read_direct_hillslope_sediment_payload` | `1419` | `17.0` | `100.0` | `17.0` |
| `Ws10ChannelImpoundmentKernel::assemble_direct_channel_baseflow` | `1160` | `15.0` | `81.13207547169812` | `16.51131470945814` |
| `Ws10ChannelImpoundmentKernel::assemble_direct_incoming_sediment_load_and_capacity` | `2082` | `14.0` | `79.01234567901234` | `15.811956548328038` |
| `Ws10ChannelImpoundmentKernel::route_direct_impoundment_outflow` | `741` | `11.0` | `69.38775510204081` | `14.471130226351265` |
| `Ws10ChannelImpoundmentKernel::compute_direct_channel_peak` | `386` | `13.0` | `80.64516129032258` | `14.225336511026821` |

Cargo-crap low-coverage note:

- `read_direct_dependency_peak_payload` is now at `100.0%` cargo-crap function
  coverage after frame-backed channel, impoundment, and invalid-kind tests.
- The lowest remaining cargo-crap function coverage row is
  `route_direct_impoundment_outflow` at `69.38775510204081%`. It has direct
  valid-route, non-finite-duration, and negative-continuity guard coverage and
  is below the CRAP risk threshold after decomposition.
- ADR-0021 per-function floor evidence uses llvm-cov deduplicated source-span
  regions and is recorded in `coverage-closure.md`; no denominator exclusions
  or `COVERAGE-EXCLUDE` annotations were introduced.

Full-workspace coverage note:

- `cargo llvm-cov --workspace --lcov --output-path /tmp/openwepp-cqr-nightly-06-direct-workspace.lcov --ignore-run-fail`
  was attempted but stopped after the known coverage-instrumented
  `laned_shadow_h2637` failure/hang. Targeted after metrics therefore combine
  orchestrator unit tests with the watershed runtime integration test that
  exercises this target module's channel and impoundment execution paths.
