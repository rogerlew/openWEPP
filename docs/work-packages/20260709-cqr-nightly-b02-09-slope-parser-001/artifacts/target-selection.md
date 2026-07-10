# Target Selection

Static: selected from fresh CQR nightly batch 02 baseline measurement.

Baseline LCOV:
`/tmp/openwepp-cqr-nightly-new-isolated.lcov`

- SHA-256:
  `87811062b33fe1c79176843204e01410aca9b2530c09cb60a98e3ec4a2f60cce`

Baseline CRAP JSON:
`/tmp/openwepp-cqr-nightly-new-isolated-crap.json`

- SHA-256:
  `5e3abb273dffdc8c8308da0af9f3d85283307b2b2c77f0c9ba00c3a55741f765`

Selected target row:

| Rank | Module | Excess CRAP | Functions > 30 | Max CRAP | Included reason |
|---:|---|---:|---:|---:|---|
| 9 | `crates/openwepp-input-contract/src/parsers/slope.rs` | 156.908 | 2 | 182.000 | High parser-display and top-level parse CRAP; existing focused integration test surface. |

Batch 02 selected targets:

1. `crates/openwepp-kernel/src/boundary_values.rs`
2. `crates/openwepp-runner/src/direct_runtime/audit.rs`
3. `crates/openwepp-input-contract/src/parsers/soil.rs`
4. watershed CLI target, locally held
5. totalwatsed3 CLI target
6. irrigation depletion parser target
7. sim status target
8. `crates/openwepp-topology/src/lib.rs`
9. `crates/openwepp-input-contract/src/parsers/slope.rs`
10. HBP payload validator target

Exclusion note: unrelated untracked files under root `artifacts/` pre-existed
this target and are outside the write set.
