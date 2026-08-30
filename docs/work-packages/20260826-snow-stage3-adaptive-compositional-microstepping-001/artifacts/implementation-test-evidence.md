# Implementation and test evidence

Status: owner-amended exact-60 focused, restart, and canonical one-day evidence
PASS; runtime regression recorded.

Evidence mode: `Ran + Static`.

## Superseded evidence boundary

Static: every result whose grid, floor disposition, attempt count, event tick,
trace, or performance depended on the provisional 600-ms floor remains
superseded. It is retained only as historical evidence and is not counted
below. The current evidence uses the exact 60-second (`60_000_000_000 ns`)
floor. No entry below implies package completion or cutover.

## Exact-60 focused owner surfaces

The following results were reported from current amended-floor execution. When
the handoff retained an exact command, it is reproduced verbatim. Otherwise
the named test filter and result are recorded without inventing a command
envelope or elapsed time.

| Surface | Ran evidence | Result and admitted claim |
|---|---|---|
| coupled day support and poisons | `nix develop --command cargo test -p openwepp-hillslope-orchestrator --lib adaptive_day_support_gate -- --nocapture` | PASS, 5/5, 0.00 s test body. Exact 48-parent grouping, adaptive supports, authenticated event handoffs, omission/duplicate/reorder, parent/interval substitution, and subfloor/off-grid poisons. |
| LSE support admission | reported filters `v11_actual_stack_accepts_the_declared_lse_minimum_support` and `v11_actual_stack_rejects_one_tick_below_lse_minimum_before_newton` | PASS. Exact 60-second admission and `60_000_000_000 - 1 ns` pre-Newton rejection; exact command envelope was not retained in the handoff. |
| surface/root/terminal chronology | reported focused parent-end, terminal-endpoint timing/ledger, exact receiver, and rollback filters | PASS. Zero-duration parent-end transfer, prefix-plus-terminal composition, exact liquid/owner custody, and omission/substitution rollback; exact aggregate command was not retained. |
| ingress | reported filter `short_complete_owner_children_advance_persistent_cursor_once` | PASS, 1/1, 0.17 s test body. One persistent cursor advance across exact short complete-owner children. |
| WB14 | reported filter `two_900_and_thirty_60_children_cover_one_parent_and_advance_once` and focused finalization-placement poisons | PASS. One exact 1800-second WB14 parent, exact 60-second child grid, one final continuation advance, and nonfinal/final/restart placement poisons. |
| coordinator | reported filter `exact_sixty_second_proposal_grid_rejects_one_tick_below_and_admits_larger_support` | PASS. Exact floor, one-tick-below rejection, and larger-support admission; exact command envelope was not retained. |
| joint carrier/terminal-survivor helper seam | reported filters `batch_phase_enters_the_shared_engine_once_with_complete_lane_state` and `terminal_and_survivor_install_from_one_joint_candidate` | PASS, 1/1 each. Shared carrier entry and one joint candidate installation; this does not by itself close the full checkpoint-D production matrix. |

Ran: `cargo check -p openwepp-hillslope-orchestrator --lib` under the Nix
environment passed warning-clean in 6.62 s after the current focused increment.

## Stable production support evidence

| Test | Result |
|---|---|
| `stable_minimum_production_support_accepts_one_direct_trial` | PASS at exactly 60 seconds: direct `1`, split-child `0`, accepted `1`, rejected `0`, one `FloorAccepted` decision, and no composed carrier call. |
| `full_1800_second_production_constructor_large_step_qualification` (`#[ignore]`, run explicitly under the optimized test profile) | PASS at exactly 1800 seconds: direct `1`, split-child `2`, accepted `1`, rejected `0`. This is the required stable ordinary support substantially larger than the floor. |

The exact shell envelopes and elapsed times for these two earlier amended-floor
runs were not retained in the received handoff, so none are reconstructed here.
The asserted trial counts are part of the completed test results, not inferred
from an incomplete run.

## Reuse, memo, and forced-oracle equivalence

