# Execution Incident 004

Status: `TERMINAL HOLD BEFORE POPULATION`

Evidence class: `Ran + Static`

The bounded fourth attempt passed preparation and both builds. Its
`native_default` production case completed all 16,437 fixture days and matched
the independent direct-kernel expectation bit-exactly. The next frozen case,
the representative interior vector `GSI-5557`, failed in the real production
consumer at lane 1, day 11,186:

```text
LAI 0.007687569841550092 > 0 with missing/non-positive post-growth canhgt
```

The error is the fail-closed `laned_active_rev21_operands` guard. It occurred
after 11,185 trace rows and before `native-consumer-proof.csv` could be issued.
The observed command receipt records exit code 1 and state `FAIL`.

Static source review localized the mismatch: the production day builder first
computes baseline legacy growth state, then the generalized-GSI override changes
current foliar biomass, interception biomass, LAI, and cover without recomputing
the current canopy height. On the observed zero-to-positive GSI transition,
positive current LAI is therefore paired with canopy height derived from the
prior zero-foliar state.

The exact source anchors are
`crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00c_day_input_builder_impl.rs`:
`growth_state_for_build` computes the baseline state (current lines 413–426)
and `native_forest_growth_state_for_build` replaces biomass, LAI, and cover but
not height (current lines 600–607). The carried height was computed from
baseline biomass by
`crates/openwepp-hillslope-orchestrator/src/direct_runtime/growth.rs`
(current lines 781–786).

The complete attempt is preserved without reuse at
`/home/workdir/cal04b-objects-native-proof-interior-failure-004`.
Key retained hashes are:

| Object | SHA-256 |
|---|---|
| observed native-proof receipt | `efce0e1d263465ab73944e5bd678d59ee911f43659b405566f88791d608302ec` |
| observed wrapper stderr | `65ae24989244ee985a021e1e89ccc831b08de81d91b35460ae35203d2eb365de` |
| interior production stderr | `79ef70c503c5614d30a5011c0dc8111f44b6b4ab4655ca2c0ba2efb0665e8bda` |
| interior management YAML | `282da9de843af4464e9665a4486a3cdc46321bef063f28d9ebd69e8ed1a51a7a` |
| completed native-default trace | `0fd449b8cb2899fe674aecc19fd3ecc5c2d8a1f579a81e66c021cb230d957671` |

No synthetic, Hubbard population, reconstruction, retention, readiness,
freeze, or holdout command ran. Harvard remains sealed. Candidate substitution,
run truncation, a non-GSI fixture override, or weakening the production guard
would evade the frozen real-consumer claim and is not an admissible correction.
