# ASSURE-03 Post-Review Heavy Gate Runner

Evidence class: `Ran`

Terminal status: `PASS` for the renewed r4 validation and release-transition
aggregates on the exact post-`VB-001` source freeze described below.

Qualification boundary: this is transition-route verification only. It is not
a conformant release-candidate or release-qualification result because both
aggregates explicitly skipped stability and the temporary release tree is not
a retained release artifact.

Frozen `HEAD` and adjudicated-CRAP base:
`3352388465f8b288aed4636e8f9752ca6c1cceb9`.

## Prior Chronology

Only r4 is terminal evidence for the current tree.

- The first post-review attempt stopped at Clippy, exit `101` after 5.596
  seconds, on `clippy::too_many_lines` in
  `transition_preflight_separates_validation_from_release`. The finding was
  remediated before later attempts.
- A later audit was intentionally interrupted, exit `100` after 301.247
  seconds, when a security fix invalidated its source freeze. It produced no
  terminal claim.
- The r3 transition aggregates passed on their then-current freeze. Subsequent
  verification and `VB-001` remediation changed the terminal source content,
  so r3 is historical evidence only.
- This r4 run established a new freeze after `VB-001`, recreated retained
  validation evidence, and ran both requested aggregates. Both exited `0`.

## Terminal R4 Source Identity

Package `docs/work-packages/**/artifacts/**` paths were excluded because they
are the authorized evidence outputs. Before r4, the runner recorded:

- porcelain-v2 status: 67 rows, 10,135 bytes, SHA-256
  `38c55a522f7464ec6cacb93411687e40118248fa341d366b6d335d930b02e4f0`
- full-index binary diff from `HEAD`: 439,133 bytes, SHA-256
  `d60c66de0a040fd1a241773c336144fd26698a655014074ee0efbdc82ff77a49`
- present changed/untracked path list: 40 paths, SHA-256
  `ddaeb9d0beeef73ff53782e68292a4db127ccda2d76e6c37f2cd9c86922b202a`
- ordered SHA-256 manifest of those 40 files: SHA-256
  `a5355bf907b0e23efae776ba3c464404e21e6c2f669d4ff1a39a00008c6248b8`

Selected release- and remediation-relevant file identities were:

- `tests/integration/assurance_dossier_build_contract.rs`:
  `79c0b36631b6a70d69c7666cc4f95d41a2620fdffdf668d903111bcdecfba230`
- `tools/release/run_release_candidate_gates.sh`:
  `edffa4cf872ee3f972b50104901b2a334b6addf2093465fc19d58eaad2e9ad64`
- `.github/workflows/release-gates.yml`:
  `ebc17a4566e9adce709deb9aecdd82419bc050f7662522fe8f8c4936419c8a04`

No competing release, coverage, Nextest, Clippy, or Cargo test process was
present when the freeze was established. After the validation aggregate and
again after the release aggregate, status, full-index diff, path list, and
complete file-content manifest each compared byte-identical with the freeze.
The runner made no source, test, workflow, governance, package, public-surface,
or exception-registry edit.

The tree is dirty relative to `HEAD`; binary sidecar `source_commit` fields
therefore identify the base commit but not the complete build source. The diff
and ordered content manifest above bind the additional r4 source content.

## Retired-Shell Preflight Verification

Before running either aggregate, the runner directly checked that these retired
filesystem shells were absent, including symlink-aware existence:

- `assurance/dossiers`
- `assurance/methods`
- `assurance/schemas`
- `usersum/assurance/snow-snotel-swe-depth-density.md`

The real release transition preflight then passed once during the validation
route's export check and twice during the explicit release route, each with
`publication_state=v1_retired_zero_reports` and `reports=0`. The validation
preflight also passed with `assembly_authorized=false`. This is direct evidence
that removing the empty retired filesystem shells admits the intended clean
zero-report state without weakening nonempty or special-file rejection tests.

## Route 1: Renewed Validation Aggregate

Command:

```text
bash tools/release/run_release_candidate_gates.sh --mode validate --release-dir docs/work-packages/20260714-assure03-v1-retirement-zero-report-001/artifacts/validation-evidence --skip-stability --crap-base-ref 3352388465f8b288aed4636e8f9752ca6c1cceb9
```