| Test | Result |
|---|---|
| `ordinary_physical_reuse_is_byte_identical_to_forced_double_evaluation` | reported PASS; ordinary reuse and forced double evaluation produced identical adaptive receipt bytes, supports, batches, complete owners, comparisons, publication topology, and controller evaluation topology. |
| `covered_physical_only_provisional_is_canonical_to_forced_full_envelope` | PASS, 1/1 in 0.98 s. Physical-only provisional evaluation equaled forced-full envelope receipts, support calls, terminal batches, publication topology, complete owner bytes, and comparison outputs; precipitation/boundary/soil/LSE omission or substitution changed the audit. |
| `reject_child_memo_is_canonical_byte_identical_to_forced_recomputation` | PASS, 1/1 in 12.33 s. Memoized and forced recomputation receipts, terminal batches, ending owners, and comparisons were byte-identical, while the memoized path performed fewer covered physical evaluations. |

## Restart and replay

Ran: optimized build:

```text
CARGO_PROFILE_TEST_OPT_LEVEL=2 RUST_MIN_STACK=67108864 cargo test -p openwepp-persisted-restart-v1 --features fixtures --lib --no-run
```

PASS in 2:28.87, peak RSS 5,675,040 KiB.

Ran: seven-posture exact-current sweep:

```text
timeout 240s .../openwepp_persisted_restart_v1-78f3ab104c698465 round_trips_and_resumes_byte_identically --nocapture --test-threads=1
```

PASS, 7/7 in 76.11 s test time; wall 76.21 s; peak RSS 899,140 KiB. The frozen
terminal multiplier `8.0411` reached every exact 60-second interruption hook.

Ran: cross-midnight continuation:

```text
timeout 360s .../openwepp_persisted_restart_v1-78f3ab104c698465 cross_midnight_owner_and_receipt_state_is_byte_identical_after_restart --nocapture --test-threads=1
```

PASS, 1/1 in 5.60 s test time; wall 5.70 s; peak RSS 203,316 KiB.

Ran: canonical V2 roundtrip and poisons:

```text
timeout 120s .../openwepp_persisted_restart_v1-78f3ab104c698465 production_attachment_round_trips_canonical_v2_and_rejects_poison --nocapture --test-threads=1
```

PASS, 1/1 in 0.06 s. Omission, owner substitution, ordinal/order, cross-parent,
and deferred-publication postures reject fail-closed.

Ran: exact-quanta/grid poisons:

```text
cargo test -p openwepp-hillslope-orchestrator restart_adaptive_trial_grid_tests --lib --features persisted-restart-v1
```

PASS, 2/2. Count, old-floor substitution, divisibility, remainder-range, and
parent-range poisons reject.

Reported additionally: default orchestrator, persisted-feature orchestrator,
and persisted-restart fixture library checks passed warning-free; formatting,
diff, and temporary-diagnostic scans passed.

## One-day production attempts

Ran: optimized one-day test build:

```text
CARGO_PROFILE_TEST_OPT_LEVEL=2 RUST_MIN_STACK=67108864 cargo test -p openwepp-runner cqr_stage3_one_day_qualification_with_telemetry --lib --no-run
```

PASS in 86.43 s; peak RSS 5,707,472 KiB.

Ran: a physical-only diagnostic prefix used the same built binary and filter
under an outer 120-second bound with an internal nine-parent stop. It reached
nine parent rows in 75.00 s, peak RSS 162,736 KiB, and intentionally exited
101 at the stop. Counts were unchanged and provisional receipts remained owner
phase zero. This was diagnostic timing evidence, not a qualification PASS.

Ran: intermediate complete one-day attempt:

```text
/usr/bin/time -f 'RELEASE_BODY_ELAPSED=%e RELEASE_BODY_MAXRSS_KB=%M EXIT=%x' timeout --signal=TERM --kill-after=15s 600s env RUST_MIN_STACK=67108864 /workdir/.cache/openwepp/targets/openWEPP-295c6e060aa9/debug/deps/openwepp_runner-04d98c7413d4179d cqr_stage3_one_day_qualification_with_telemetry --ignored --nocapture > /tmp/runner_cutover_telemetry/released-one-day.log 2>&1
```

