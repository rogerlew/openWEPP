# Implementation Record

Evidence labels follow the repo convention: `Static:` for inspection and
`Ran:` for commands executed in this session.

## Static

- Existing `.config/nextest.toml` already used `test-threads = "num-cpus"` for
  general tests and capped fixture-heavy groups (`snowbench`, `cli-fixture`,
  `frost-fixture`, `runner-fixture`).
- Existing full-suite JUnit data showed the slowest test tail is dominated by
  snow/frost export tests, followed by CLI/runner fixture families.

## Changes

- Added `tools/local_ci/nextest_timing.py` and `tools/local_ci/README.md`.
- Hardened `nextest_timing.py run` and `sweep` after review so they delete the
  selected JUnit before execution and refuse to record unless a fresh JUnit is
  produced after command start. `summarize` remains the existing-JUnit mode.
- Added `docs/standards/local-ci-gate-selection.md`.
- Updated agent/contributor guidance to prefer focused/domain gates during
  review loops and reserve full-suite runs for branch-head closure unless a
  package requires otherwise.
- Updated nextest capped groups and added an erosion-focused profile based on
  measured local behavior.
- Final cap decisions:
  - `cli-fixture`: `2 -> 4`
  - `runner-fixture`: `2 -> 4`
  - `frost-fixture`: `2 -> 4`
  - `snowbench`: remains `1`

## Ran

- `python3 -m py_compile tools/local_ci/nextest_timing.py`
- `python3 tools/local_ci/nextest_timing.py summarize --label existing-full-baseline --profile full --junit target/nextest/full/junit.xml --top 8`
- `python3 tools/local_ci/nextest_timing.py run --label stale-junit-negative --profile default -- true` returned `1` with a missing fresh-JUnit error.
- `python3 tools/local_ci/nextest_timing.py run --label fresh-nextest-smoke --profile default -- cargo nextest run --test erod14_contract_authority_closure_contract`
- `cargo nextest show-config test-groups --profile full`
- `cargo nextest list --workspace --profile erosion`
- `cargo nextest run --workspace --profile erosion --no-run`
- `wctl doc-lint --path docs/standards`
- `git diff --check`

Additional gate evidence is in `gate-results.md`; concurrency evidence is in
`empirical-concurrency.md`.
