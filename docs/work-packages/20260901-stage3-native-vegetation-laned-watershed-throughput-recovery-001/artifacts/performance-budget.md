# Performance budget

Status: `TERMINAL HOLD — BOUNDED RECOVERY MEASURED; PERFORMANCE QUALIFICATION FAILED`

Evidence mode: `Static + Ran`

## Measurement host and source

- host: `ow-dev-01`;
- CPU: Intel Core i7-13620H, 10 physical cores / 16 logical CPUs;
- memory: 30 GiB;
- source: `a28c55c2d0f06e0c4aab58642f1009f70f82b3d3`, plus package-doc-only
  intake edits that cannot affect the binary;
- release test binary:
  `/workdir/.cache/openwepp/targets/openWEPP-295c6e060aa9/release/deps/dff_ws2_ksatadj_direct_runtime-fd56fd916f0fc0ce`;
- binary size 32,131,888 bytes; SHA-256
  `de8d94fd7c595cb077f7627af72c3f19e789319179caa7c6411eb44daed2988c`.

Ran:

`/usr/bin/time -v timeout 20m env RUST_MIN_STACK=67108864 nix develop -c cargo test --release --test dff_ws2_ksatadj_direct_runtime dff_ws2_forest_high_severity_loam_runs_with_live_direct_ksatadj_effect -- --exact --nocapture`

Log: `/tmp/phase1-baseline-logs/phase1_baseline_20260901T200857Z.log`,
SHA-256
`cd88148824402cc0ec93af01592f6b4e546421e5616cda723c521df93b968d0`.
The optimized test itself failed after 33.56 s at `1800..1860 s`; the complete
command took 335.64 s including a 4m58s release build, used 1,228.12 user
seconds during the parallel build, and peaked at 3,805,604 KiB. The run
recorded 34 Picard limiter samples and no completed day. Build cost and peak
build RSS are not runtime acceptance measurements.

This reproduces the failure mechanism in an optimized binary. It does not
provide an OFE-day rate because zero full days completed.

The exact release runner CLI was then built successfully:

`timeout 1200 env RUST_MIN_STACK=67108864 /usr/bin/time -v nix develop -c cargo build --release -p openwepp-runner --bin openwepp-cli-hill`

- binary:
  `/workdir/.cache/openwepp/targets/openWEPP-295c6e060aa9/release/openwepp-cli-hill`;
- SHA-256: `9bda52dd0f1b581428b45d1f18ed19e629a104026f46ab98a64053d12c852975`;
- build PASS, wall/user/system `259.94/669.16/6.85 s`, peak RSS
  `3,666,392 KiB`;
- log `artifacts/logs/phase1_build_openwepp_cli_hill.log`, SHA-256
  `dbbe7ca6d12a8d2214215cf4e861d53e20bb9f7d7660ebf5de584c86ab161062`.

Two production-executor CLI inputs failed during validation before day zero,
both with typed `CLIHILL-E-011` because no fixture/default Stage-3 owner seed is
admitted:

- native H.J. Andrews `p2`: target 16,438 days / 1 OFE, input bundle SHA-256
  `790790cca1e4a5071cfbc2d069db4e8689a90213bd04d7b024d9430de7a82b94`,
  wall `0.02 s`, RSS `20,492 KiB`, log
  `artifacts/logs/phase1_dff_ws1_run.log`, SHA-256
  `ef2c67d35f83b1f7d7e3861571f0b4b917593a25f01f48fd9f24d340c79f12f0`;
- H2637 Lane-D geometry `p2637`: target 732 days / 19 OFEs, wall `0.02 s`,
  RSS `12,112 KiB`, log `artifacts/logs/phase1_h2637_run.log`, SHA-256
  `258bc2883347f88a55062655b60e71632ea91947a2f311d51edb0c9ecb1bbc4a`.

These are valid real-runner expected reds and provide no throughput rate.

## Engineering SLO derivation

These limits are package-selected engineering qualification SLOs, not
process-science constants. The terminal 10-OFE/100-year hillslope wall limit is
`210 s`, selected so one worker can process at least 400 such hillslopes/day
before orchestration/I/O and so the declared 5,000-hillslope workload remains
roughly a day-scale parallel compute campaign on a 16-thread development host.
The `500 us/OFE-day` steady-state limit follows from the stricter `182.625 s`
CPU allocation over `365,250` OFE-days. Regime ceilings are deliberately
higher transient caps whose weighted long-run mean must still meet that
allocation. These limits never reject a physically required cadence; failure
to meet them is an engineering qualification failure.

The inherited path has no completed-day rate and therefore cannot establish
feasibility. Terminal acceptance requires the replacement to execute the full
workloads; baseline failure is retained only as severity and mechanism
characterization.

## Hard throughput budgets

Micro/day budgets use at least 30 measured batches after five warm-up batches;
each batch is long enough to exceed 250 ms, and bootstrap 95% confidence
intervals are reported for median and p95. Year and 100-year workloads use at
least five independent process runs after one warm-up and report every sample,
median, maximum, and coefficient of variation; no p95 claim is made from five
samples. Build time is excluded. The measurement pins one logical CPU where
practical, disables unrelated package profiling, records source/binary/input
hashes and counter overhead, and uses a monotonic in-process timer for
millisecond-scale surfaces plus `/usr/bin/time -v` for process CPU/RSS. A run
that does not complete is a failure and cannot be extrapolated.