**FAIL**, exit 101. The production controller completed exactly 48 parent rows,
1,435 accepted supports, and 49 event handoffs; parent 47 reported cumulative
487.609 s. The test body ended at 488.29 s with peak RSS 1,146,504 KiB, then
failed at the downstream typed gate:

```text
stage3_committed_publication: accepted Stage-3 unresolved liquid
```

This was not a timeout or completed publication. Later receiver/publication
custody corrections superseded this failure.

Ran: current pre-archive complete one-day qualification:

```text
/usr/bin/time -f 'ELAPSED=%e MAXRSS_KB=%M EXIT=%x' timeout --signal=TERM --kill-after=15s 600s env RUST_MIN_STACK=67108864 <optimized openwepp-runner test binary> cqr_stage3_one_day_qualification_with_telemetry --ignored --nocapture
```

PASS, 1/1: 48 parent supports, 1,435 accepted publication supports, 52 event
handoffs, committed qualification snapshot, WB13 downstream consumption, and
accepted/rejected fixed-point audit. Body 493.90 s, wall 493.96 s, peak RSS
1,761,440 KiB, exit 0. The retained log is
`/tmp/runner_cutover_telemetry/qualification-final-one-day.log`.

Ran: current compacted one-day qualification after the event-tail ending
endpoint correction:

```text
/usr/bin/time -f 'ELAPSED=%e MAXRSS_KB=%M EXIT=%x' timeout --signal=TERM --kill-after=15s 900s env RUST_MIN_STACK=67108864 /workdir/.cache/openwepp/targets/openWEPP-295c6e060aa9/debug/deps/openwepp_runner-04d98c7413d4179d cqr_stage3_one_day_qualification_with_telemetry --ignored --nocapture
```

**FAIL**, exit 101 after complete Stage3 execution: 48 parent supports, 1,435
accepted publication supports, and 52 event handoffs. Adaptive cumulative time
was 494.851 s; body wall was 500.18 s and peak RSS was 1,346,792 KiB. The
bounded qualification fold then rejected:

```text
qualification_day_delta: Stage-3/V11 attachment identity failure: qualification daily beginning endpoint
```

The final qualification snapshot, WB13 sink, archive-root/count, and public
output/spool transaction assertions were not reached, so this is not a
publication or qualification PASS. Logs are
`/tmp/runner_cutover_telemetry/archive-v3-one-day.log` and
`/tmp/runner_cutover_telemetry/archive-v3-one-day.time`; the exact-source
optimized build completed in 36.34 s with its evidence in the matching
`archive-v3-one-day-build.*` files.

Ran: exact compacted rerun after the authenticated day-start event bridge:

```text
/usr/bin/time -f 'ELAPSED=%e MAXRSS_KB=%M EXIT=%x' timeout --signal=TERM --kill-after=15s 900s env RUST_MIN_STACK=67108864 /workdir/.cache/openwepp/targets/openWEPP-295c6e060aa9/debug/deps/openwepp_runner-04d98c7413d4179d cqr_stage3_one_day_qualification_with_telemetry --ignored --nocapture
```

**FAIL**, exit 101 after 48/1,435/52 and 504.329 s cumulative adaptive
execution. Body wall was 510.37 s and peak RSS was 1,346,804 KiB. The
day-start bridge passed, then the nonempty route-qualified delta rejected:

```text
qualification_day_delta: Stage-3/V11 attachment identity failure: qualification canonical serialization
```

Static trace identifies a structured route key in the day delta's JSON-sealed
map as the real-data-only serialization seam; the seal must retain an explicit
canonical ordered route representation rather than omit the map. Logs are
`/tmp/runner_cutover_telemetry/archive-v3-one-day-rerun.log` and its `.time`
and `-build.*` siblings. The final snapshot, WB13 sink, archive-root/count, and
public output/spool assertions again remained unreached.

Ran: exact compacted rerun after strict structured-route wire, adaptive versus
snow-free cardinality, and cross-midnight post-receiver owner-lineage fixes:

```text
/usr/bin/time -f 'ELAPSED=%e MAXRSS_KB=%M EXIT=%x' timeout --signal=TERM --kill-after=15s 900s env RUST_MIN_STACK=67108864 /workdir/.cache/openwepp/targets/openWEPP-295c6e060aa9/debug/deps/openwepp_runner-04d98c7413d4179d cqr_stage3_one_day_qualification_with_telemetry --ignored --nocapture
```

**FAIL**, exit 101 after 48/1,435/52 and 486.771 s cumulative adaptive
execution. Body wall was 492.68 s and peak RSS was 1,346,492 KiB. All preceding
qualification seams cleared, then daily validation rejected:

```text
qualification_day_delta: Stage-3/V11 attachment identity failure: qualification daily ordered record identity
```

The current guard combines accepted-support, surface-receipt, and event vectors
under one error. Vector-specific zero/duplicate index and source identity must
be captured before changing semantics. Logs are
`/tmp/runner_cutover_telemetry/archive-v3-one-day-lineage.log` and its `.time`
and `-build.*` siblings. Seasons remain held and downstream snapshot, WB13,
archive-root/count, and public output/spool assertions remain unreached.

## Completed-day archive and bounded-residency increment

Season A was then terminated without a typed simulation error after 835.63 s,
at 9,859,504 KiB peak RSS, because the full adaptive receipt chain and accepted
publication history grew across days. This is a resource FAIL, not a seasonal
qualification PASS. The retained log is
`/tmp/runner_cutover_telemetry/season-a-full-finite-discrete.log`.

The current correction retains canonical uncompressed content-addressed day
evidence in a transaction-private durable spool, folds a sealed bounded prefix
and qualification accumulator, rotates accepted publication supports/events,
and materializes WB14 before severing predecessor history. Acknowledgement
occurs only after durable append and exact digest verification.

Ran:

```text
cargo test -p openwepp-hillslope-orchestrator committed_day_archive_tests --lib -- --nocapture
```

PASS, 2/2. Manifest and prefix tests reject omission, duplication, reordering,
truncation, content/prior-root/final-owner/day-count/sequence substitutions.

Ran:

```text
cargo test -p openwepp-hillslope-orchestrator accepted_publication --lib -- --nocapture
```

PASS, 7/7. Exact archive acknowledgement leaves zero resident completed-day
supports/events, retains sealed counts `1435/52`, and preserves canonical WB14
materialization and the final owner/root.

Ran:

```text
nix develop --command cargo test -p openwepp-runner stage3_v11_runner_streams_rows_and_durably_archives_each_day_without_batch_retention --lib -- --nocapture
```

PASS, 1/1 after a 37.87 s build. This source guard proves the production runner
streams rows, durably appends canonical day evidence, verifies its digest,
updates the manifest, and only then acknowledges; it also excludes the retired
batch-retention call. The concurrent zero-duration receiver-capacity work was
not yet fully wired during this run, so ten dead-code warnings from that active
module are recorded and no warning-clean claim is made.

These archive gates predate the owner performance amendment. Seasonal and
further archive work are paused and are not promoted as evidence for the
amended one-day count objective.

## Owner-amended fixed-point and one-day qualification

Ran on the terminal implementation:

```text
cargo test -p openwepp-hillslope-orchestrator exact_threshold_sides_are_fail_closed
cargo test -p openwepp-hillslope-orchestrator covered_fixed_point_exhaustion_at_96_is_fail_closed
cargo test -p openwepp-hillslope-orchestrator receipt_reseal_one_ulp_density_mapping_reenters_and_converges_before_install
cargo test -p openwepp-hillslope-orchestrator noncontracting_receipt_reseal_density_mapping_exhausts_96_fail_closed
```

PASS. Exact `TOL-SNOWENERGY-005` energy and temperature threshold sides,
bounded cap exhaustion, convergent one-ULP receipt reseal, and noncontracting
fail-closed behavior all passed.

Ran: the guarded five-parent real fixture passed in `105.54 s`. Its maximum
accepted installed-endpoint receipt residuals were
`9.66338120633736253e-10 J m^-2` and `1.42108547152020037e-12 K`. Retained log:
`/tmp/adaptive_microstep_amendment/contracted-causal-reseal-five-parent.log`.

