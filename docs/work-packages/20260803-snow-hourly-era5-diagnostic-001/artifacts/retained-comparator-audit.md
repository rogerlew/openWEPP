# Retained Comparator Audit

Ran: the four EB-04W2 source climate fixtures are present and match the hashes
frozen by EB-04W2:

| Site | Climate fixture | SHA-256 | Lines |
|---|---|---|---:|
| Mica Creek | `tests/fixtures/snotel_observed/snotel_mica_creek_st_joe_id/p1.cli` | `e8470ae78711f85cc84045052467fa5d75fc8ec4ca1f92ce49b1af9ecf95fb63` | 14,260 |
| Paradise | `tests/fixtures/snotel_observed/snotel_paradise_wa/p2.cli` | `6e0c874e38825a7f4def18b87d81e61be9c59496a25e5f5affa9d25755db173c` | 16,452 |
| Snowbird | `tests/fixtures/snotel_observed/snotel_snowbird_ut/p8.cli` | `10c1ede130f697ccec01a4fb076d937213f0699e2f6c100492c7a4ef28ec11a7` | 14,260 |
| Niwot | `tests/fixtures/snotel_observed/snotel_niwot_co/p2.cli` | `841d6390b511c3b6ad613e166788fd0b3c48b1d83317779ecd7ba2cfd7916ead` | 16,452 |

Static: the comparator is not native observed-hourly Daymet/gridMET. These
daily climate records are transformed to hourly winter forcing through
`HillslopeClimateRuntimeRequest::diagnostic_winter_hourly_forcing` in
`crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs`
(SHA-256
`011f3195149c133a021821637e1cf2daf3bcd1df1f2e630daa76bd72fd3e068b`).
The operator uses the production SIMIMPL28 solar geometry/radiation,
temperature, and precipitation partition calculations while forcing complete
diagnostic rows. Any result-bearing successor must hash its emitted comparator
rows and preserve the selected calibrated precipitation transformation.

Ran: the target filesystem reported approximately 25.8 TB free, so output
capacity is not the present blocker. Retrieval batching and quota remain
external operational constraints.