The package now owns an executable release-only implementation of this
micro/day and scaling protocol:
`hillslope::tests::stage3_laned_release_qualification_matrix_1_10_19_ofe`.
It is ignored by default and must be launched explicitly in release mode on
one logical CPU, with `--nocapture --test-threads=1`. For each of 1, 10, and 19
OFE surfaces it launches a fresh child test process and executes five warm-up
batches followed by 30 measured batches;
each batch accumulates only completed
`execute_hillslope_run_with_runtime_policy` intervals until at least 250 ms of
monotonic in-process wall time. Fixture construction, owner-seed authoring,
validation, and cleanup remain outside the timed interval. Required
qualification and adaptive-parent counters remain enabled during the measured
interval; their overhead is included and never subtracted. The harness emits
every warm-up and measured batch as stable `STAGE3_RELEASE_QUALIFICATION` JSON,
then median/p95 summaries, deterministic 10,000-resample bootstrap 95% CIs,
`T10/T1`, `T19/T10`, and explicit hard-budget booleans. A final `PASS` requires
every applicable boolean, including the 10-OFE CPU and wall limits, 19-OFE
CPU/wall long-run-average limits, current-RSS and RSS-return checks, physical
map median/p95/maximum limits when the surface actually accepts ordinary
canonical-covered supports, attribution fractions, and Lane-D internal call
count/order. A terminal-only surface reports the canonical-map distribution as
not applicable and retains its authentic terminal direct/split trial counts;
the broader adaptive-support receipt count is never relabeled as an ordinary
canonical support count. A failed run is never extrapolated. Missing required telemetry produces
`INCOMPLETE` and a failing qualification test, not a partial pass.

The JSON protocol header records Git HEAD, an exact tracked-plus-untracked
source-tree SHA-256, the test-binary path and SHA-256, CPU affinity, Linux
process-CPU clock method and tick rate, deterministic bootstrap method/seed,
and counter posture. Each raw batch records the aggregate and per-file input
SHA-256 values, repetitions, wall and process-CPU timing, current RSS before
fixture allocation, at 1 ms intervals throughout fixture construction and the
active run, and after output cleanup for every iteration, plus isolated-process
`VmHWM`, adaptive receipts and candidate trials/rejections,
rejection categories, accepted-support width histograms, available overlapping
Stage 3 phase timings, routed-day/public OFE-order evidence, and independent
closure operands/residuals. Counters that are not exposed at this boundary are
emitted as `null` with an availability explanation; rejected candidate attempts
are not relabeled as rejected supports. The release qualification refuses to
start unless Linux process CPU conversion is available and affinity is exactly
one logical CPU. No qualification run or performance pass is claimed by this
harness-authoring increment.

All measured batches in a surface must have byte-identical aggregate/per-file
input and output-result hashes, result/closure identity, counters,
accepted-width histogram, and Lane-D public evidence. The summary construction
fails on any mismatch and retains the accepted identity and aggregate/per-file
input and result hashes in its stable JSON.

The per-iteration live-allocation return criterion is
`RSS_after_cleanup <= RSS_before_fixture + 8 MiB + 1 MiB * OFE_count`. The
fixed 8 MiB allowance covers allocator arena/high-water retention after the
five warm-up batches; the linear allowance covers one bounded lane/output
working set per OFE. This engineering tolerance is independent of the hard
active-memory ceiling and is checked for every measured iteration. The hard
active-memory observation is the maximum of the concurrent active `VmRSS`
samples and isolated-surface-process `VmHWM`; both must be available. This lets
`VmHWM` supplement the sampler for allocations freed between samples without
misusing it as return evidence. A missing `VmRSS` or `VmHWM` sample fails
closed. Only the before/after current-RSS pair proves return.

| Surface | Hard budget |
| --- | --- |
| Stable representative 10+-OFE long-run average | `<=500 us/OFE-day` CPU and `<=550 us/OFE-day` wall on one worker. |
| Snow-free native vegetation/ET | `<=250 us/OFE-day`. |
| Strictly frozen covered day | `<=750 us/OFE-day`. |
| Mixed-phase covered day | `<=1.5 ms/OFE-day`. |
| Thaw/refreeze or terminal-event day | `<=2.5 ms/OFE-day`. |
| Complete 10-OFE day | `<=5 ms` CPU and `<=5.5 ms` wall. |
| Native 1-OFE 365-day accumulation/persistence/meltout season | `<=0.182625 s` CPU. |
| Legacy-management 2-OFE 365-day reappearance/routing/BGC season | `<=0.36525 s` CPU; this is lifecycle/routing evidence and cannot carry the native-vegetation claim. |
| Complete representative 10-OFE 365-day year | `<=1.825 s` CPU. |
| Complete 10-OFE 100-year hillslope | `<=182.625 s` CPU and `<=210 s` wall. |
| Lane D scaling | `T_10/T_1 <=12`; `T_19/T_10 <=2.2`; no quadratic allocation or state growth. |
| Active memory | `<=128 MiB + 16 MiB * OFE_count` runtime RSS; temporary live allocation returns to that bound after each day. |

The representative long-run attribution budget is enforced with four exclusive
transaction-envelope buckets that sum exactly to the measured runner window:
the complete snow-free native-vegetation/ET successor envelope is a
conservative upper bound for native vegetation/ET and must be `<=35%`; the
remaining Stage-3 day-preparation/LSE/soil envelope after nested scopes is a
conservative upper bound and must be `<=40%`; successful Lane D route calls
must be `<=20%`; and all remaining runner work must be `<=20%`. These are
deliberately conservative envelope timings, not claims of isolated component
timing. The separately retained overlapping adaptive-parent phase timers are
diagnostic only and are not substituted for the exclusive acceptance buckets.

## Physical-map and iteration budget

- one canonical solver is selected before iteration for each contracted
  physical regime;
- at most eight charged physical-map evaluations per accepted support;
- median `<=2` and p95 `<=4` evaluations per accepted stable support;
- no receipt-cycle, ULP-lattice, historical solver, or exact continuous replay
  evaluations;
- support counts are reported, not capped. The contract's physical/event
  cadence remains authoritative; days requiring finer support must still meet
  closure and are engineering performance failures only when they exceed the
  applicable time SLO;
- budget exhaustion produces the same-solver adaptive response or typed error.

The evaluation cap is an upper safety bound, not permission to consume the
whole cap routinely. The old shared cap of 96 evaluations per support is
rejected as a throughput budget: r151 consumed seconds without completing the
first day.