Ran: exact-head canonical one-day production qualification:

```text
nix develop --command cargo test -p openwepp-runner cqr_stage3_one_day_qualification_with_telemetry -- --ignored --nocapture
```

PASS, 1/1 in `3118.32 s` unoptimized test time; external wall `3139.30 s`, peak RSS
`1,321,064 KiB`, exit 0. All 48 parents, the committed snapshot, downstream
publication consumer, archive fold, and output transaction passed. Exact
aggregate evidence is recorded in `one-day-microstep-performance-amendment.md`;
logs are `/tmp/adaptive_microstep_amendment/one-day-final-v3.log` and `.time`.

Ran after the terminal increment:

```text
cargo check -p openwepp-hillslope-orchestrator -p openwepp-runner --all-targets
cargo fmt --all -- --check
git diff --check
```

PASS. No temporary `eprintln!` remains in the production covered-snow solver.

## Restart V3 bounded archive-prefix and active-tail evidence

Ran on current custody/qualification source:

```text
nix develop --command cargo nextest run -p openwepp-persisted-restart-v1 --features fixtures -E 'test(adaptive_microstep_boundary_round_trips_and_resumes_byte_identically)'
```

PASS, 1/1 in 16.316 s. The same fixture also reseals and rejects independent
beginning-LSE and ending-LSE substitutions in the V3 support-liquid custody
supplemental. The bridge selector omission/owner-substitution poison passed
1/1 in the orchestrator suite.

Ran the four terminal event/receiver postures serially from the current fixture
binary:

```text
openwepp_persisted_restart_v1-7734825300647529 terminal_ --nocapture --test-threads=1
```

PASS, 4/4 in 61.30 s. Ran the two computationally long reappearance postures
separately and without contention under an external 960 s timeout: after snow
reappearance PASS, 1/1 in 554.95 s; before snow reappearance PASS, 1/1 in
535.46 s. These runs cross the former 417 s archive-prefix join failure and
prove the corrected actual-child-support/snow-free execution chronology.

Ran:

```text
nix develop --command cargo nextest run -p openwepp-persisted-restart-v1 --features fixtures -E 'test(cross_midnight_owner_and_receipt_state_is_byte_identical_after_restart)'
```

PASS, 1/1 in 97.782 s on independent rerun. The prerequisite qualification
join proves the exact 15 adaptive subslabs within a 17-publication mixed
adaptive/snow-free parent and binds effective owner lineage through the sealed
ordered same-tick publication event chain.

Ran:

```text
nix develop --command cargo nextest run -p openwepp-persisted-restart-v1 --features fixtures -E 'test(production_attachment_round_trips_canonical_v2_and_rejects_poison)'
```

PASS, 1/1 in 0.969 s. V3 projects the bounded active tail, custody supplemental,
publication rotation, archived prefix root/count, coupled owners/clock, and
materialized WB14 checkpoint. V2 wire bytes remain historical; live V2
projection rejects new custody instead of silently omitting it.

The seven interruption postures plus cross-midnight are therefore current V3
PASS evidence with exact archive root/count and active-output comparison.

Ran the isolated nonzero archived-prefix fixture:

```text
nix develop --command cargo nextest run -p openwepp-persisted-restart-v1 --features fixtures -E 'test(adaptive_microstep_boundary_round_trips_and_resumes_byte_identically)'
```

PASS, 1/1 in 19.512 s. A stable snow-free production day is executed,
published, committed, staged, and acknowledged as archive day 0 before restart.
The admitted state binds archive count/root `1`, exact `day_count=2`, and
`next_day_index=1`, then reproduces canonical V3 bytes. The same gate rejects
missing, truncated, same-length substituted, wrong-root, duplicated, and
reordered external archive evidence. It also reseals and rejects an empty
publication-rotation substitution, proving the empty resident V2 history cannot
stand in for the nonzero V3 sealed publication prefix. V1/V2 project and restore
behavior remains unchanged.

Ran after this increment:

