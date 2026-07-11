# Gate Results

Status: `PASS`

Evidence mode: `Ran`

Runner: delegated `comparator_suite_runner`

## Final Gate Ledger

| Gate | Exact command | Result | Evidence |
|---|---|---|---|
| Source/test/contract cleanliness | `git diff --quiet HEAD -- crates tests docs/specifications/science-contracts/contracts` | PASS | exit `0`; no tracked change beyond current HEAD |
| Formatting | `cargo fmt --check` | PASS | exit `0`; wall `2.01 s` |
| Workspace clippy | `cargo clippy --workspace --all-targets -- -D warnings` | PASS | exit `0`; wall `1.72 s` |
| Exact watershed release build | `cargo build --release -p openwepp-runner --bin openwepp-cli-watershed` | PASS | exit `0`; wall `0.26 s` |
| Exact release sanity suite | `OPENWEPP_W11C_WATERSHED_CLI=/home/workdir/openWEPP/target/release/openwepp-cli-watershed cargo nextest run -p openwepp-runner --test mt3_hbp_hourly_consumer_contract --no-capture` | PASS | `7/7`; nextest `2.553 s`; wall `3.13 s` |
| Erosion profile, accepted rerun | `cargo nextest run --workspace --profile erosion` | PASS | `319/319`, `3` slow, `1,377` skipped; nextest `149.930 s`; wall `150.91 s` |
| Full workspace | `cargo nextest run --workspace --profile full` | PASS | `1,693/1,693`, `4` slow, `3` skipped; nextest `585.941 s`; wall `587.07 s` |
| Dependency policy | `cargo deny check` | PASS | exit `0`; wall `1.26 s`; advisories, bans, licenses, and sources all `ok` |

Release-binary identity and the exact consumer binding are recorded in
`artifacts/release-binary-provenance.md`.

## Exact-Release Sanity Metrics

The exact release suite emitted `15` `W11C_RESULT` rows and `4`
`W11C_TIMESTEP` rows. It emitted no `W11C_FINDING` row.

- KW/CREAMS zero controls were exact zero.
- Across printed KW rows, terminal storage was finite and nonnegative in
  `[0, 110.260168179987943] m3` and peak/input ratio was at most `1.0`.
- The largest terminal volume was `7200.000000000010004 m3` for `7,200 m3`
  external input, inside the asserted `1e-9 m3` roundoff tolerance.
- Maximum printed absolute channel-balance residual was approximately
  `1.779e-12 m3`; maximum absolute sediment residual was approximately
  `4.83e-13 kg`.
- Uniform KW retained authorized steady hydraulic storage
  `10.168594800427801 m3`; the initial-storage-aware public ledger, rather
  than raw `input - outlet - final storage`, is the closing quantity.
- `W11E-F001` is reproduced in the four timestep rows: from the 3,600-second
  to 600-second grid, early-spike peak changes
  `0.999951840 -> 1.999993817 m3/s`, late-spike peak changes
  `0.992440232 -> 1.999993817 m3/s`, and late storage changes
  `65.473952630 -> 110.260168180 m3`. These results remain finite,
  nonnegative, passive, volume-bounded, and ledger-consistent, so this is the
  accepted Medium classification/evidence finding rather than a demonstrated
  canonical defect or physical timestep-convergence claim.
- CREAMS serial publication selected topology-terminal element `2` and
  published `7,200 m3` runoff plus `240 kg` sediment rather than internal
  throughflow aliases.
- Both nonzero admitted 60-second static/dynamic MC routes executed with finite
  passive outputs and closed balance.
- All `16` active inadmissible MC cases retained typed
  `WKERNEL-WS10-CHANNEL-E-003` rejection before publication; `4` MC zero
  controls executed normally.
- The canonical three-record `nchnum=0` case retained `dtchr=600`, `ntchr=144`,
  no selected channel IDs, and output-disabled semantics without default
  aliasing.

All heavy gates pass. Package classification remains
`SANITY-PASS-WITH-FINDING` because the exact release run reproduces
`W11E-F001`. Absence of a `W11C_FINDING` row reflects the existing test's
narrower finding predicates and does not erase the material timestep response.

## Superseded Erosion Red Run

The first erosion-profile attempt is retained as evidence, not counted as the
accepted gate:

- Command: `cargo nextest run --workspace --profile erosion`.
- Result: `318/319` passed, `1` failed, `3` slow, `1,377` skipped; nextest
  `151.896 s`, wall `152.91 s`, exit `100`.
- Failed test:
  `wshedw7r_p102_sediment_active_fixture_publishes_nonzero_sediment_and_jobs_identity`.
- Direct child stderr:
  `CLIHILL-E-014 release metadata failure: RELMD-E-001 io error at
  /home/workdir/openWEPP/target/debug/openwepp-cli-hill (deleted): No such file
  or directory (os error 2)`.

This is an executable-path race, not a routing assertion: the spawned hillslope
process found its running debug binary unlinked while release metadata tried to
hash it. The test's generated HBP/pass files existed, but the child correctly
exited nonzero. After the shared Cargo window was made exclusive:

1. the isolated p102 test passed `1/1` (nextest `29.878 s`, wall `30.43 s`), and
2. the complete erosion profile passed `319/319` (nextest `149.930 s`, wall
   `150.91 s`).

No code, fixture, contract, threshold, or assertion was changed to obtain the
accepted rerun.

## Provenance and Scope Audit

- Gate-start W11D source baseline:
  `21f2844a1ee4ebcc265477a716da54c494dd6e89`.
- Current HEAD after unrelated documentation-only work:
  `592df2f11eeef1c13aa346cee794921cb6b64cef`.
- The HEAD advance adds an unrelated audit and changes only
  `docs/dev-guide/06-history-and-performance.md`; it changes no source, test,
  fixture, or science contract.
- `git diff --quiet` passes for `crates/`, `tests/`, and the science-contract
  tree relative to current HEAD and across the two commit IDs.
- W11E itself modifies no Rust, test, fixture, or contract file. Concurrent
  roadmap/catalog/package work and the explicitly excluded dev-guide/audit
  paths were preserved untouched by this runner.

## Disposition

Every assigned current-scope heavy gate has direct final PASS evidence. The
superseded infrastructure red is diagnosed and independently recovered without
a semantic change. Heavy-lane disposition: `PASS`.