## Watershed projection

The declared workload is
`5,000 * 10 * 365.25 * 100 = 1,826,250,000 OFE-days`. At the hard
`500 us/OFE-day` average it requires 913,125 CPU seconds (253.65 CPU-hours).
The terminal projection measures 1/2/4/8/16-worker local scaling rather than
assuming 70% efficiency. The planning example at 70% efficiency is 22.65
compute hours; a separately labeled 10% orchestration/I/O scenario is 24.9
hours. Neither percentage is acceptance evidence until measured. Hillslopes
are independent; no cross-hillslope numerical batching or wepppy scheduling
claim is made here.

The projection is accepted only after the single 10+-OFE 100-year hillslope
executes and 1/10/19-OFE measurements demonstrate the declared scaling.

## Comparator availability at post-implementation qualification

The package-required `comparator_suite_runner` was dispatched for the exact
post-handoff optimized one-OFE command and failed before command execution with
the service response:

`You've hit your usage limit for GPT-5.3-Codex-Spark. Switch to another model
now, or try again at 11:37 AM.`

The same role-level failure occurred on the earlier Phase-3 scale and Phase-4
profile dispatches. No comparator workload ran and no result is inferred from
those failures. Under the kickoff's explicit unavailable-role fallback, the
parent may run the exact bounded commands locally and must retain command,
timing, exit, and log evidence. Comparator-independent terminal review and
verification remain mandatory.

## Post-handoff bounded release probes

Ran on the same host with the exact ignored one-OFE positive real-runner
fixture:

`timeout 1200 /usr/bin/time -v env RUST_MIN_STACK=67108864 nix develop -c cargo test --release -p openwepp-runner --lib hillslope::tests::stage3_laned_release_one_ofe_positive_baseline_profile -- --ignored --exact --nocapture --test-threads=1`

Every completed probe reported the same accepted physical result:
`source=0.8488061229561478 m3`, `outlet=0.8471105124736579 m3`,
`end_storage=0.0016956104824910018 m3`, and `clamp=0 m3`. Each also
reported 48 parent supports (4 covered and 44 snow-free), 56 accepted
publication supports, 4 accepted microsteps, 20 direct trials, 32 split-child
trials, and zero fixed-point evaluations.

| Source state | `run_wall_us` | Change from first probe | Result |
| --- | ---: | ---: | --- |
| post-handoff baseline | 36,377,997 | baseline | PASS |
| physical-history comparison, no rollback-history reserialization | 22,994,809 | -36.8% | PASS |
| validated in-process frozen-litter resident installation | 13,775,128 | -62.1% | PASS |
| cached already-replayed V4 owner projection bytes | 13,198,562 | -63.7% | PASS |
| sealed shared covered-carrier owner-byte handoff | 12,436,785 | -65.8% | PASS |

The final command's test phase completed in 12.47 s after a 5m34s release
rebuild; its kernel timer excludes build, fixture, bootstrap, and validation
work. The peak 4,160,736 KiB process RSS belongs to the parallel release build,
not the runtime-only test phase. This is a real, reproducible improvement, but
the full mixed-regime one-OFE fixture is not the pure snow-free surface
governed by the `250 us/OFE-day` micro budget and is not terminal throughput
acceptance. The sealed handoff is bound to the exact native-V2
unpublished-trial digest, and the existing stale-carry substitution proof
passes; selected carriers share the immutable proof rather than cloning full
owner maps.

Three bounded interrupt samples of the final optimized release binary found
the worker in (1) repeated canonical owner JSON serialization while assembling
covered-carrier typed owner bytes, (2) the real covered LSE solve, and (3)
V8/V9/V10 vegetation canonical-digest validation during trusted V11 parent
finalization projection. These samples guide further authority-preserving
validation/custody work; they are attribution evidence, not a statistical
profile or permission to weaken physical cadence.

## Historical 2026-09-02 budget disposition

Terminal correctness source/diff digest: `9578803c432cef0f53a70d870b604b2f4c668d3acea99a35317d7cd15136adfe`;
release test binary SHA-256:
`3a94e7ecb52e20c5897e0303cf8abebfc67f678f3f9f5547a622751a27d36d7d`.
The full 1/9/10/19-OFE release real-consumer matrix passed in `320.13 s`.

At that checkpoint, the best measured full mixed-regime one-OFE day was
`12,436,785 us`. It is a diagnostic measurement and cannot be compared to the
pure snow-free `250 us/OFE-day` budget or the 10-OFE complete-day ceiling.
Year/century qualification was not run. Correctness evidence was retained and
throughput was `HOLD`. The source/binary identities in this section are
historical; the continued-recovery measurements below contain later bounded
evidence.

## 2026-09-03 continued recovery measurements

The user-directed continuation reopened performance correction after the
preceding hold. All values below are the in-process `run_wall_us` from the same
one-OFE ignored release test, pinned to logical CPU 0 with release LTO disabled
for repeatable edit-loop builds. Every retained run reproduced exactly the
same source, outlet, end-storage, clamp, parent/support, publication, and
adaptive-trial values stated above.

