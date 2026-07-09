# CRAP After

Evidence label: Static/Ran.

Status: `COMPLETE`

After command:

- `cargo crap --workspace --lcov /tmp/openwepp-cqr-nightly-05-helpers-focused.lcov --min 0 --format json --output /tmp/openwepp-cqr-nightly-05-helpers-focused-crap.json`
  - exit `0`.

Target module:
`crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/helpers.rs`

Summary:

- Unique target functions reported: `21` (`18` production functions plus `3`
  test-only helper functions).
- Production functions above CRAP `30`: `0`.
- All reported target functions above CRAP `30`: `0`.
- Max target CRAP: `19.023147604437927`.
- Total excess above `30`: `0`.

Rows above CRAP `30`: none.

Production function rows after:

| Function | Line | CC | Coverage | CRAP |
|---|---:|---:|---:|---:|
| `Ws10ChannelImpoundmentKernel::missing_required` | `9` | `1.0` | `100.0` | `1.0` |
| `Ws10ChannelImpoundmentKernel::non_finite` | `20` | `1.0` | `100.0` | `1.0` |
| `Ws10ChannelImpoundmentKernel::domain_violation` | `33` | `1.0` | `100.0` | `1.0` |
| `Ws10ChannelImpoundmentKernel::impoundment_outflow_at_stage` | `46` | `5.0` | `97.36842105263158` | `5.000455605773436` |
| `Ws10ChannelImpoundmentKernel::impoundment_drop_spillway_outflows` | `87` | `6.0` | `100.0` | `6.0` |
| `Ws10ChannelImpoundmentKernel::impoundment_culvert_family_outflows` | `111` | `11.0` | `100.0` | `11.0` |
| `Ws10ChannelImpoundmentKernel::impoundment_rockfill_outflow` | `166` | `5.0` | `100.0` | `5.0` |
| `Ws10ChannelImpoundmentKernel::impoundment_emergency_spillway_outflow` | `191` | `4.0` | `100.0` | `4.0` |
| `Ws10ChannelImpoundmentKernel::impoundment_filter_fence_outflow` | `209` | `3.0` | `100.0` | `3.0` |
| `Ws10ChannelImpoundmentKernel::impoundment_perforated_riser_outflows` | `224` | `7.0` | `96.42857142857143` | `7.002232142857143` |
| `Ws10ChannelImpoundmentKernel::impoundment_validate_total_outflow` | `253` | `3.0` | `100.0` | `3.0` |
| `Ws10ChannelImpoundmentKernel::impoundment_area_at_stage` | `276` | `3.0` | `100.0` | `3.0` |
| `Ws10ChannelImpoundmentKernel::impoundment_continuity_rate` | `292` | `8.0` | `100.0` | `8.0` |
| `Ws10ChannelImpoundmentKernel::impoundment_rk4_step` | `336` | `8.0` | `83.01886792452831` | `8.31338621815324` |
| `Ws10ChannelImpoundmentKernel::crosses_threshold` | `392` | `4.0` | `100.0` | `4.0` |
| `Ws10ChannelImpoundmentKernel::impoundment_crosses_regime_transition` | `396` | `3.0` | `100.0` | `3.0` |
| `Ws10ChannelImpoundmentKernel::integrate_impoundment_stage_with_adaptive_retry` | `410` | `16.0` | `77.22772277227723` | `19.023147604437927` |
| `Ws10ChannelImpoundmentKernel::route_impoundment_stage_over_duration` | `523` | `9.0` | `81.35593220338984` | `9.524936824115416` |
