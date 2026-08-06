# Gate Results

Evidence class: Ran. Exact clean candidate
`ffeecbaeaa3d104284007180ffb012bf5e2ec60c`; clean worktree confirmed at start
and end. TESTGATE was not run.

## Exact-Head Closeout Gates

| Gate | Exact argv | Result | Duration |
| --- | --- | --- | ---: |
| Formatting | `cargo fmt --all -- --check` | PASS | `2.97 s` |
| Clippy | `cargo clippy --workspace --all-targets --all-features -- -D warnings` | PASS | `6.92 s` |
| Doctests | `cargo test --workspace --doc` | PASS, 20 crates and 0 cases | `6.88 s` |
| Quick | `cargo nextest run --workspace --profile quick` | PASS, `2,189/2,189`; 40 profile-skipped | `2,354.527 s` Nextest; `2,361.571 s` wrapper |
| Frost | `cargo nextest run --workspace --profile frost` | PASS, `360/360`; 1,923 profile-skipped | `536.319 s` Nextest; `539.040 s` wrapper |
| Full | `cargo nextest run --workspace` | PASS, `2,238/2,238`; 33 profile-skipped | `2,266.490 s` Nextest; `2,268.286 s` wrapper |

Evidence locations:

- static logs:
  `target/local-ci-history/snow-stage3-closeout-ffeecbae/static-gates/`;
- profile histories:
  `target/local-ci-history/snow-stage3-closeout-ffeecbae/{quick,frost,full}/`;
  and
- JUnit:
  `target/nextest/{quick,frost,full}/junit.xml`.

## Focused And Contract Gates

- Corrected snow runtime and contract tests: PASS, `32/32`.
- Assurance source plus corrected snow runtime/contract tests: PASS, `44/44`.
- `openwepp-assurance validate --all`: PASS, `3/3` DRAFT reports and public
  count zero.
- Strict `SC-SNOWENERGY-001` binding exposure: PASS, eight rows.
- Science-contract unit compliance: PASS.
- Package/roadmap/catalog Markdown lint and validation: PASS with zero
  findings.
- Corrected Snowbird direct-production run: PASS, `14,245/14,245` days; trace,
  binary, sidecar, source commit, commands, and frozen medians independently
  reconciled.

## Superseded Failure Evidence

Failures were corrected rather than hidden:

1. Clean `01df43ad` failed workspace Clippy because an assurance amendment test
   exceeded the function-size limit. The behavior-neutral helper extraction in
   `2d035638` corrected it.
2. Clean `d1cf59d5` passed static gates but quick failed after 86 executed
   cases: 55 passed, 31 failed, 40 skipped, and 2,103 were canceled. All 31
   failures reduced to stale `SC-SNOWENERGY-001` assurance identity or its
   downstream assembly/planner effects. The typed DRAFT source adoption
   advanced generation to `910ab3d3`; the fresh exact-head suite then passed.

Superseded evidence remains under
`target/local-ci-history/snow-stage3-closeout-{01df43ad,d1cf59d5}/`.