| Retained source state | `run_wall_us` | Change from 8,487,xxx-us pre-continuation reference | Result |
| --- | ---: | ---: | --- |
| LSE duplicate final validation removed; rejected zero-hit memo absent | 8,392,062 | about -1.1% | PASS |
| allocation-reduced byte-identical vegetation canonical encoder | 7,101,591 | about -16.3% | PASS |
| same-call soil digest, Surface V2 proof, and frozen-publication constructor reuse | 6,886,447 | about -18.9% | PASS |
| enclosing phase telemetry, no behavior change | 6,905,478 | about -18.6% | PASS |
| private physical-result V9-to-V8 projection handoff | 6,805,935 | about -19.8% | PASS |
| same-call constructed V10 state admission | 6,680,640 | about -21.3% | PASS |
| validated inactive surface-resource handoff | 6,598,510 | about -22.3% | PASS |
| typed nonfinal WB14 parent retention comparison | 6,336,812 | about -25.3% | PASS |
| atomic validated V4 projection/canonical-byte construction | 5,930,859 | about -30.1% | PASS |
| discard no same-call validated V3 projection round-trip | 5,792,447 | about -31.8% | PASS |
| borrowed normalized V10 endpoint-owner proof | 5,693,455 | about -32.9% | PASS |
| move-only physical-producer soil-ending custody | 5,615,489 | about -33.8% | PASS |
| validated V11 parent-finalization V10/V9 handoff | 5,484,820 | about -35.4% | PASS |
| typed V4 construction without immediate self-reparse | 5,343,745 | about -37.0% | PASS |
| same-trial borrowed V9-to-V8 physical projection proof | 5,219,037 | about -38.5% | PASS |
| potential-request construction without immediate digest replay | 5,211,924 | about -38.6% | PASS |
| candidate-bound validated soil-read capability | 5,204,454 | about -38.7% | PASS |
| one immutable covered-evaluation admission per canonical solve | 5,156,696 | about -39.2% | PASS |
| exact represented-snow ground/soil identity-anchor Jacobian probes | 5,005,555 | about -41.0% | PASS |
| exact same-evaluation leaf maximum-demand reuse | 4,964,642 | about -41.5% | PASS |
| exact four-column hydraulic probe recomputation | 4,958,688 | about -41.6% | REVERTED after exact-head repetitions failed target-plus-total retention |
| exact-head retained V29 after hydraulic shortcut removal | 4,936,273 | about -41.8% | PASS |

The first enclosing attribution found that all snow-free imported-stack work
was about `578 ms`, consistent with the `628 ms` conservative native
vegetation/ET envelope. Terminal direct plus composed trials consumed about
`4.0 s`. Deeper attribution localized `2.74 s` to covered provider-carrier
execution: about `1.77 s` physical prefix and `0.97 s` complete-owner
continuation after the retained projection handoff. In the final retained
probe before the last two corrections, physical evidence was about `1.41 s`;
complete-envelope construction was `298 ms`, candidate adoption `114 ms`,
projection/lineage `241 ms`, and owner sealing `317 ms`. Same-call constructed
V10 admission then reduced projection/lineage to about `146 ms`. The private
validated inactive surface-resource handoff reduced its immediately adjacent
surface-ingress bucket from about `93.7 ms` to `57.8 ms` without changing the
ordinary public validator. Comparing the exact same-call typed WB14 parent
field instead of serializing and reparsing both complete owner sets reduced
terminal provider retention from about `309 ms` to `15.8 ms`.
The atomic same-call V4 construction/byte handoff preserved one complete
constructor validation plus every public/restart/external parser check while
removing two redundant nested replays. Its targeted snow-free frozen-runtime
bucket fell from about `296.9 ms` to `201.2 ms`, and whole-run time improved
another 6.4% from the prior retained best.
The private-field V3 constructors also complete full validation before return;
removing the runtime's subsequent serialize/reparse/compare-and-discard cycle
reduced the same snow-free core again from about `201.2 ms` to `153.7 ms` and
whole-run time by another 2.3%. Public V3 canonical parsing and the later V4
nested replay remain unchanged.
Endpoint-exclusive telemetry then attributed `97.6 ms` of the covered-carrier
owner bucket to repeating full V10-to-V9/V8 validation after exact lineage
normalization. A private borrowed proof verifies V10 model/configuration
identity, all normalized transaction-bearing fields, and the recomputed
canonical digest, and is consumable only against the exact same state/config
references before mutation. It reduced that validation bucket to `13.5 ms`,
the owner bucket from `322.6 ms` to `232.7 ms`, and whole-run time another 1.7%.
The remaining endpoint telemetry then isolated about `147.1 ms` in a second
full authentication of the exact soil candidate/continuation pair already
authenticated by the physical producer. A private move-only, non-wire soil-
ending token now keeps that pair inseparable through the physical result and
complete trial and is consumed at the endpoint. Public, restart, external, and
independently supplied custody paths remain fully authenticated. The soil-
custody bucket fell to `23 us`, the complete-owner bucket to `87.9 ms`, and
whole-run time improved another 1.4%.
V11 parent finalization already performed the contract-required full
validation after its lineage mutation, but its immediate trusted installation
then repeated V11-to-V10 and V10-to-V9 validation. The existing
`INV-VEGETATION-134/C-006` handoff now retains the exact validated pre-lineage,
finalized V11, V10, and derived V9 revisions in one move-only, non-wire proof.
The two trusted production finalizers consume it only against exact
configuration/beginning/ending equality; the ordinary public projection and
all untrusted/restart/external boundaries retain full validation. Whole-run
time improved another 2.3% to `5,484,820 us`.
The typed V4 constructor now performs the same nested identity, mirror,
operand, and receipt checks directly against its supplied typed values and
retains the ordinary canonical parser for wire, restart, mutation, and
external inputs. Two consecutive same-binary release probes measured
`5,345,492 us` and `5,343,745 us`; the targeted snow-free frozen-runtime
bucket fell from about `153.7 ms` to `110.4 ms` and `109.8 ms`, respectively.
The latter is the current retained best, with every exact output and workload
count unchanged.
The carrier then retained that same trial's already fully validated V9-to-V8
projection in a private borrowed proof through native physical preparation.
Pointer identity prevents use with any other configuration/state revision;
the values move onward into the existing physical result after the borrow is
consumed. Two same-binary probes measured `5,237,702 us` and `5,219,037 us`;
the targeted physical-evidence bucket fell from about `1.385 s` to `1.287 s`
and `1.280 s`, respectively, with exact output and count parity.

