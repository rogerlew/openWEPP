# Gate Results

| Gate | Status | Evidence |
|---|---|---|
| Timing script compile | `PASS` | `python3 -m py_compile tools/local_ci/nextest_timing.py` |
| Existing JUnit parse | `PASS` | `nextest_timing.py summarize` parsed the pre-sweep `target/nextest/full/junit.xml` (`1333` tests) and recorded it in `target/local-ci-history/nextest-runs.jsonl` |
| Stale JUnit rejection | `PASS` | `python3 tools/local_ci/nextest_timing.py run --label stale-junit-negative --profile default -- true` returned `1` and reported missing fresh JUnit |
| Fresh JUnit recording | `PASS` | `python3 tools/local_ci/nextest_timing.py run --label fresh-nextest-smoke --profile default -- cargo nextest run --test erod14_contract_authority_closure_contract`: `2` tests passed and timing recorded |
| Concurrency sweep | `PASS` | runner, CLI, snowbench, and non-snowbench frost sweeps recorded in `empirical-concurrency.md` |
| Nextest config syntax/focused run | `PASS` | `cargo nextest list --workspace --profile erosion` selected `212` tests; `cargo nextest run --workspace --profile erosion --no-run` passed |
| Nextest group assignment | `PASS` | `cargo nextest show-config test-groups --profile full` shows `cli-fixture`, `frost-fixture`, and `runner-fixture` at max threads `4`, and `snowbench` at max threads `1` with `snowdensity05e_melt_adjudication` assigned there |
| Independent review | `PASS` | Bernoulli review findings accepted/fixed/rechecked; Locke second review reported no remaining merge-blocking findings |
| Docs/path check | `PASS` | `wctl doc-lint --path docs/standards`: `7 files validated, 0 errors, 0 warnings`; package path lint reported `0 files validated` |
| Whitespace | `PASS` | `git diff --check` |

## Not Run

- Full workspace nextest was intentionally not run for this docs/tooling package.
  The package objective is to reduce reflexive full-suite use; no Rust kernel or
  production behavior changed.
- Workspace clippy and `cargo deny` were not run for the same reason. The
  executable change is a Python stdlib local utility validated with
  `py_compile` and a live JUnit parse.
