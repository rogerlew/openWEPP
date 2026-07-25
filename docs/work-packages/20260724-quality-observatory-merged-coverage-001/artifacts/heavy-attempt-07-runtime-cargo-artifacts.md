# Heavy Attempt 07: Runtime Cargo Artifacts

Evidence class: Ran / Static.

Attempt:
`/home/workdir/openWEPP-quality-history/20260724-order3-local-attempt7-utDWC1`.

Admission ID:
`1eeb10daafd4750748ed85df2e92ae0e7fc2b226a760d3e18aff0419dab28752`.

## Result

The instrumented `full` profile passed all 2,279 tests with 31 configured
skips and 15 slow tests in `2290.882s`. The three attempt-5 exact-checkout
regressions passed.

Collection then failed closed with:
`instrumented build changed while executing full`.

Science-manual, merge, CRAP, snowbench disposition, publication, and terminal
verification did not run. Published files: 0.

## Manifest Delta

The admitted executable manifest contained 280 rows; post-full contained 291.
There were 11 additions, no removals, and no changed admitted artifact:

- the `openwepp-assurance` binary and its dependency executable;
- `serde_derive`;
- paired build-script paths for `proc-macro2`, `quote`, `serde`, and
  `serde_core`.

Assurance publication tests legitimately invoke repository release scripts
that run `cargo run --quiet -p openwepp-assurance`. Nextest inventory builds
the assurance test/library targets but not that runtime binary. The nested
consumer therefore extended the shared instrumented target after admission.

## Correction Intent

Admission will build the exact repository-owned runtime consumer
`openwepp-assurance` under the same instrumented environment and target before
sealing the executable manifest. The runtime artifact declaration is included
in build identity. Full execution must then leave the manifest byte-for-byte
unchanged. A final exact identity check after CRAP/evaluator work and before
PASS publication prohibits all later post-admission growth.