An attempted single retained owner-byte map produced `6,852,619 us` and an
owner bucket of `321 ms`, which was not an improvement over measurement noise;
that structural experiment was reverted. A later zero-work arbitration
shortcut moved its targeted resource bucket by only about `1 ms` while an
unrelated physical bucket moved by `14 ms`; it too was reverted as
noise-dominated. Removing an immediate complete-envelope validation whose
constructor already validates also failed to move its target bucket and was
reverted. Borrowed raw-JSON embedding of the already-canonical surface owner
was byte-identical but moved its target only from `316.8 ms` to `316.3 ms`
while whole-run time slightly regressed to `5,798,250 us`; the code and serde
feature were reverted in full. A weaker borrowed soil-candidate proof targeted
only the adjacent read-view replay: candidate-byte work was merely `2.5 ms`,
the owner bucket regressed from `232.7 ms` to `240.4 ms`, and whole-run time
regressed to `5,784,124 us`; it was reverted before the producer-owned token
was implemented. Extending the same-call validated V8 configuration/beginning
proof across the enclosing final receipt projection reduced the complete-
envelope bucket from about `298.0 ms` to `271.4 ms`, but adjacent physical and
vegetation work moved upward and whole-run time regressed from `5,484,820 us`
to `5,493,146 us`; the extension was reverted in full under the target-plus-
total retention rule. A narrower correction that omitted only the binding
validator's repeated configuration check moved the target from about
`298.0 ms` to `296.0 ms` while whole-run time regressed to `5,506,858 us`; it
was also reverted. A Surface V2 replacement fast path then deferred success-
irrelevant error-context serialization and removed repeated constructor/wrap
validation. It reduced frozen runtime from about `155.3 ms` to `145.0 ms`, but
terminal carrier work moved upward and whole-run time regressed to
`5,508,644 us`; source and its prospective write-set entry were reverted. A
test-only audit gate was retained
because it prevents expensive canonical projection/sensitivity payload
construction when its thread-local unit-test audit is unarmed; it does not
affect the runner probe because Cargo dependencies are compiled without
`cfg(test)`. Its guarded forced-complete parity tests passed `2/2` serially.

The current `4.921 s/OFE-day` mixed-regime result is improvement evidence, not
snow-free micro, complete-day, qualification, or closure evidence. The
completed prior-source 1/10/19-OFE matrix remains a performance/RSS `FAIL`
despite passing exact science and scaling checks; its 10-OFE complete-day and
19-OFE long-run-average surfaces are the applicable absolute-budget evidence.

## Rejected terminal first-child memo

A private single-use memo attempted to reuse the first composed terminal child
as the next refined direct candidate. Its exhaustive serialization-key form
passed focused authority/real-terminal tests and exact output parity but
measured `5,320,227 us`; replacing redundant full parent, consumer, and Stage-3
reserialization with the already-sealed adaptive beginning-owner authority
measured `5,264,846 us`. Neither beat the retained `5,219,037 us` total or
decisively reduced the terminal target buckets, so the memo source and tests
were reverted in full. Raw logs are
`artifacts/terminal-heavy-gates/terminal_trial_memo_one_ofe_release.log` and
`terminal_trial_memo_sealed_key_one_ofe_release.log`.

## Retained potential-request validation-once increment

`PotentialWaterRequestBatch::try_new` previously computed and installed the
canonical request signature and then called public `validate()`, which
recomputed that same signature before any mutation or trust boundary. The
constructor now preserves its original error order—canonical digest first,
then exact transaction/cardinality/key/finite/nonnegative/uniqueness checks—
but performs the latter through a private content validator. Public
`validate()` still performs those checks plus a fresh canonical digest and
therefore retains mutation, restart, and untrusted-input detection. The two
in-crate open/covered producers now use the same constructor rather than
manually repeating its digest-plus-validation sequence.

Ran 2026-09-04:

- LSE transaction-focused nextest filter: `28/28` PASS;
- authentic two-active-lane terminal/survivor test: `1/1` PASS;
- `cargo check -p openwepp-runner`: PASS;
- exact CPU-0 ignored release one-OFE probe: PASS at `5,211,924 us`, versus
  the retained `5,219,037 us` best; potential-phase time was `601,092 us`;
- exact source/outlet/storage/clamp values remained
  `0.8488061229561478/0.8471105124736579/0.0016956104824910018/0 m3`, with
  48 parents, 56 publications, 20 direct trials, 32 split children, and 4
  accepted microsteps unchanged.

The `7,113 us` whole-run improvement is small and may overlap ordinary timing
variation, but the intended duplicate canonical work was removed, its target
bucket moved downward from the approximately `646 ms` nearest logged control,
and the exact run did not regress. The increment is retained as bounded
behavior-preserving cleanup, not as evidence that the throughput gate is near
passing.

An immediately adjacent experiment then consumed the private-field multi-tile
potential candidate without replaying public aggregate-batch validation. The
canonical parity/poison tests passed `2/2`, the authentic terminal test passed,
runner check passed, and the release probe preserved exact outputs/counts.
Authorization moved from `25,232 us` to `17,113 us`, but the whole run measured
`5,221,788 us`, above the retained `5,211,924 us`. The source and prospective
write-set description were therefore reverted under the target-plus-total
rule; this is rejected/noise-dominated evidence, not a retained optimization.

## Retained candidate-bound soil-read capability

The carrier's private ephemeral candidate already sealed the exact V1 soil
value or V2 unpublished-trial digest after typed candidate construction. The
physical preparation then authenticated the candidate and continuation before
calling `DirectSoilThermalReadView::validate()` again on that same immutable
ending state. The carrier now mints a private, borrowed, non-Clone/non-wire
capability only after the seal matches. Its read view is usable only for the
pointer-identical candidate during the same physical call; calls without
the capability retain the original full validation. Public constructors,
restart/external admission, continuation authentication, and poison paths are
unchanged.

Ran 2026-09-04:

- soil continuation reconstruction/lineage/substitution/support/receipt/carry
  plus canonical physical parity/rollback filter: `6/6` PASS;