Result: `PASS`, exit `0`, 2810.786 seconds.

Assurance validation and check passed with zero reports. The export check
passed with zero reports, zero documents, and vendoring unauthorized.

Gate results:

- `cargo fmt --check`: `PASS`
- workspace Clippy with warnings denied: `PASS`
- full-profile Nextest: `PASS`
- `cargo deny check`: `PASS` for advisories, bans, licenses, and sources
- 17 adjudicated-CRAP contract tests: `PASS`
- fresh adjudicated-CRAP acquisition: `PASS`
- seven required authority-suite invocations: `PASS`, 12 tests total

Full-profile Nextest identity:

- run UUID: `35e07ed8-ee99-4b26-89ef-2d675b5adb1d`
- tests: 1974; failures: 0; errors: 0
- JUnit time: 582.025 seconds
- JUnit SHA-256:
  `44d8ade0c210b88f7b319d1688964eff54d6140b01381d1f44dbf1d07754bff7`

Fresh adjudicated-CRAP result:

- acquisition: `fresh`; status: `PASS`; closure eligible: `true`
- threshold: 30; debt status: `PASS`
- production entries: 8422; production source files: 222
- raw over-threshold rows: 2; adjudicated: 2; actionable: 0
- touched production files: 13; touched actionable: 0; untouched actionable: 0
- production-source-manifest SHA-256:
  `3a28ecde0c65f38b55b10cb58b5e0967ac82a88a013b0ee082cba08b4280a0e2`
- LCOV SHA-256:
  `4a4dad862b50d3de3bfa6dd748ff5818696ced7898b23f4cc71f1cb8aa6b18a1`
- raw CRAP JSON SHA-256:
  `f093a86c129415309fefd99d41d25998a372ac620df185833a80174b29da3fe5`
- adjudication-registry SHA-256:
  `10b19679e382ebacd6b2d20ee02144c461e01b1ac958731d07dd6585acb7d67f`
- report SHA-256:
  `01dd7f1f9d9d54f6a10f6b2844aaddd9b89bb5a6e42fe266aa362bcdc6f4b291`
- run-status SHA-256:
  `8930517bf400ed347b54f18171081f85fa6ef51173dce55c7ce6548bcee52842`
- acquisition interval: `2026-07-15T10:39:27Z` through
  `2026-07-15T11:16:21Z`
- all 16 CRAP checksum-manifest entries verified

The checksum failure printed within
`soilauth03_injected_drift_vectors_fail_guards` is the required injected
negative vector. The enclosing test passed and the canonical fixtures then
verified cleanly.

The recreated retained validation evidence contains 19 files totaling
8,201,956 bytes. Its path-sorted per-file hash manifest has SHA-256
`cc2b394362b03b2f78e68e1b2681e220d3fffa8544c79adc3228ed6b3d3019cd`.
Additional retained identities are:

- authority results:
  `dd989b1d0067886d1ded66bb8048d7ab6c9cde1e0e5d2677ce8dc4543ef1aa56`
- cargo-nextest version:
  `b94f9fca6aa62c8d95f088fbde71d75f1aa2796bf4ab5715320def301eb08f85`

## Route 2: Renewed Release Transition Exercise

Command:

```text
bash tools/release/run_release_candidate_gates.sh --mode release --release-tag 260714assure03r4 --release-dir /tmp/openwepp-assure03-release-evidence-r4 --skip-stability --crap-base-ref 3352388465f8b288aed4636e8f9752ca6c1cceb9
```

Result: `PASS`, exit `0`, 2828.559 seconds.

Formatting, workspace Clippy, full-profile Nextest, dependency policy, all 17
adjudicated-CRAP contract tests, fresh CRAP, and all seven required authority
invocations passed. The authority suite again ran 12 tests. The aggregate then
created the zero-report snapshot, built both release binaries, emitted
sidecars, linted the release directory, and reported release automation passed.

Full-profile Nextest identity:

- run UUID: `e3208b83-1287-4723-be48-ef6b600bf5fd`
- tests: 1974; failures: 0; errors: 0
- JUnit time: 594.001 seconds
- JUnit SHA-256:
  `9b82699bb5bf2c6d9ebc51fa188c199ae2c1a8785d8b076b3d1717bbf80ffb6a`

Fresh adjudicated-CRAP result:

- acquisition: `fresh`; status: `PASS`; closure eligible: `true`
- threshold: 30; debt status: `PASS`
- production entries: 8422; production source files: 222
- raw over-threshold rows: 2; adjudicated: 2; actionable: 0
- touched production files: 13; touched actionable: 0; untouched actionable: 0
- production-source-manifest SHA-256:
  `3a28ecde0c65f38b55b10cb58b5e0967ac82a88a013b0ee082cba08b4280a0e2`
- LCOV SHA-256:
  `68a6c7d84eae59f4ed970d70eec7bee05843f24624f7bdca60a5a9a6803ec789`
- raw CRAP JSON SHA-256:
  `f093a86c129415309fefd99d41d25998a372ac620df185833a80174b29da3fe5`
- adjudication-registry SHA-256:
  `10b19679e382ebacd6b2d20ee02144c461e01b1ac958731d07dd6585acb7d67f`
- report SHA-256:
  `1e6890468cea8710614e04500aca37881472b92c2b0dd254178c2539b639f21f`
- run-status SHA-256:
  `08a6475f16889cbb0d8b2e6b648ed41090ddf3008b62a609a566ceac719f2c48`
- acquisition interval: `2026-07-15T11:27:08Z` through
  `2026-07-15T12:04:08Z`
- all 16 CRAP checksum-manifest entries verified

### Zero-Report Snapshot

Snapshot ID: `260714assure03r4`.

The schema-v2 manifest has publication state `v1_retired_zero_reports`, report
count 0, an empty `reports` array, and exactly these two file records:

- `assurance/generated/wepppy-usersum.yaml`:
  `08b21cbe20ed059eb61f01f9965d8603779501bde90a728bc7a6c138a69258eb`
- `usersum/assurance/README.md`:
  `65115fe549cbee3107a120f2719b45c00ca2e63b49ebc5c6fc2d7ea350a3cb70`

Snapshot-manifest SHA-256:
`d1f613ab0a1b47d3012fbc2edb55f7492485b2b9d9a5b1cd4a20dc3cc0e8f16f`.

### Binaries And Sidecars

- `openwepp_260714assure03r4`: binary SHA-256
  `ebf5bbc9e9ef3b327fb85c01d61626610401372de37a620b9c6c09d59d795d33`;
  sidecar SHA-256
  `f8657c1ed5c1d2bafbbe73cd002180999f56295bd50f1992afc8ec302848dbc1`
- `openwepp_260714assure03r4_hill`: binary SHA-256
  `7ed022035a38cc9e74b544e0e9e3033f24beded009ec48f6a7c384608382468b`;
  sidecar SHA-256
  `dd2407d2d544eda3f279d5715a926cb48c54555f7463bf2c6c241ec5b67b7582`

Each actual binary hash equals its sidecar `sha256` field. Both sidecars name
source commit `3352388465f8b288aed4636e8f9752ca6c1cceb9`, have
`validation.schema_valid=true`, and record release lint level `contract_v1`.

Before cleanup, the temporary release tree contained 28 files totaling
28,317,386 bytes. SHA-256 over its path-sorted manifest of per-file SHA-256
records was
`8bf28c5aa32c79deea028627d1dba86c720af11fd6e6fcf70b8c7cf5a06c5e48`.

## Cleanup And Disposition

The temporary release tree was fully identified above and then deleted.
`/tmp/openwepp-assure03-release-evidence-r4` was confirmed absent. After
cleanup, status, full-index diff, path list, and the complete ordered file
manifest again compared byte-identical with the initial r4 freeze.

Terminal disposition: the post-`VB-001` r4 freeze passes both requested
transition-route aggregates, including the fresh adjudicated CRAP closure gate
at threshold 30. Stability remains unexecuted by explicit instruction. This
supports route integration and zero-report retirement behavior; it does not
support a conformant candidate, retained release, or release-qualification
claim.
