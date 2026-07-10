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
| 10 | `crates/openwepp-input-contract/src/parsers/hbp/payload_validator.rs` | 152.000 | 1 | 182.000 | One uncovered high-complexity event-payload parser branch with existing HBP integration-test surface. |

Exclusion note: unrelated untracked files under root `artifacts/` pre-existed
this target and are outside the write set.