- authentic two-active-lane terminal/survivor test: `1/1` PASS;
- orchestrator and runner checks: PASS;
- formatting and diff hygiene: PASS;
- exact CPU-0 one-OFE release probe: PASS at `5,204,454 us`, with
  `carrier_physical_evidence=1,264,577 us`; the preceding retained values were
  `5,211,924 us` and `1,267,495 us`.

The exact source/outlet/storage/clamp values and 48-parent/56-publication/
20-direct/32-split/4-microstep counts remain unchanged. Both measured deltas
are noise-scale despite removal of a genuine redundant replay, so this is
retained cleanup rather than evidence of material throughput recovery.

A subsequent allocation-only experiment moved each covered phase's validated
`PotentialCoveredVegetationOperands` into the accepted aggregate after the
fixed-final solve, replacing a deep clone immediately before the original
phase was consumed. The V3 module/canonical parity/rollback filter passed
`6/6`, the authentic terminal test and runner check passed, and exact release
science/counts were unchanged. Nevertheless, physical evidence increased to
`1,275,074 us` and total time to `5,253,829 us`, regressions of `10,497 us` and
`49,375 us` from the retained best. The source and prospective write-set entry
were reverted in full; this is allocator/layout-sensitive negative evidence.

## Retained same-solve covered-evaluation admission

Every Newton, finite-difference, prospective, and backtracking evaluation had
revalidated the unchanged covered-column topology, authorization caps,
shortwave authority, and Stage-3 optical/lower-boundary identity. Under
`SC-LANDSURFACEENERGY-001#INV-LANDSURFACEENERGY-159/C-014`, the canonical
solver now mints one private borrowed proof after the existing initial-trial
guard and reuses it only inside that solve. Every trial still receives its
fresh shape, bounds, constitutive, residual, tolerance, branch, and error
evaluation. Public `evaluate_covered_column`, V3 evaluation calls, and every
independent input retain full immutable admission. Probe coordinates/order,
finite differences, equations, LU/pivot order, backtracking, convergence, and
accepted results are unchanged.

Contract-derived pre-implementation evidence was expected red only on the
absent validation-count audit API. After implementation:

- the count test proves exactly one immutable admission across one complete
  canonical solve;
- the full LSE crate passed `139/139`;
- canonical physical parity/rollback plus authentic terminal tests passed
  `3/3`;
- runner check, formatting, and diff hygiene passed;
- the exact CPU-0 release probe preserved all science/counts at `5,156,696 us`.

The targeted potential bucket fell from `596,235 us` to `532,983 us`, physical
evidence from `1,264,577 us` to `1,203,666 us`, and total time from
`5,204,454 us` to `5,156,696 us`. This `47,758 us` whole-run improvement is
material relative to the immediately preceding millisecond-scale increments,
but the day remains orders of magnitude outside every applicable SLO.

## Retained Stage-3 identity-anchor Jacobian probes

Contract version 28 now binds the represented-snow ground and soil equations
as exact identity anchors. After complete immutable Stage-3 input admission,
their coordinates affect only the matching normalized residual. The canonical
solver still constructs and domain-checks the exact minus-then-plus probes and
passes the resulting values through the unchanged centered or inward
finite-difference operation. Only the complete constitutive reevaluation of
those dependency-independent probes is removed; all other columns, regimes,
base evaluations, backtracking, LU/pivot order, tolerances, and errors remain
unchanged.

Ran 2026-09-04:

- contract-derived expected-red compilation failed only on the absent anchor-
  probe and audit APIs;
- the bitwise oracle then passed for every ground/soil anchor at an interior
  value and exact `200 K`/`350 K` closed bounds, comparing every normalized
  residual and dense Jacobian entry with the complete evaluator; non-anchor
  audit count remained exactly two full centered evaluations;
- full LSE: `139/139` PASS;
- forced-complete represented-snow physical-prefix parity and rollback poison:
  `2/2` PASS;
- `cargo check -p openwepp-runner`, formatting, and diff hygiene: PASS;
- exact CPU-0 release probe: PASS at `5,005,555 us`, with potential
  `390,208 us` and physical evidence `1,055,762 us`.

Private leaf maximum-demand reuse retained the same outputs and counts while
reducing total/potential/physical evidence to
`4,964,642/347,810/1,019,292 us`. A subsequent shared-hydraulic probe
experiment initially measured `4,958,688/345,669/1,014,050 us`, but later
exact-head repetitions measured `4,997,200`, `4,968,488`, and `4,984,552 us`
total with potential between `361,991` and `363,133 us`. Because the target
and total did not both improve reproducibly, the shortcut and prospective v30
contract were removed. The final retained V29 source measured
`4,936,273/350,569/1,020,838 us` on its terminal warm repetition.

The preceding retained values were `5,156,696/532,983/1,203,666 us`. Exact
source/outlet/storage/clamp remained
`0.8488061229561478/0.8471105124736579/0.0016956104824910018/0 m3`; 48 parents,
56 publications, 20 direct trials, 32 split children, and 4 accepted
microsteps were unchanged. This is retained exact dependency reuse, but the
one-OFE mixed-regime day remains diagnostic and is not adjudicated against the
pure snow-free micro budget.

## Rejected shared terminal-carrier custody experiment

Static + Ran 2026-09-04: a private non-wire shared-beginning experiment
replaced repeated deep `DirectV10RealConsumerShadow` copies with immutable
`Arc` custody, Rc-backed provider generations, borrowed exact/fallback
selection, and one explicit post-physical mutable ending materialization.
Contract-derived behavioral tests, dual Rust review, formatting, focused
correctness, poison/rollback, and orchestrator/runner checks passed. The
candidate Rust manifest was
`40b5ec577d8a8a706f493ae6b41527eef3df52ba052247aaea6b3ad981f38a4f`;
the exact release test binary was
`9cc920765ba5a09ce3c09d9e65c19989599bad9ab1ae99affb6a3f6ab2ba6323`.

