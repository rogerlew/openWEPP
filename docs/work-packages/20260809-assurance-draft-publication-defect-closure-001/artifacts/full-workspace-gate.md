# Exact-Tree Full-Workspace Gate

Evidence class: `Ran`

## Terminal Run Identity

- Delegated role: `comparator_suite_runner`
- Working directory: `/home/workdir/openWEPP`
- Git HEAD: `4237552aa8dbc84a8baeff800014b23c7e75be9f`
- Reviewed assurance test blob at pre-run and post-run:
  `07e65f289049cfa6a96617a9922f70a06d8f5165`
- Pre-run working-tree fingerprint:
  `4243658ad03b52e20ff621b8c957664abd549b65cbb0b6feb20aa3957546360d`
- Pre-run inventory: `full-workspace-gate-pre.txt`
- Terminal log: `full-workspace-gate-run.log`
- Terminal log SHA-256:
  `d125d5ff4c5050e5068ac07bd698aa07ae2118ce46a204e1dfc679e159d9730d`

The delegated agent hit a service-capacity error after launching the terminal
process. The process remained healthy and was monitored to termination by the
primary agent; the terminal log and process exit summary were preserved. This
service event did not alter, restart, or narrow the gate.

## Command

```bash
TMPDIR=/home/workdir/openwepp-task-tmp \
  cargo nextest run --workspace --profile full
```

## Result

`PASS`.

```text
Summary [3300.706s] 2325 tests run: 2325 passed (55 slow), 33 skipped
TIME_ELAPSED_SECONDS=3302.06
```

The 33 skips are the declared full-profile filter recorded by Nextest at
startup: `33 tests and 5 binaries skipped`, including `28 tests and 5 binaries`
through `profile.full.default-filter`. No selected test failed or timed out.

## Superseded Pre-Review Attempt

The first delegated run compiled test blob
`6e54e56258c621849179f13b4bb6ca6fb595cd11` before review finding disposition.
It was intentionally stopped with `SIGINT` because it could not validate the
terminal Rust diff. Its log is retained as `full-workspace-gate.log` and is not
counted as correctness evidence.

## Reuse

This full-profile pass directly includes the complete
`assurance_v2_publication_contract` target, the `openwepp-assurance` crate, the
vegetation authority contract test, and all other selected workspace tests. It
therefore closes the assurance package's Critical gate and is reusable to lift
the vegetation authority-reframe package's unrelated full-workspace hold.
