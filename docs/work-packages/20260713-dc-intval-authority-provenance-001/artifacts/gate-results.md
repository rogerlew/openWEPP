# Gate Results

Status: `FAIL`

Evidence class: **Ran**

## Pinned Input Verification

Execution repository HEAD was
`ed22f37bfef45eee4ae06eb7e08a2abc8561fc81`; package-authorized fixture
provenance and test changes were already dirty. The delegated runner did not
edit source, tests, fixtures, scripts, or documentation outside this artifact.

| Input | Expected | Observed | Status |
| --- | --- | --- | --- |
| `/workdir/wepp-forest` HEAD | `375ccc296ed1ea491f599ff1b1a25b415d494a2a` | exact match | PASS |
| `defect_seeds.csv` SHA-256 | `42b7d827d842ecbe75843175a80ab4f67a097784156658df8fb849161eb98958` | exact match | PASS |
| `hillslope_watchlist.csv` SHA-256 | `42214345a228d27a0536b771dd73068dc897d369f54cb8a197457dea675e26ab` | exact match | PASS |

Verification evidence is
`artifacts/logs/00-pinned-input-verification.{log,time}`: exit 0 in 0.02 s,
maximum RSS 3,840 KB.

## Exact Release Command

```text
bash tools/release/run_release_candidate_gates.sh --cohort-seeds-csv /workdir/wepp-forest/docs/work-packages/20260503-wb05b-forest-hillslope-closure-sweep/artifacts/audits/_meta/defect_seeds.csv --watchlist-csv /workdir/wepp-forest/docs/ablation/hillslope_watchlist.csv --expect-suite wb05b_1166=1166 --expect-suite release_gate_watchlist=19
```

| Attempt | Exit | Elapsed | Max RSS | Evidence | Result |
| --- | ---: | ---: | ---: | --- | --- |
| 01 | 1 | `0:02.13` | 70,272 KB | `artifacts/logs/01-release-candidate.{log,time}` | formatting FAIL; preserved pre-correction evidence |
| 02 | 1 | `9:44.72` | 209,340 KB | `artifacts/logs/02-release-candidate.{log,time}` | required authority target absent; terminal FAIL |

No skip, unchanged retry, retry-until-green, or waiver was used. Attempt 02 was
authorized only after formatting the new guard and passing fmt/AUTH06 locally.

## Attempt Results

Attempt 01 failed at the workspace formatting lane on the new AUTH-PROV
regression. The parent corrected that in-envelope formatting and reported both
`cargo fmt --check` and AUTH06 5/5 passing before authorizing attempt 02.

Attempt 02 restarted the exact command from the beginning. Workspace full
nextest passed 1,946/1,946 tests with three skipped and four slow; dependency
policy reported advisories, bans, licenses, and sources all OK. Authority
fixture-integrity verification also advanced beyond the corrected provenance.

The terminal failure occurred when the registry's first required hard-fail
lane requested nonexistent Cargo test target
`auth05_level4_constitutive_authority_hardening_contract`. Cargo listed the
current targets and returned nonzero; the release script failed closed with
exit 1. The separately noted absent
`hphys0227_wb19_fcwp_coca_watyld_authority_contract` target was not reached
because the missing AUTH05 target failed first.

| Release lane | Result | Status |
| --- | --- | --- |
| Workspace formatting | passed in attempt 02 | PASS |
| Workspace full nextest | 1,946/1,946; three skipped; four slow | PASS |
| Dependency policy | advisories, bans, licenses, sources all OK | PASS |
| Authority fixture integrity | corrected provenance accepted | PASS |
| Required authority suites | missing AUTH05 Cargo target | FAIL |
| Release binary build/staging/lint | not reached | BLOCKED |
| Required stability suites | not reached | BLOCKED |

## Release Artifacts

- Attempt 01 empty scratch directory:
  `/tmp/openwepp_release_260713ci_dzNEnF`.
- Attempt 02 release scratch directory:
  `/tmp/openwepp_release_260713ci_Zg2Gjk`.
- Attempt 02 partial authority report:
  `/tmp/openwepp_release_260713ci_Zg2Gjk/authority_suite_results.md` (2,032
  bytes).
- No release binary, sidecar, lint result, or stability JSON was produced.

## Separate Final Gates

The package permits separate final gates only after an exit-zero release run.
All are therefore blocked:

| Gate | Status | Reason |
| --- | --- | --- |
| `cargo fmt --check` | BLOCKED | exact release command exit 1 |
| `cargo clippy --workspace --all-targets -- -D warnings` | BLOCKED | exact release command exit 1 |
| `cargo nextest run --workspace --profile full` | BLOCKED | exact release command exit 1 |
| `cargo deny check` | BLOCKED | exact release command exit 1 |
| scoped `markdown-doc lint` | BLOCKED | exact release command exit 1 |
| `git diff --check` | BLOCKED | exact release command exit 1 |

Disposition: `HOLD`. Restart requires registry-required authority lanes to name
current executable Cargo test targets, including the first failing AUTH05
binding. This runner made no correction, waiver, or retry after attempt 02.

After review fixes and dual verification, scoped Markdown lint and
`git diff --check` passed for the terminal HOLD and iterative finalizer
documentation. These lightweight checks do not replace blocked post-release
closure gates.