Three exact CPU-0 warm release diagnostics preserved the same source, outlet,
storage, clamp, and `48/56/20/32/4` counts:

| run | total | provider custody | carrier complete adoption | carrier complete owner | provider retention |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | `4,929,640 us` | `1,431 us` | `125,405 us` | `78,336 us` | `13,237 us` |
| 2 | `4,912,984 us` | `1,439 us` | `126,345 us` | `80,305 us` | `13,339 us` |
| 3 | `4,918,547 us` | `1,506 us` | `125,883 us` | `78,576 us` | `13,210 us` |

Against the retained V29 diagnostic total of `4,936,273 us`, the observed
gain was only about `6.6--23.3 ms`, below the prospectively declared
`0.10 s` minimum recoverable benefit. Target custody improved, but total and
adjacent completion buckets showed that the opportunity was not material.
The experiment was therefore rejected and reverted in full; its results are
diagnostic evidence only.

Post-revert exact-source confirmation used Rust manifest
`f065ecdb039ce69a0ebb38f08e958ad8152bd9d89ff981ee470787d559a3c1cb`
and release binary
`5d70ce0966222480e20f12a468e47fc32fdc136e77e90a87334270a3289e6564`.
The warm diagnostic passed at `4,920,992 us`, RSS `60,708 KiB`, potential
`352,898 us`, carrier physical evidence `1,018,388 us`, provider custody
`10,176 us`, carrier complete adoption `124,297 us`, carrier complete owner
`87,223 us`, and provider retention `15,656 us`. Exact closure and
`48/56/20/32/4` counts remained unchanged. This confirms the reverted V29
source; it remains a mixed-regime diagnostic rather than qualification.

## Rejected outer terminal-trial finalization candidate

Static 2026-09-04: the apparent `752,618 us` outside-carrier envelope cannot be
removed by retaining only physical terminal endpoints. Composed child 2 reads
child 1's finalized consumer, private accepted clock, complete-owner digest,
WB14 state, physical ledger and receipt identities. Terminal-event comparison
also requires final lane/destination receipts and the accepted zero-duration
event mutation. Reconstructing these products in a pending token would recreate
most of the complete temporal/owner shell and introduce a new high-risk
identity surface. The candidate was rejected before a completed contract or
production amendment; no timing gain is claimed. A narrower shared precomputed-
payload idea remains unselected because static evidence does not establish the
prospective `0.10 s` benefit.

## Rejected pure carrier structural-admission candidate

Ran 2026-09-04: exact CPU-0 attribution preserved the canonical science values,
`48/56/20/32/4` counts, and completed in `4,984,488 us` at `62,560 KiB` RSS.
The safely reusable immutable subtotal was only `39,740 us`: forcing
normalization `335`, reseal `7,671`, full validation `3,983`, later hash
`3,947`, and V8 static configuration/binding/index work `23,804 us`.

The separately measured `63,319 us` native-V3 validation/hash bucket is not
reusable static work: it validates the current LSE state and canonicalizes the
current surface beginning on every map. Including it would reach `103,059 us`
only through a separately authorized validated-resident handoff; it is not
cross-map static work. The pure-static candidate therefore fails the
prospective `0.10 s` pre-kill rule. Raw evidence:
`artifacts/terminal-heavy-gates/carrier_static_attribution_one_ofe_release.log`.

## Rejected combined parent-static and validated-resident attempt

Static + Ran 2026-09-04: contract v30 mapped the combined `103,059 us`
upper-bound target to existing `INV-LANDSURFACEENERGY-159` and passed dual
independent contract review and verification. The source-real native component
uses the existing `FrozenLitterV3Resident` validated revision at the native
validation position; V8 does not attest to those distinct V3/V2 objects.

The bounded production attempt was rejected before benchmarking. The authentic
52-map workload crosses multiple adaptive terminal, batch, single, and
canonical-subslab stacks whose configuration objects are deep-cloned. A
per-stack plan therefore cannot establish one stable parent generation or
pointer-bound structural identity. Meeting the contract requires creating the
plan at the real parent lifetime and threading a stable borrow through the full
executor graph, while minting the exact forcing proof before the
ordinary/native split and preserving each replaced check's error position.
That is materially broader than the selected bounded increment. All temporary
production code was reverted; formatting, orchestrator check, diff hygiene,
and symbol-residue checks passed. No release run or performance gain is
claimed. Detailed disposition:
`artifacts/science-contracts/carrier-parent-static-validation-once/implementation.md`.

## Rejected terminal precomputed-package sharing candidate

Ran 2026-09-04: exact CPU-0 release attribution at Rust manifest
`b71377b40c408db71ce49a6ae1280f57100df2447936c673a0001896dff4c73d`
and release binary
`2300fbcfc62efd88fb5d70c8cc798dd2f33d79ce3266b9437f18cb30365bc968`
proved 20 slab-independent physical-payload groups were rebuilt across
distinct accepted-slab bindings. The identity prerequisite therefore passed.

The opportunity was not material: all 108 authentic package constructions
consumed only `29,937 us`. The largest exclusive buckets were carrier-phase
cloning (`14,590 us`) and vector cloning (`13,433 us`); map cloning was
`327 us`, chain validation `118 us`, inclusive endpoint coalescence `388 us`,
assembly `6 us`, pre-event sealing `948 us`, and diagnostic identity evidence
`140 us`. Test-audit cloning was inactive. Exact source/outlet/storage/clamp,
complete telemetry, `48/56/20/32/4` counts, and `61,212 KiB` RSS were retained.

Because even the entire construction envelope is less than one third of the
prospective `0.10 s` floor, the shared-payload candidate was rejected without
a custody contract or production edit, and all profiler-only Rust changes were
reverted. Raw selected evidence is in
`artifacts/terminal-heavy-gates/terminal_precomputed_attribution_one_ofe_release.log`.
The post-revert Rust manifest is
`6e3f339d35ac2efa859bc51b7f08ce611dcd127800a54d83d6a781c2353ca906`;
an exact profiler-symbol search returned no production or runner residue, and
formatting, orchestrator check, runner check, and diff hygiene passed.

