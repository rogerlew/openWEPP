# Canonical one-day wall-time attribution

Evidence mode: `Ran + Static`.

Date: `2026-08-29`.

Scope: profiling only. This increment adds opt-in thread-local timers to the
ignored one-day qualification harness. It changes no equations, tolerances,
fixed-point or adaptive decisions, receipts, publication, restart, or wire
state. Without `begin_adaptive_parent_telemetry_v1`, every detailed recorder is
a no-op and no production diagnostic is persisted.

## Execution

`perf` sampling was unavailable because the host has
`perf_event_paranoid=4`. A five-parent syscall profile nevertheless measured
`79.91 s` user CPU, `0.15 s` system CPU, and only `0.113 s` in all syscalls
over `80.17 s` wall, excluding external orchestration/I/O as the material
cost. Retained report:
`/tmp/openwepp-stage3-five-parent-strace.txt`.

The terminal detailed run used the optimized test profile and exact canonical
one-day fixture:

```text
CARGO_PROFILE_TEST_OPT_LEVEL=3 RUST_MIN_STACK=67108864
openwepp_runner-ee7cbb626e58bc3a --ignored --exact \
  hillslope::tests::cqr_laned_active_outputs::cqr_stage3_one_day_qualification_with_telemetry \
  --nocapture
```

Artifacts:

- log: `/tmp/adaptive_microstep_amendment/one-day-profile-detail-v19.log`
  (`sha256 16f38f1edc1b9700622947dec45d5eb165a5209da24779b0b9d2eb25dfa1e25c`)
- external time: `/tmp/adaptive_microstep_amendment/one-day-profile-detail-v19.time`
  (`sha256 fb9568969d6b1e0496efea5c49335876c5175364de995bd0c10b70887780597a`)
- binary: `sha256 385730cb76999cad6c236541386966d15c2276071ab269be262acf916dec87dc`

The test body was `416.94 s` (`416.96 s` external wall), `416.44 s` user CPU,
`0.41 s` system CPU, and 99% CPU utilization. Two preceding detailed runs were
`415.99 s` and `418.17 s`; the original v16 body was `420.11 s`. The detailed
timer layer therefore caused no observed material slowdown beyond ordinary
run-to-run variation.

## Exact behavior preservation

- accepted adaptive supports: `588`
- rejected trials: `320`
- retained publication supports: `1,078`
- events: `59`
- fixed-point nonconvergence rejections: `124`
- scaled comparison rejections: `140`
- exact-discrete comparison rejections: `0`
- ledgers validated: `2,037`
- maximum mass residual: `3.55271367880050093e-15 kg m^-2`
- maximum energy residual: `1.39698386192321777e-9 J m^-2`
- maximum receipt-reseal residuals: `9.98625182546675205e-10 J m^-2` and
  `1.06297193269710988e-11 K`

These values are identical to the accepted v16 evidence.

## Attribution

The parent executions consumed `381.486 s` (91.5% of the body); direct and
composed candidate evaluation consumed `375.584 s` (90.1%). The complete
multi-tile carrier runtime, including provisional iterations and
non-provisional replay, consumed `178.492 s` (42.81%). Its non-overlapping
major buckets were:

| Carrier operation | Time (s) | Body | Carrier |
|---|---:|---:|---:|
| potential tile physics solves | 80.373 | 19.28% | 45.03% |
| final authorized tile physics solves | 16.789 | 4.03% | 9.41% |
| WB14 surface ingress execution | 18.360 | 4.40% | 10.29% |
| surface-resource phase | 11.493 | 2.76% | 6.44% |
| soil candidate + receiver application | 5.503 | 1.32% | 3.08% |
| exact entry/protocol/custody validation and protocol/ingress construction | 38.772 | 9.30% | 21.72% |
| topology/request/post and measured residual overhead | 7.202 | 1.73% | 4.03% |

The validation/protocol row includes request preflight `3.218 s`, water
authorization `6.672 s`, exact derived-ingress entry validation `12.339 s`,
protocol validation `3.760 s`, protocol construction `8.379 s`, ingress
construction `4.015 s`, and candidate validation `0.389 s`. The second row
includes topology `0.992 s`, request batching `3.772 s`, post-validation
`0.333 s`, and `2.105 s` not covered by the nested child timers. The
classification is functional rather than a claim that hydrology state
construction contains no custody checks.

The fixed-point finalization/replay bucket consumed `102.533 s` (24.59% of the
body):

| Finalization stage | Time (s) | Share |
|---|---:|---:|
| converged-candidate rebuild | 32.163 | 31.34% |
| sealed-source envelope and replay | 46.623 | 45.47% |
| installed-candidate rebuild and residual audit | 23.733 | 23.15% |
| identity-only receipt replay | 0.014 | 0.01% |

About `51.213 s` of finalization is complete non-provisional carrier
re-evaluation (`178.492 s` all carrier runtime minus `127.279 s` provisional
physical runtime). Thus finalization is approximately half repeated physical/
hydrology evaluation and half exact sealing, reconstruction, and custody
verification. Identity-only resealing itself is negligible.

## Disposition

The workload is primarily repeated in-process solver and hydrology evaluation,
not external orchestration. Explicit process-state work accounts for roughly
`132.5 s` of carrier time, while explicit validation/protocol/bookkeeping
accounts for about `46.0 s`. Both are multiplied by the `54,753` fixed-point
iterations and by direct/composed/rejected candidate evaluation.

Ranked optimization targets, not implemented in this profiling increment:

1. reduce fixed-point evaluations and iterations, especially the 124 cap
   failures and refrozen-liquid comparison cascade;
2. eliminate provably redundant full non-provisional carrier rebuilds in the
   three finalization stages without weakening exact replay;
3. only after those multipliers fall, optimize WB14 surface ingress and exact
   entry/protocol validation.

Generic runner, archive, publication, and filesystem optimization remains a
low-value target for this workload.

## Focused validation

- `cargo fmt --all -- --check`: PASS.
- `cargo check -p openwepp-runner --tests`: PASS.
- covered convergence plus 96-cap focus: 17/17 PASS, nextest run
  `21583226-eb35-4f67-8033-33795666c89a`.
- exact canonical one-day qualification: PASS as recorded above.
- post-consolidation exact-current five-parent gate: PASS in `80.82 s`; binary
  `sha256 49b25e7b008d1540d9044b3b36be94fa195a9cc224f4b51bb9d486a492922952`;
  log `sha256 643282871d20c8272667113fa21e52f5c5ed255459328b4f5972e5ed69a51ae7`.
- `git diff --check`: PASS.

The post-run structural consolidation moved the flat detailed fields into
`snow_stage3_v11_profile.rs`; it did not move or alter any timed operation.
The exact-current five-parent output reproduces every detailed bucket and the
same typed bounded stop. `snow_stage3_v11_attachment.rs` is `2,981` lines after
the split, below the 3,000-line threshold.