```text
nix develop --command cargo check -p openwepp-persisted-restart-v1 --features fixtures
nix develop --command cargo clippy -p openwepp-persisted-restart-v1 --features fixtures --lib --no-deps -- -D warnings
nix develop --command cargo fmt --all -- --check
git diff --check
```

PASS: check in 12.81 s, Clippy in 13.95 s, formatting and diff hygiene clean.

## 2026-08-28 bounded archive resource evidence and workload pivot

Ran the exact current-source borrowed parent-receipt archive projection through
the nonzero V3 archive round-trip. PASS, 1/1 in 19.621 s; canonical decode and
re-encode remained byte-identical. Default and persisted-restart orchestrator
checks were warning-clean; runner all-target check, spool 8/8, archive prefix
2/2, formatting, and diff hygiene passed.

The optimized runner binary SHA-256 was
`7eb1f6e3f60a32d881b86d70d87cb096eb8b9fd6d5f4837b64adae7ae7d5256e`.
The 2-day archive gate passed in 1.31 s at 31,040 KiB peak RSS with
3,807,261 canonical and 535,940 stored record bytes. The 60-day gate passed in
1,663.16 s body / 1,663.26 s wall at 862,348 KiB peak RSS with 60 archived
days, 2,880 parents, 10,043,153,589 canonical bytes, and 755,824,894 stored
record bytes. Logs are
`/tmp/runner_cutover_telemetry/archive-v3-borrowed-{two-day,sixty-day}.{log,time}`;
the 60-day resource samples are in the matching `.resource` file.

Season A was then started under the fixed 21,600 s timeout and 1 GiB external
high-water guard. The owner performance-objective amendment arrived while it
was active, so PID 3079317 was terminated with SIGTERM. The process exited 15
after 731.29 s wall with 852,740 KiB peak RSS, no typed simulation failure, and
no memory-guard breach. This is `PAUSED/INCOMPLETE`, not qualification evidence;
logs are `/tmp/runner_cutover_telemetry/archive-v3-borrowed-season-a.*`.

## 2026-08-28 restart failure-cluster rerun

The adaptive restart gate from workspace nextest run
`8ec6202e-fafa-454a-8fc9-f9f2e621d149` was reproduced and corrected without
relaxing receipt, event, owner, or custody validation. The terminal receiver
fixture did not legitimately contain positive-support custody; the current
gate adds a warm nonterminal production day with real accepted support-liquid
custody and resealed beginning-LSE, ending-LSE, runoff-topology, and
runoff-disposition poisons. Ran with `RUST_MIN_STACK=67108864`:

```text
nix develop --command cargo nextest run -p openwepp-persisted-restart-v1 --features fixtures -E 'test(adaptive_microstep_boundary_round_trips_and_resumes_byte_identically)' --no-fail-fast --no-capture
```

PASS, 1/1 in 214.284 s, nextest run
`5ff8f7aa-5009-42e8-9280-995c35743df1`.

The two reappearance cases were not semantic failures: the workspace profile's
540 s limit was shorter than their current runtime. Each was run serially from
the exact focused libtest binary with a 64 MiB stack, one test thread, and a
1,200 s external fail-closed bound. `AfterSnowReappearance` passed 1/1 in
768.72 s test / 768.81 s wall at 1,673,092 KiB peak RSS.
`BeforeSnowReappearance` passed 1/1 in 754.86 s test / 754.90 s wall at
1,691,136 KiB peak RSS. Both exited zero.

After the semantic runs, the restart test module was mechanically split to
restore line-count compliance. Current-source `cargo test ... --lib --no-run`
passed in 6.22 s, `cargo check -p openwepp-persisted-restart-v1 --features
fixtures` passed in 13.72 s, and restart-lib Clippy with `--no-deps -D
warnings` passed in 16.96 s. Rustfmt and diff hygiene passed. The broader
all-target Clippy invocation reached pre-existing shared dependency findings in
`openwepp-coupled-time` and `openwepp-land-surface-energy`; it emitted no
restart-owned finding before that dependency failure.

## 2026-08-29 final WB14 factorization and one-day qualification