## Rejected post-v30 bounded candidates

Static 2026-09-04: four independent source audits excluded three further
bounded production increments before contract or implementation work:

- topology move-only projection has an exact measured native projection ceiling
  of `10,246 us`; even incorrectly treating the complete fresh V8 dynamic
  projection (`73,013 us`) as removable totals only `83,259 us`;
- final-tile assembly's whole inclusive bucket is `141,364 us`, but it contains
  52 mandatory fixed-final solves required by `INV-LANDSURFACEENERGY-110`.
  No source or timing evidence assigns `100,000 us` to removable authorization,
  operand-clone, sealing, or protocol bookkeeping;
- the V4 frozen-litter path does repeat final V3 projection validation and
  canonicalization, but its entire fixed execution-and-accept envelope is only
  `119,121 us` (`107,804 us` runtime). Clearing the floor would require removing
  more than 84% of an envelope that includes irreducible litter physics,
  ingress/WB14, exact aggregation, owner advancement, and installation;
- the potential evaluator resolves identical V3 litter vapor twice per call,
  but the removable second resolver is cheaper and less broadly applicable
  than the already measured full beta-one leaf-evaluation reuse, whose potential
  reduction was about `42 ms`. It therefore cannot credibly reach `100 ms`.

These are source-grounded pre-kill dispositions. None authorizes a cache,
cross-map reuse, solver-trajectory change, weakened validation, or production
edit. The separate physical-evidence assembly seam remains under profiler-only
attribution because its nonoverlapping inclusive upper bound is above the
materiality floor.

## Rejected physical-evidence assembly candidate

Ran 2026-09-04: exact CPU-0 attribution at Rust manifest
`b9f0e1018e72ac0aded97aa1d4004cba3e670d7ec856baa9e662f8584415155f`
and binary
`e54e71af09af60e1c7c993011bc794a9355ce8c11e6ac7987085ad0b9f73780e`
corrected the static call estimate: 20 direct trials plus 32 split children are
adaptive topology counters, while the authentic common physical-evidence path
executed 400 complete map scopes. All 800 later Stage-3 joins matched the first
same-map digest; mismatch and incomplete counts were zero.

The source-proven removable subtotal was only `8,121 us`: Stage-3 later seal
and projection `1,103 + 1,404 us`, repeated soil authority/operands
`84 + 4,857 us`, immediate terminal-receipt validation `197 us`, and surface
custody revalidation `474 us`. All 17 exclusive stages totaled `27,257 us`;
the conservatively required or mixed subtotal was `19,136 us`, and identity
bookkeeping was excluded. Exact science, `48/56/20/32/4` counts, and telemetry
completeness passed; measured RSS was `70,572 KiB`, above the package ceiling.

The candidate therefore fails the `0.10 s` materiality rule by more than an
order of magnitude and also fails diagnostic RSS. No validation-once contract
or production custody path was added, and every profiler-only Rust edit was
reverted. Raw selected evidence:
`artifacts/terminal-heavy-gates/carrier_physical_evidence_attribution_one_ofe_release.log`.

All profiler code was removed and residue, formatting, orchestrator, runner-
tests, and diff checks passed. The diagnostic did leave behavior-neutral
formatting changes in standalone `include!` fragments because direct `rustfmt`
was used and no per-file pre-edit snapshot existed. The current reconciled Rust
manifest is
`86e30dd6fdcad99b77c67641379c08af70f5faf68acf441797f3e006d1b461bb`;
it is not claimed as a byte-exact restoration of the prior `6e3f...` manifest.

## Rejected revision-61 feed-forward terminal carrier

The exact three-run keep/revert gate used Rust manifest
`650f67132aec95818228c1f3ed85db7310a7e66671dcb5f94cb18948ae257d41`
and release test binary
`e6b57efad9121915e95d45fa85323dfd29e9b5b8c8201f6356a2c6a10ea2df65`.
Provider-carrier times were `1_030_868`, `1_038_467`, and `1_032_416 us`;
run-wall times were `3_936_777`, `3_913_131`, and `3_895_367 us`. Both medians
passed their prospectively fixed ceilings. RSS was `69_768`, `59_504`, and
`70_484 KiB`; the first and third runs failed the per-run `65_536 KiB` limit.
Exact science, `48/56/20/32/4`, 200 one-call invocations, and the complete
invocation multiset were identical. The increment was nevertheless rejected
and reverted exactly as the conjunctive contract required. Raw log:
`artifacts/terminal-heavy-gates/revision61_feed_forward_release_3run.log`.

## Rejected revision-31 component-temperature dependency replay

The frozen same-source baseline at Rust manifest
`78d756be1fa11ed85ee92b7d19e6c04427b01b122efaf7804d1b55d60536bbbe`
and binary `9a91c82f...73f` recorded total/potential tuples
`4_926_758/354_838`, `4_903_570/353_374`, and `4_896_095/353_431 us`.
The prospectively bound candidate ceilings were therefore median total
`4_803_570 us`, median potential `253_431 us`, every-run RSS at most
`65_536 KiB`, exact science/count parity, and one authentic completed
`N=2,S=6` `58/14/16/28` audit in every run.

The approved candidate retained source digest
`039a312502a5e6ef442b1e81ac78b988141199f6283fedcc86518ba78ba61abc`
and used release binary
`f9386eec584664f9639da281c15796730240239cd43ad2f158f4fa6d27fbeeaf`.
Run 1 exited `101` before JSON because the real release aggregate contained no
authentic completed sweep matching that binding audit. Consequently there are
no candidate timing/RSS medians to adjudicate. This is a failed release
conjunct, not a timing pass or an inconclusive performance sample. Runs 2/3
were not run and the candidate was fully reverted. Raw log:
`artifacts/terminal-heavy-gates/component_dependency_replay_candidate_3run.log`.
