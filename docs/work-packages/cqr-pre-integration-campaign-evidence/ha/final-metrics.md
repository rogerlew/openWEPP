# High-A Final Metrics

Evidence class: **Ran**

## Source State

- Repository: `/home/workdir/openWEPP`.
- Commit: `fdf16c9d0b70996e9811acf7879fdfe1fda8a6d8`.
- Branch: `main`.
- `git status --short` was empty before and after measurement.
- The delegated `comparator_suite_runner` performed read-only measurement and
  did not rerun any failure.

## Exact Commands And Results

The binding protocol was expanded with `slug=ha` and `phase=final`:

```text
cargo llvm-cov clean --workspace
/usr/bin/time -v -o /tmp/openwepp-cqr-preint-ha-final-lcov.time cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path /tmp/openwepp-cqr-preint-ha-final.lcov > /tmp/openwepp-cqr-preint-ha-final-lcov.log 2>&1
/usr/bin/time -v -o /tmp/openwepp-cqr-preint-ha-final-json.time cargo llvm-cov --workspace --ignore-run-fail --json --output-path /tmp/openwepp-cqr-preint-ha-final.json > /tmp/openwepp-cqr-preint-ha-final-json.log 2>&1
/usr/bin/time -v -o /tmp/openwepp-cqr-preint-ha-final-crap.time cargo crap --workspace --lcov /tmp/openwepp-cqr-preint-ha-final.lcov --min 0 --format json --output /tmp/openwepp-cqr-preint-ha-final-crap.json > /tmp/openwepp-cqr-preint-ha-final-crap.log 2>&1
sha256sum /tmp/openwepp-cqr-preint-ha-final.lcov /tmp/openwepp-cqr-preint-ha-final.json /tmp/openwepp-cqr-preint-ha-final-crap.json
wc -c /tmp/openwepp-cqr-preint-ha-final.lcov /tmp/openwepp-cqr-preint-ha-final.json /tmp/openwepp-cqr-preint-ha-final-crap.json
```

The exact production filter from the execution contract materialized
`/tmp/openwepp-cqr-preint-ha-final-production-over30.json`.

| Step | Exit | Elapsed | Max RSS |
| --- | ---: | ---: | ---: |
| Clean | 0 | included before LCOV | not recorded |
| LCOV | 0 | 35:29.06 | 892,128 KB |
| JSON | 0 | 36:20.22 | 829,908 KB |
| CRAP | 0 | 1.22 s | 206,504 KB |
| Hash, size, exact filter | 0 | negligible | not recorded |

Both coverage formats emitted 175 test-result sets: 1,831 passed, four failed,
three ignored, zero measured, and 476 filtered.

## Artifact Integrity

| Artifact | Bytes | SHA-256 |
| --- | ---: | --- |
| `/tmp/openwepp-cqr-preint-ha-final.lcov` | 4,443,698 | `86d83d5ece93ed2b5a737a56cfc4d749f094c5e231631ccc6d8d3f7c6d6d7d8b` |
| `/tmp/openwepp-cqr-preint-ha-final.json` | 19,495,485 | `ca3185cd758a44acce9ad22b2e1b6b4dfd78ce37f3bd548d52729e487e5718f7` |
| `/tmp/openwepp-cqr-preint-ha-final-crap.json` | 2,905,674 | `80a156327b64d74f9db821585f68181bcda4bdba63edd34ce497b8f2fe0bcf00` |
| `/tmp/openwepp-cqr-preint-ha-final-production-over30.json` | 12,261 | `df4163e492b733cd1b1722186991bd0c5afbe0017cea564015cf1c7ce55c3d7c` |

Logs and time reports remain under
`/tmp/openwepp-cqr-preint-ha-final-{lcov,json,crap}.{log,time}`. Their hashes
were captured by the delegated runner.

## Failure Attribution

Both formats reproduced the three start-baseline parallel-environment failures
in `-p openwepp --test laned_shadow_h2637`: five passed, three failed, and two
ignored. The failing symbols are
`h2637_active_fails_closed_without_routing_coefficients`,
`h2637_active_and_disable_are_mutually_exclusive`, and
`h2637_active_and_shadow_are_mutually_exclusive`; each received a successful
report because of the known shared-environment family. The unchanged test file
SHA-256 is `e6a8b65fe0fe02951a0751fbc6c187dbd75bec189ac049e5a39deea917edcd9d`.

The fourth failure is the known parallel audit-counter assertion in
`r3c_lane_transfer_span_projects_multilane_topology` (`left: 2`, `right: 1` at
`direct_runtime_r3c_r4b.rs:780`), previously attributed during HA-05. Its
unchanged test source SHA-256 is
`9117d2ff4e0a0d9ecc5f30ae1fe1dfd2aecee28574fbe3dea5aed034a9ddaf7c`.
Neither family is target-related or newly regressed. No failure was rerun.
The `compat_quoted_header... FAILED` text is intentional drift-guard subprocess
output inside a passing test, not a failed target.

## Census And Ratchet

The exact production filter yields 54 deduplicated rows across 35 modules,
down from 67 rows across 45 modules at High-A start. All 13 fixed High-A rows
are absent, and every touched production module has zero row above 30.

Comparing normalized `(file, function)` identities finds zero new row, 13
removed fixed-cohort identities, and 54 persistent untouched-backlog
identities. Therefore the new-row, touched-module, and attributable-consumer
ratchets all pass. The final CRAP JSON is the numeric authority; source-
unchanged coverage variation in the untouched backlog does not change the
ratchet disposition.