Ran: the transient five-parent audit first proved all 87 reported discrete
differences were transaction-local `ReceiptLineage`. After aligning the audit
with production, the same 87 differences were all
`surface_liquid.wb14_parent_working_state.per_ofe_authorities.ofe-1.receipts`
`ReceiptOrdering`; the exact values were 64-hex receipt digest keys. Logs:
`/tmp/adaptive_microstep_amendment/first-five-parent-comparison-audit.log` and
`first-five-parent-comparison-audit-post-lineage-fix.log`.

Static: `SC-SURFACELIQUID-001@13` and
`adaptive_receipt_container_kind` classify only that WB14 digest-key/history
surface and the existing child ordinal as exact per-trial factorization
lineage. Focused projection/audit tests passed 3/3, nextest run
`5d018cd1-1a65-40ec-943b-cfd3d39daaf9`. Both direct and composed trials retain
different exact-discrete digests; non-WB14 receipt membership/order remains a
typed mismatch.

Ran: the corrected five-parent real fixture passed in 122.09 seconds, nextest
run `1efec434-5e98-408d-979b-325f31359ba5`. It emitted 30 comparisons versus
94 before correction, zero exact-discrete mismatches, eight scaled errors over
one, and candidate widths `120 s x17`, `420 s x6`, `900 s x3`, and
`1800 s x4`.

Ran: the final optimized canonical one-day fixture passed 1/1 in 357.55 seconds
test body. External cold optimized rebuild plus test wall was 561.39 seconds;
the build-inclusive peak RSS was 5,894,016 KiB and is not model-residency
evidence. Logs are
`/tmp/adaptive_microstep_amendment/one-day-final-v7-opt.log` and `.time`.
It accepted 497 supports, rejected 206 trials, retained 975 publication
supports and 61 events, and passed all 48 parents, committed snapshot,
downstream consumer, archive fold, and output transaction. Widths were
`19x60`, `112x120`, `354x180`, `3x360`, `6x420`, `1x900`, and `2x1800`
seconds. Limiting counters were 155 fixed-point nonconvergences, 16 scaled
comparison rejections, and zero exact-discrete comparison rejections.
Independent maxima were `1.77635683940025046e-15 kg m^-2` mass,
`1.39698386192321777e-9 J m^-2` energy,
`9.98625182546675205e-10 J m^-2` receipt energy reseal, and
`4.37694325228221714e-12 K` receipt temperature reseal.

Ran after final source/contract rebinding:

- affected adaptive and SurfaceLiquid contract suites: 16/16 PASS, run
  `e6337da1-a9a8-4b06-9eb0-cc215a33ab05`;
- `cargo check --workspace --all-targets`: PASS;
- `openwepp-assurance validate --all`: PASS, 3/3 reports;
- authority anti-evasion shell guard: PASS;
- AUTH11 required-suite obligation guards: 3/3 PASS, run
  `b0e27df0-b257-4dc1-a96e-a6d508be30d5`;
- workspace rustfmt and `git diff --check`: PASS.

## 2026-08-29 terminal exact-head replacement

Ran: exact chronology, forcing, carrier-identity, and qualification-partition
corrections invalidate the earlier v7 performance result as terminal evidence.
The superseding final exact-head one-day run passed 1/1 in 420.11 seconds
model/test body after exact accepted-terminal custody composition. The
incremental optimized rebuild plus test wall was 489.99 seconds and
compilation-inclusive peak RSS was 3,935,368 KiB. Logs:
`/tmp/adaptive_microstep_amendment/one-day-final-v16-exact-head.log` and
`.time`.

The run accepted 588 supports, rejected 320 trials, retained 1,078 publication
supports and 59 events, and passed all 48 parents, the committed snapshot,
real downstream consumer, archive fold, and output transaction. Widths were
`139x60`, `111x120`, `320x180`, `12x240`, `1x300`, `3x420`, `1x900`, and
`1x1800` seconds. Limiting counters were 124 fixed-point nonconvergences, 140
scaled comparison rejections, and zero exact-discrete/event rejections.
Independent maxima were `3.55271367880050093e-15 kg m^-2` mass,
`1.39698386192321777e-9 J m^-2` energy,
`9.98625182546675205e-10 J m^-2` receipt energy reseal, and
`1.06297193269710988e-11 K` receipt temperature reseal across 2,037 ledgers.

