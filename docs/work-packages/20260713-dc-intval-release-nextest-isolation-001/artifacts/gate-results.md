# Gate Results

Status: `FAIL`

Evidence class: **Ran**

## Intake Verification

Execution repository HEAD was
`1a6a03494745e77e352c3c1c9ab190d6fb0746a7`; the release-script, README, and
source-level contract changes were already present in the package worktree.
The delegated runner did not edit source, tests, release scripts, or fixtures.

| Input | Expected | Observed | Status |
| --- | --- | --- | --- |
| `/workdir/wepp-forest` HEAD | `375ccc296ed1ea491f599ff1b1a25b415d494a2a` | exact match | PASS |
| `defect_seeds.csv` SHA-256 | `42b7d827d842ecbe75843175a80ab4f67a097784156658df8fb849161eb98958` | exact match | PASS |
| `hillslope_watchlist.csv` SHA-256 | `42214345a228d27a0536b771dd73068dc897d369f54cb8a197457dea675e26ab` | exact match | PASS |

Verification log/time:
`artifacts/logs/00-pinned-input-verification.{log,time}`. The command exited 0
in 0.02 s with maximum RSS 3,840 KB.

## Exact Release Command

```text
bash tools/release/run_release_candidate_gates.sh --cohort-seeds-csv /workdir/wepp-forest/docs/work-packages/20260503-wb05b-forest-hillslope-closure-sweep/artifacts/audits/_meta/defect_seeds.csv --watchlist-csv /workdir/wepp-forest/docs/ablation/hillslope_watchlist.csv --expect-suite wb05b_1166=1166 --expect-suite release_gate_watchlist=19
```

| Exit | Elapsed | Max RSS | Log/time | Status |
| ---: | ---: | ---: | --- | --- |
| 1 | `9:43.01` | 209,716 KB | `artifacts/logs/01-release-candidate.{log,time}` | FAIL |

No skip flag, retry, waiver, or source edit was used.

## Lane Results

| Release lane | Result | Status |
| --- | --- | --- |
| Workspace check | completed before full tests | PASS |
| Canonical full nextest | 1,945/1,945 passed; three skipped; four slow | PASS |
| Dependency policy | advisories, bans, licenses, and sources all OK | PASS |
| Authority fixture integrity | stopped on missing provenance metadata | FAIL |
| Remaining authority suites | not reached after fail-fast integrity error | BLOCKED |
| Release binary build/staging/lint | not reached | BLOCKED |
| Required stability cohorts | not reached | BLOCKED |

The corrected nextest lane establishes process-isolated Rust closure and does
not reproduce the original three H2637 shared-environment failures.

The new real blocker is suite
`cas_l4_subhyd_watyld_fcwp_consistency_001`. Its fixture
`wb19_fcwp_coca_watyld_cases.json` is bound to
`tests/fixtures/constitutive/cas_l4_subhyd_watyld_fcwp_consistency_001/fixtures.provenance.yaml`,
which the release authority gate reports as missing required provenance keys
and `schema_version`. The script failed closed with exit 1.

## Release Artifacts And Reports

- Release scratch directory: `/tmp/openwepp_release_260713ci_IIxdqo`.
- Partial authority report:
  `/tmp/openwepp_release_260713ci_IIxdqo/authority_suite_results.md` (1,357
  bytes).
- No release binaries, sidecars, lint result, or stability JSON were produced
  because failure occurred before those lanes.

## Separate Final Gates

The package allows these only after an exit-zero release command. They are
therefore blocked, not deferred:

| Gate | Status | Reason |
| --- | --- | --- |
| `cargo fmt --check` | BLOCKED | release command exit 1 |
| `cargo clippy --workspace --all-targets -- -D warnings` | BLOCKED | release command exit 1 |
| `cargo nextest run --workspace --profile full` | BLOCKED | release command exit 1 |
| `cargo deny check` | BLOCKED | release command exit 1 |
| scoped `markdown-doc lint` | BLOCKED | release command exit 1 |
| `git diff --check` | BLOCKED | release command exit 1 |

Disposition: `HOLD`. The next release attempt requires corrected, authoritative
fixture provenance for `cas_l4_subhyd_watyld_fcwp_consistency_001`; this runner
did not alter that protected fixture evidence.

After dual review corrections and dual verification, scoped Markdown lint and
`git diff --check` passed for the terminal HOLD artifact set. These lightweight
document checks do not replace the blocked post-release closure gates above.
