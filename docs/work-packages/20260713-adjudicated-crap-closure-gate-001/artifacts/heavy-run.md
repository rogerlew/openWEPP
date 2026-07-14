# Heavy Verification Run

Evidence class: **Ran**

Status: `FAIL`

Date: `2026-07-13`

Runner role: package-authorized `comparator_suite_runner`

## Command Results

Commands were run from `/home/workdir/openWEPP` in the required order.

| Order | Exact command | Exit | Result | Observed result |
| ---: | --- | ---: | --- | --- |
| 1 | `bash tools/release/run_adjudicated_crap_gate.sh --base-ref 3071849a0aec2abf8c17fe2405ce468f1533f631 --output-dir /tmp/openwepp-acrap-live-20260713` | 127 | **FAIL** | About 36 minutes; fresh LCOV and CRAP JSON were produced, but the delegated command did not produce a passing adjudicated report. |
| 2 | `cargo fmt --check` | 0 | **PASS** | Completed in 2.014 seconds with no output. |
| 3 | `cargo clippy --workspace --all-targets -- -D warnings` | 0 | **PASS** | Completed in 17.597 seconds with no warnings. |
| 4 | `cargo nextest run --workspace --profile full` | 0 | **PASS** | 1,960 tests passed, 4 slow, and 3 skipped across 180 binaries in 586.222 seconds. Run ID: `0bbe820f-55f6-4f95-8cd7-e4a5ff649984`. |
| 5 | `cargo deny check` | 0 | **PASS** | Completed in 2.171 seconds: advisories, bans, licenses, and sources all `ok`. |

## Live CRAP Result

The fresh measurement produced `workspace.lcov` and `workspace-crap.json`.
The coverage subprocess used `--ignore-run-fail` and its log recorded two
failed test targets:

- `-p openwepp --test laned_shadow_h2637`
- `-p openwepp-hillslope-orchestrator --lib`

Those failures are not ordinary test authority. The separately executed full
Nextest command above is the binding ordinary-test result and passed all 1,960
tests.

A post-measurement diagnostic assessment present in the output directory
reported:

- production entries: `8,328`
- raw rows with CRAP strictly greater than 30: `3`
- adjudicated rows: `2`
- actionable rows: `1`
- touched production files: `0`
- touched actionable rows: `0`
- actionable rows outside the touched set: `1`

The actionable row was:

| File | Function | Line | CC | Coverage | CRAP |
| --- | --- | ---: | ---: | ---: | ---: |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/growth.rs` | `DirectGrowthInputs::compute_equation_growth_state` | 659 | 31 | 97.4359% | 31.0162 |

The two existing exact adjudications remained current:

| Adjudication | File | Function | CRAP |
| --- | --- | --- | ---: |
| `CQR-LOW-L08` | `crates/openwepp-meteorology/src/error.rs` | `MeteorologyError::fmt` | 56 |
| `CQR-LOW-L11` | `crates/openwepp-sim-contract/src/symbols.rs` | `SymbolAliasRegistryError::fmt` | 90 |

## Generated Artifacts

All paths below are outside the repository under
`/tmp/openwepp-acrap-live-20260713`.

| Artifact | SHA-256 |
| --- | --- |
| `workspace.lcov` | `14d5c30b0014a1336d3014594d261398b1d28f3183ec4196d97f6fcab822a823` |
| `workspace-crap.json` | `df3656c3b051cf1fcd45535c1c3b042903b592ac3324a0def44dc9908b142a1d` |
| `llvm-cov.log` | `2d1f84b449bef2f96406c6bc5e8c0191f1afcd1f5d6b0239abd8fed74dc4aaf0` |
| `llvm-cov-clean.log` | `8b0416565fb0e73da0638bacc62679a4ef28e00b9dedd9f8e27d8040ce92051d` |
| `cargo-crap.log` | `cb58c0da71795e8d71b399c93762f64644854c7962f8455fae4464b9e812434b` |
| `cargo-llvm-cov-version.txt` | `35c027b707427b0becc406096c32bd507961068f5d869735d7997da10bf44fd7` |
| `cargo-crap-version.txt` | `613daf2b27209d9be123ababd5ebd0bbc27ce14c9f63463552454b7cea0e7f86` |
| `manual-report.json` | `52d2e27a7e8ad338a7b2febfd277d4a442b3958910e1288791faef70b01028a1` |
| `manual-report.md` | `c825780c832ea59db011f327b3844363dd3ba08cea53aadfe4cfc682d5b0786c` |

Tool versions recorded by the driver were `cargo-llvm-cov 0.8.7` and
`cargo-crap 0.2.2`. The adjudication registry SHA-256 recorded by the diagnostic
assessment was
`97847eca9497b7547e058859f0b89bffb4aa5068b5cb44735aef78a66b5d29f6`.

## Freshness And Disposition

The live CRAP measurement completed before a narrow helper extraction was
applied to
`crates/openwepp-hillslope-orchestrator/src/direct_runtime/growth.rs`. That
file's observed modification time was `2026-07-13 22:44:16 -0700`, after the
live CRAP artifacts were written at about `22:37:33 -0700` and while the
standalone Nextest command was running. Therefore:

- the live CRAP failure is valid diagnostic evidence for the pre-extraction
  source;
- the formatting, clippy, and Nextest results do not establish terminal-source
  closure for the later helper extraction; and
- a fresh adjudicated CRAP measurement plus affected terminal-source gates is
  required before package closure.

No second live measurement was started, per the parent-agent instruction.

## Terminal Rerun

Evidence class: **Ran**

Status: `PASS`

A terminal-source rerun was completed after the growth helper extraction. The
source SHA-256 before and after the lane was
`1ce345e533159d7317f8c7d1a5f41b292a27896aa53d8e10d693d6366a6eb041`.

- live adjudicated gate: exit `0`, `raw=2`, `adjudicated=2`, `actionable=0`,
  `touched_files=1`
- repaired target: CC `27`, coverage `97.22222222222221%`, CRAP `27.015625`
- `cargo fmt --check`: `PASS`
- clippy with warnings denied: `PASS`
- full Nextest: `PASS`, 1,960 passed, 4 slow, 3 skipped
- `cargo deny check`: `PASS`

The terminal report is
`/tmp/openwepp-acrap-terminal-20260713/adjudicated-crap-report.json`, SHA-256
`4e0b3ca77b6c7019e8f386fc18e8462eb5df002b7611bc954f1b3bfd268a030a`.
Detailed command, metric, artifact-hash, coverage-subprocess, and freshness
evidence is recorded in
`docs/work-packages/20260713-cqr-growth-state-crap-regression-001/artifacts/heavy-run.md`.

## Post-review Hardened-driver Rerun

Evidence class: **Ran**

Status: `PASS`

The interrupted pre-hardening attempt is not evidence. This section records a
clean restart from an absent output directory using exactly:

    bash tools/release/run_adjudicated_crap_gate.sh --base-ref 3071849a0aec2abf8c17fe2405ce468f1533f631 --output-dir /tmp/openwepp-acrap-postreview-20260713

The command exited `0`. Its complete combined output was:

    INFO: collecting fresh workspace coverage for adjudicated CRAP
    production-source-manifest: sources=216 sha256=99930077a9965e42cb5791c52e909e17791fa400d869dd09f8c5b3c5ad80a367
    production-source-manifest: sources=216 sha256=99930077a9965e42cb5791c52e909e17791fa400d869dd09f8c5b3c5ad80a367
    adjudicated-crap: status=PASS raw=2 adjudicated=2 actionable=0 touched_files=1
    production-source-manifest: sources=216 sha256=99930077a9965e42cb5791c52e909e17791fa400d869dd09f8c5b3c5ad80a367
    INFO: adjudicated CRAP artifacts: /tmp/openwepp-acrap-postreview-20260713

No stderr text was observed. `run-status.json` records fresh acquisition from
`2026-07-14T06:57:06Z` through `2026-07-14T07:33:14Z`, exit `0`, and result
`PASS`.

### Hardened Evidence Checks

- `source-manifest-before.json`, `source-manifest-after.json`, and
  `source-manifest-final.json` are byte-identical at SHA-256
  `99930077a9965e42cb5791c52e909e17791fa400d869dd09f8c5b3c5ad80a367`.
- Each manifest records `216` production sources and `418` measurement inputs.
  The inputs include `394` Rust paths, `144` top-level Rust test paths, `21`
  Cargo/coverage/Nextest configuration paths, the checker, the driver, and the
  adjudication registry.
- The archived and canonical registries are byte-identical at SHA-256
  `10b19679e382ebacd6b2d20ee02144c461e01b1ac958731d07dd6585acb7d67f`.
- `sha256sum -c sha256sums.txt` passed for every listed artifact.
- The fresh report is `PASS`, debt status `PASS`, and
  `closure_eligible=true`. Its expected and reported production-crate censuses
  are identical at `17/17`.
- The report assessed `8,330` production entries: `2` raw, `2` adjudicated,
  `0` actionable, and `0` invalid or stale adjudications.
- The touched record is
  `M crates/openwepp-hillslope-orchestrator/src/direct_runtime/growth.rs`, with
  `0` touched and `0` untouched actionable rows.
- `DirectGrowthInputs::compute_equation_growth_state`: line `690`, CC `27`,
  coverage `97.22222222222221%`, CRAP `27.015625`.
- `DirectGrowthInputs::compute_root_mass_and_depth_candidates`: line `658`, CC
  `5`, coverage `100%`, CRAP `5`.
- The target Rust source remained SHA-256
  `1ce345e533159d7317f8c7d1a5f41b292a27896aa53d8e10d693d6366a6eb041`.

### Focused Affected Checks

| Command | Result |
| --- | --- |
| `.venv/bin/python -m unittest -v tests.python.test_adjudicated_crap_gate` | **PASS**, 15/15 in 4.583 seconds |
| `.venv/bin/python -m py_compile tools/release/check_adjudicated_crap.py tests/python/test_adjudicated_crap_gate.py` | **PASS** |
| `bash -n tools/release/run_adjudicated_crap_gate.sh tools/release/run_release_candidate_gates.sh` | **PASS** |
| `jq empty tools/release/adjudicated_crap_exceptions.json` | **PASS** |
| `ruby -e 'require "yaml"; YAML.load_file(".github/workflows/release-gates.yml")'` | **PASS** |
| `cargo fmt --check` | **PASS** |
| Scoped `markdown-doc lint` over the two packages and changed Markdown governance/tool paths | **PASS**, 35 files, 0 errors, 0 warnings |
| `git diff --check` | **PASS** |

Full Nextest, clippy, and deny were not repeated. The byte-identical hardened
manifests and unchanged target SHA prove that Rust source remained the source
already validated by the terminal 1,960/1,960 Nextest, clippy, and deny lane.

### Sealed Artifact Hashes

All paths are under `/tmp/openwepp-acrap-postreview-20260713`.

| Artifact | SHA-256 |
| --- | --- |
| `adjudicated-crap-report.json` | `1097766adfd1c06b2f5a80271f8b589de286486b549665d23b74d740b20275d0` |
| `adjudicated-crap-report.md` | `b41a6c01aae9c3667d26ec74f416b29fd1b43660e1dce73068c6edf0be4b5d44` |
| `adjudication-registry.json` | `10b19679e382ebacd6b2d20ee02144c461e01b1ac958731d07dd6585acb7d67f` |
| `cargo-crap-version.txt` | `613daf2b27209d9be123ababd5ebd0bbc27ce14c9f63463552454b7cea0e7f86` |
| `cargo-crap.log` | `cb58c0da71795e8d71b399c93762f64644854c7962f8455fae4464b9e812434b` |
| `cargo-llvm-cov-version.txt` | `35c027b707427b0becc406096c32bd507961068f5d869735d7997da10bf44fd7` |
| `llvm-cov-clean.log` | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| `llvm-cov.log` | `2fa2b74e3132b1834bb669351b46f2a0fd71114371e7b79be53130c4129dde62` |
| `run-status.json` | `8ddc9df6fafaa727b2e650128b195718bb9fd38b019aa17ac9682837786eca06` |
| `sha256sums.txt` | `005f0cade21e83f666c55b3984463e5e66cd12955fe30db4cfe29baa18311f7c` |
| `source-manifest-before.json` | `99930077a9965e42cb5791c52e909e17791fa400d869dd09f8c5b3c5ad80a367` |
| `source-manifest-after.json` | `99930077a9965e42cb5791c52e909e17791fa400d869dd09f8c5b3c5ad80a367` |
| `source-manifest-final.json` | `99930077a9965e42cb5791c52e909e17791fa400d869dd09f8c5b3c5ad80a367` |
| `workspace-crap.json` | `c0f4cf2fe61b8f1d214025bfe25a78a0d35c7a1a6b6db873a9db22fb71f289ac` |
| `workspace.lcov` | `67f371fee21127f8fc318ab7f42adc9904acfa0de93bfd825f017d94abb0b22e` |

## Final Residual-fix Rerun

Evidence class: **Ran**

Status: `PASS`

After the accepted Reviewer B residual fixes, a new clean output directory was
measured using exactly:

    bash tools/release/run_adjudicated_crap_gate.sh --base-ref 3071849a0aec2abf8c17fe2405ce468f1533f631 --output-dir /tmp/openwepp-acrap-final-20260713

The driver exited `0`; no stderr text was observed. Its combined output was:

    INFO: collecting fresh workspace coverage for adjudicated CRAP
    production-source-manifest: sources=216 sha256=2b40242a65895c3e1dff365c87e8eca237570a188313fd7777a741c019096483
    production-source-manifest: sources=216 sha256=2b40242a65895c3e1dff365c87e8eca237570a188313fd7777a741c019096483
    adjudicated-crap: status=PASS raw=2 adjudicated=2 actionable=0 touched_files=1
    production-source-manifest: sources=216 sha256=2b40242a65895c3e1dff365c87e8eca237570a188313fd7777a741c019096483
    INFO: adjudicated CRAP artifacts: /tmp/openwepp-acrap-final-20260713

`run-status.json` records fresh acquisition from `2026-07-14T07:51:01Z`
through `2026-07-14T08:26:23Z`, exit `0`, and result `PASS`.

### Final Seal And Closure

- Manifest schema is `openwepp-production-source-manifest-v2`.
  Before/after/final manifests are byte-identical at SHA-256
  `2b40242a65895c3e1dff365c87e8eca237570a188313fd7777a741c019096483`.
- Each manifest contains `216` production sources and `419` measurement
  inputs: `394` Rust inputs, including `144` top-level Rust tests, `21`
  Cargo/coverage/Nextest configuration inputs, the checker, driver, registry,
  and `rust-toolchain.toml`.
- `rust-toolchain.toml` is bound at SHA-256
  `3e18e70208ee460635e239a91c142cf67371feafb718b05617ff06f388bf96df`.
- The checker and driver are bound at SHA-256
  `e4eee61bb2cc573e3b49716dd64bc6216dbc2674217a9fd92559a06ee07fb66e`
  and
  `ad25cfdcbb31d103e11fb4688703b865b8ec61b443562291028b6b9a17b5a819`.
- The archived registry is byte-identical to canonical at SHA-256
  `10b19679e382ebacd6b2d20ee02144c461e01b1ac958731d07dd6585acb7d67f`.
- Recorded tools are Cargo `1.92.0` commit `344c4567c`, Rustc `1.92.0`
  commit `ded5c06cf`, LLVM `21.1.3`, cargo-llvm-cov `0.8.7`, and cargo-crap
  `0.2.2`. The report carries the Cargo/Rustc version text in acquisition
  provenance.
- `sha256sum -c sha256sums.txt` passed for all `16` listed artifacts.
- The report is fresh, status `PASS`, debt status `PASS`, and
  `closure_eligible=true`. Expected and reported crate censuses are identical
  at `17/17`.
- It assessed `8,330` production entries: `2` raw, `2` adjudicated, `0`
  actionable, and no invalid or stale adjudication.
- The touched record is
  `M crates/openwepp-hillslope-orchestrator/src/direct_runtime/growth.rs`; both
  touched and untouched actionable counts are `0`.
- `DirectGrowthInputs::compute_equation_growth_state`: line `690`, CC `27`,
  coverage `97.22222222222221%`, CRAP `27.015625`.
- `DirectGrowthInputs::compute_root_mass_and_depth_candidates`: line `658`, CC
  `5`, coverage `100%`, CRAP `5`.
- The target Rust source remained SHA-256
  `1ce345e533159d7317f8c7d1a5f41b292a27896aa53d8e10d693d6366a6eb041`.

The coverage subprocess used `--ignore-run-fail` and recorded the known
`-p openwepp --test laned_shadow_h2637` target failure. It is not ordinary test
authority. Rust source is byte-identical to the terminal source that already
passed the binding 1,960/1,960 standalone Nextest lane.

### Final Focused Checks

| Command | Result |
| --- | --- |
| `.venv/bin/python -m unittest -v tests.python.test_adjudicated_crap_gate` | **PASS**, 17/17 in 4.813 seconds |
| `.venv/bin/python -m py_compile tools/release/check_adjudicated_crap.py tests/python/test_adjudicated_crap_gate.py` | **PASS** |
| `bash -n tools/release/run_adjudicated_crap_gate.sh tools/release/run_release_candidate_gates.sh` | **PASS** |
| `cargo fmt --check` | **PASS** |
| `jq empty` over the canonical registry and final JSON artifacts | **PASS** |
| Ruby parse of `.github/workflows/release-gates.yml` | **PASS** |
| Scoped `markdown-doc lint` over current changed documentation | **PASS**, 37 files, 0 errors, 0 warnings |
| `git diff --check` | **PASS** |

### Final Artifact Hashes

All paths are under `/tmp/openwepp-acrap-final-20260713`.

| Artifact | SHA-256 |
| --- | --- |
| `adjudicated-crap-report.json` | `3ad2a65a0c8526ab2155bad26d08f915e8b257f5b70b62f3006dd78381fe098d` |
| `adjudicated-crap-report.md` | `f149a0eecb8bde48561d74f630fbd39889992c981590478ff4686208ca96cf76` |
| `adjudication-registry.json` | `10b19679e382ebacd6b2d20ee02144c461e01b1ac958731d07dd6585acb7d67f` |
| `cargo-version.txt` | `faf93773c4f1319d248c431f7ef1d7a7cd3bd556cfd4990b83989c1591e3490f` |
| `rustc-version.txt` | `8f97c91e920ce0d02680e3d01eb87afa0c77290dcd5d19eb901ae194d4700d90` |
| `cargo-crap-version.txt` | `613daf2b27209d9be123ababd5ebd0bbc27ce14c9f63463552454b7cea0e7f86` |
| `cargo-crap.log` | `cb58c0da71795e8d71b399c93762f64644854c7962f8455fae4464b9e812434b` |
| `cargo-llvm-cov-version.txt` | `35c027b707427b0becc406096c32bd507961068f5d869735d7997da10bf44fd7` |
| `llvm-cov-clean.log` | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| `llvm-cov.log` | `f69cfcda0cd847ad0253ef5a6871354b4d17b34feef8c22410b0e9c7267ab03e` |
| `run-status.json` | `e1a55d4d914c202af97d3000fe1cc0e9bf6c4adfea47806b8bd3a70da9f71c66` |
| `sha256sums.txt` | `9838e7d5533d60ce6048d414aefb0a245247f5c7169d8440fc8d7f0873191a1f` |
| `source-manifest-before.json` | `2b40242a65895c3e1dff365c87e8eca237570a188313fd7777a741c019096483` |
| `source-manifest-after.json` | `2b40242a65895c3e1dff365c87e8eca237570a188313fd7777a741c019096483` |
| `source-manifest-final.json` | `2b40242a65895c3e1dff365c87e8eca237570a188313fd7777a741c019096483` |
| `workspace-crap.json` | `5fe3d67263508a9c2a7fbfb473ab40b00380d38392a5a033a33e4658a9452c40` |
| `workspace.lcov` | `bf6c20a4dab61145011051e982aff4d749190036979e7298cbeacecf2a9c9256` |