This replacement invalidates v12 as exact-head evidence after the broad gate
exposed final-child-only terminal publication. The correction retains the
ordered exact physical carrier-phase chain, composes every child precipitation
parcel into one enclosing sealed set without changing mass/provider operands,
folds same-key water/N debits in physical-child order, retains every material
proposal, and binds the chain into pre-event authority. The CLI03 terminal heat
join now admits only the existing `TOL-SNOWENERGY-005` receipt regrouping bound;
the observed residual was `4.54747350886464119e-13 J m^-2`. The BGC sequential
ending versus enclosing-debit regrouping residual was exactly one ULP
(`1.1102230246251565e-16 kg N m^-2`); exact-bound and one-bit-over tests pass.
The EROD16 fixture cleared its former material ledger residual
(`0.039175350379679175 kg m^-2`, `13068.896886660974 J m^-2`) and progressed
until the long fixture timeout. The interior-terminal fixture cleared the
former material missing-debit error and reached its separately retained
snow-free LSE backtracking blocker. Restart boundary roundtrip passed 1/1 in
153.301 seconds, run `e9161973-87ad-4342-83c0-c57266f1928b`.

Ran: the CLI03 multi-OFE fixture now clears Stage-3 adaptive/subslab
qualification after positively overlapping successors were separated from a
boundary-starting same-parent tail. Adaptive partition tests pass 3/3,
terminal prefix/successor poisons pass 1/1, and production qualification debug
prints are absent. The fixture next reaches its unrelated historical
`UpStrmQ` assertion. Log:
`/tmp/adaptive_microstep_amendment/cli03-mf-qualification-cardinality-final.log`.

Ran: workspace all-target/all-feature check, rustfmt, and diff hygiene pass.
Four mechanically split parent files are below 3,000 lines; focused extracted
tests and the persisted-restart orchestrator check pass.

Ran: the final exact-head full-workspace profile after the accepted-terminal
custody correction executed 3,624 tests in 4,932.64 seconds: 3,490 passed, 105
failed, 29 timed out, and 48 were skipped; compilation-inclusive peak RSS was
3,190,500 KiB. The broad snapshot's failures partition exactly into 82
assurance tests blocked by the intentionally stale SnowEnergy source identity,
12 contract tests still pinned to SnowEnergy v25 or Vegetation v29, and 11
external science/fixture failures. No occurrence of `accepted terminal
integrated snow-soil heat identity`, `physical outcome ledger closure`, or
`V11 BGC mineral-pool delta` remains in the log. EROD16 progressed to its
720-second timeout; the interior-terminal fixtures progressed to the retained
snow-free LSE backtracking limit; DFF-WS2 progressed to the separately retained
196.46939 K open-snow lower-boundary domain at the exact floor. Log:
`/tmp/adaptive_microstep_amendment/final-v5-terminal-custody-exact-head.log`.

Ran: the typed assurance source-adoption workflow advanced generation
`931f2f31c529378f63377c3fda7ea1906654f5d2fd7ef114eb4b0b47e18fe809`
to `5c275785cc0af6681c2430b19857aa85166e4f16e3402a8b9b532385a8382a83`
for `SC-SNOWENERGY-001.md`. Repeating `--check` was a no-op and
`openwepp-assurance validate --all` passed all three reports. After updating
the canonical evidence pins and generation-chain receipt, affected contract
tests passed 75/75, assurance integration tests passed 113/113 with two
declared skips, and the assurance crate passed 32/32. Vegetation passed 279/279
on the exact v16 production source. Workspace all-target, all-feature check
passed. The combined warnings-denied Clippy gate remains non-passing with six
Vegetation and 990 orchestrator style findings across the active Stage-3
surface. A style-only cleanup was deliberately not retained because it
postdated the exact-head performance run; no lint was suppressed broadly and
this is not represented as a correctness failure or PASS.
