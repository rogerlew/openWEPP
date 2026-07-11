# Coverage and CRAP before

Status: fresh post-characterization/pre-decomposition baseline captured
Evidence mode: Ran

The immediately preceding nightly target recorded production coverage
`398/614` lines (`64.821%`). Eligible CRAP included
`parse_fixeddate_str=53.909`; observability-only formatter CRAP `132` is an
ADR-0021 Decision-3 exclusion. This package will run a fresh same-run LCOV/CRAP
baseline before decomposition and retain the upstream figures as selection
provenance.

Ran: delegated full-workspace LCOV completed in `2094.67s` with
`--ignore-run-fail`. The target is now `614/631` lines (`97.306%`) after
characterization and the finite guard. Same-LCOV cargo-crap reports one eligible
row above 30: `parse_fixeddate_str`, CC `39`, coverage `92.233%`, CRAP
`39.71266839750459`. Every other deduplicated logical function is at most 15;
`FixedDateParseError::fmt` is now fully covered rather than excluded.

LCOV: `/tmp/fdir-fq01.lcov`, 4,363,246 bytes, SHA-256
`8f4d6678b6ca0cc48a9715eccfcfab1f63cd21cb7820736fe45cfefab2f2c8ba`.
CRAP JSON: `/tmp/fdir-fq01-crap-early.json`, 2,844,114 bytes, SHA-256
`9ce14d3a9fcdae651bb286474f105dd75175053b9f770745e4a9b3c5e8296fcc`.

Full-workspace JSON: `636/649` regions (`97.997%`), all `23/23` logical
functions covered, and no function below the 75% region floor. JSON size
18,963,212 bytes; SHA-256
`728de8c8fe188025397694dfe159b085d64ed9037a9d4315a268d74f3873fdaa`.

Attributed unrelated workspace coverage failures: only
`openwepp::laned_shadow_h2637` failed (5 failed, 2 ignored); one HPARITY test
was explicitly ignored. The focused fixed-date suite passed 27/27 and no
fixed-date test was ignored.

Exact commands and results on production source SHA-256
`99641f2d8b8d47407d26dfa6ca203f7ac5e0d64a58699b9925efe7febcecb8e7`
and focused-test SHA-256
`3376bdf537bea94fa10b30f61ec9e42d4d12c89987439f9a0887bec7feea3c32`:

| Command | Exit | Elapsed |
| --- | ---: | ---: |
| `cargo llvm-cov clean --workspace` | 0 | 1.39s |
| `cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path /tmp/fdir-fq01.lcov` | 0 | 2094.67s |
| `cargo llvm-cov --workspace --ignore-run-fail --json --output-path /tmp/fdir-fq01.json` | 0 | 2056.19s |
| `cargo crap --workspace --lcov /tmp/fdir-fq01.lcov --min 0 --format json --output /tmp/fdir-fq01-crap.json` | 0 | 1.09s |

Eligible surface: the complete production module; no line, region, formatter,
or defensive-arm exclusion was applied. CRAP rows were deduplicated by logical
function and source line. The worktree also contained package docs and a
concurrent unrelated root `README.md` edit, neither of which affects the
compiled source/test hashes; `README.md` is excluded from this package commit.
