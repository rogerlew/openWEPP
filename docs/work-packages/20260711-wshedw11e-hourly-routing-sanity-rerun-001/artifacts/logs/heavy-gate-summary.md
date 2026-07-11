# Heavy Gate Summary

Status: `PASS`

Evidence mode: `Ran`

Runner: delegated `comparator_suite_runner`

## Final Results

| Gate | Result | Metrics |
|---|---|---|
| `cargo fmt --check` | PASS | exit `0`; wall `2.01 s` |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | exit `0`; wall `1.72 s` |
| Exact release build | PASS | exit `0`; wall `0.26 s` |
| Exact release consumer | PASS | `7/7`; nextest `2.553 s`; wall `3.13 s` |
| Erosion profile | PASS | `319/319`; `3` slow; `1,377` skipped; nextest `149.930 s`; wall `150.91 s` |
| Full profile | PASS | `1,693/1,693`; `4` slow; `3` skipped; nextest `585.941 s`; wall `587.07 s` |
| `cargo deny check` | PASS | exit `0`; wall `1.26 s`; advisories, bans, licenses, and sources `ok` |

## Release Identity

- Path: `/home/workdir/openWEPP/target/release/openwepp-cli-watershed`
- SHA-256:
  `f82cc9fa539d26cdf9a6797d3e272bca22a7a19dc4b9988a3a95e7cd4c38d792`
- Consumer command bound `OPENWEPP_W11C_WATERSHED_CLI` to that absolute path.

## Superseded Erosion Attempt

The first erosion run produced `318/319` passed, `1` failed, `3` slow, and
`1,377` skipped (nextest `151.896 s`; wall `152.91 s`; exit `100`). The child
failed while hashing its running executable:

`CLIHILL-E-014 release metadata failure: RELMD-E-001 io error at
/home/workdir/openWEPP/target/debug/openwepp-cli-hill (deleted): No such file
or directory (os error 2)`

No semantic assertion failed. With an exclusive Cargo window, the isolated
p102 consumer passed `1/1` (nextest `29.878 s`; wall `30.43 s`), followed by
the accepted complete erosion rerun at `319/319`. No code, fixture, contract,
threshold, or assertion changed between the red run and recovery.

Detailed evidence remains in `../gate-results.md` and
`../release-binary-provenance.md`.
