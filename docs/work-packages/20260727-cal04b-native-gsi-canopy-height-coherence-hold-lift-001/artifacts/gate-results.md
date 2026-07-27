# Gate Results

Status: `PASS`

Evidence class: `Ran + Static`

## Passing gates

- `cargo deny check`: `PASS` — advisories, bans, licenses, and sources.
- `cargo fmt --all -- --check`: `PASS`.
- warnings-denied all-target Clippy for orchestrator and runner: `PASS`.
- contract/state/consumer focused suites: `PASS`, including real GSI
  transitions, transactional rollback, native production traversal, exact
  active/shadow Lane D seam evidence, and legacy erosion non-regression.
- final frozen native proof: `PASS`, 12/12 cases; 11 bit-exact 16,437-day
  production cases and one typed-failure/trace-absent case. Artifact:
  `native-consumer-proof.csv`, SHA-256
  `df1a5bf0f0e36e34ba03338460ab9555cd63f74780d48b149696bb602029855a`.
- `cargo nextest run --workspace --profile full -E
  'not package(openwepp-assurance) & not
  binary(/assurance_(v2|dossier)/)'`: `PASS`, run
  `cd7e4f55-4bf1-4f26-b1be-f8997e1e4fc1`, 2,180/2,180 passed, 41 skipped.
- `bash tools/release/check_authority_suite_antievasion.sh`: `PASS`.
- `cargo nextest run --test
  auth11_required_suite_obligation_guards_contract`: `PASS`, run
  `b971ade6-4a0f-4d50-81f4-ff309a1571f0`, 3/3 passed.
- SC-PLANT binding exposure and unit compliance: `PASS`.
- checker/verifier Python tests: `4 passed`.
- frozen CAL-04B executor validator: `PASS`, 9,261 candidates, 27,783
  saturation rows, 18 commands.
- Markdown lint: package 26 files and SC-PLANT one file, zero findings.
- `git diff --check`: `PASS`.

Coverage/CRAP disposition: `DEFERRED_TO_QUALITY_CI` per ADR-0041.

## Historical blocking gate and closure

`cargo nextest run --workspace --profile full`:

- exact-head `FAIL`, run `497732f4-6d9c-41a0-b55d-ca5f871e98d0`;
- 2,292 tests run: 2,229 passed, 63 failed, 43 skipped;
- all 63 failures are assurance crate/contract tests; every observed failure
  resolves to
  `generated identity member changed:
  tests/fixtures/cancov_forest/README.md`, the equivalent identified-source
  SHA-256 mismatch, or the dependent report being not current;
- the fixture README SHA-256 is
  `b81fbe2efa5624e5018c18f24c55ada53d7c484ff020b19d6fa1deae8bd1dd7b`;
- `assurance/v2/identity.lock.json` binds predecessor hash
  `703a138076900f24a3232457dfab8744e60f69ab196b4b361eeb12bbfedb268c`;
- the same mismatch exists at authenticated package base `f4b3db6c` and was
  introduced by earlier commit `502dd745`; this package changed neither file.

An earlier unfiltered run
`0185c3f5-c8ec-4bfa-9649-7d128e533de5` also exposed two package regressions:
the owning direct-frame size bound and legacy P61 erosion behavior. Those
findings were accepted and fixed before `bdeaa2b2`. The subsequent 2,180-test
non-assurance pass and the exact-head 2,229 passing tests prove those failures
do not remain; the exact-head unfiltered run contains no frame-size, P61,
canopy, runner, or orchestrator failure.

The compact retained evidence is
`artifacts/full-workspace-current-failure-summary.md`; it records the exact run
counts, current JUnit checksum, failure ownership, and source-hash provenance.

The separately authorized assurance lifecycle and TESTGATE historical-root
packages corrected that external blocker without manual hash editing or
registry retargeting.

The comparator-owned exact closure-candidate rerun at
`3e78dedbd3b0f0a2c3e1e6d7d90bf625a240ddfd` passed:

- `cargo nextest run --workspace --profile full`: 2,301/2,301 passed,
  43 skipped, zero failures;
- run `7e79049d-0871-4142-a9f7-86ac7ac714be`;
- cargo-deny and warnings-denied all-target Clippy for the owning assurance and
  gate-planner crates also passed at that exact head;
- retained log:
  `/tmp/openwepp-testgate-full-3e78dedb-20260727T163100Z/full-gate.log`.

The mandatory critical terminal gate is no longer blocked. Dual terminal
verification passes.
