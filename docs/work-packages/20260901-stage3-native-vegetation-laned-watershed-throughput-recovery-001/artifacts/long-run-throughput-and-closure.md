# Long-run throughput and closure

Status: `PRIOR-SOURCE LONG-RUN EVIDENCE RETAINED — CURRENT QUALIFICATION NOT COMPLETE`

Evidence mode: `Ran + bounded feasibility disposition`

## Completed real-runner evidence

On the pre-late-optimization tested source, this release correctness workload
passed:

`env RUST_MIN_STACK=67108864 nix develop -c cargo test --release -p openwepp-runner --lib hillslope::tests::accepted_stage3_real_runner_routes_lane_d_and_publishes_summary -- --exact --nocapture --test-threads=1`

Historical result: `1/1 PASS`, test wall `320.13 s`. The uninterrupted test runs baseline
and factor-1000 area sensitivity for 1, 9, 10, and 19 OFEs, then a positive
two-OFE authored production-seed case. It validates topology order, one
downstream vegetation/BGC binding, WAT5 operands, Lane-D closure, HBP hourly
volume/peak, outlet-only PASS, routed-shape sensitivity, and rollback poisons.

The latest bounded post-revert one-OFE release probe completed in `4,920,992 us` kernel wall with
48 physical supports, 56 accepted publication supports, 4 accepted
microsteps, 20 direct trials, 32 split-child trials, and zero fixed-point
evaluations. Exact ledgers were source `0.8488061229561478 m3`, outlet
`0.8471105124736579 m3`, end storage `0.0016956104824910018 m3`, clamp `0`.

## Interim long-run disposition

The `4,920,992 us` sample is a full mixed-regime one-OFE day. It is not the
pure snow-free/native micro surface governed by the `250 us/OFE-day` budget,
and the complete-day `5 ms` CPU / `5.5 ms` wall ceilings apply to the 10-OFE
surface. It is therefore retained only as a diagnostic point, not adjudicated
against either mismatched limit. A same-rate feasibility projection is about
1,802 s for a 365-day one-OFE year and 180,297 s for a 100-year one-OFE run,
before expanding to 10 OFEs. These are projections, not benchmark results.

The required 10-OFE year and century were `NOT RUN`: the prior-source exact
10-OFE matrix median of `12.440 s` CPU / `12.4533514695 s` wall validly fails
the complete-day `5 ms` CPU / `5.5 ms` wall budgets by about `2,488x` /
`2,264x`. No
incomplete, narrowed, or extrapolated workload is presented as qualification.
The 5,000-hillslope projection and worker-scaling acceptance remain unqualified.

Disposition: these measurements do not satisfy Phase 4. A slow or partially
improved benchmark alone was not treated as a HOLD boundary; the subsequent
in-envelope recovery campaign continued through the retained/rejected
increments recorded below. Terminal verification rejected the later proposed
owner-choice HOLD because broader canonical architecture work remains in
scope. Execution continues; this intermediate measurement is not a boundary.

## Isolated 1/10/19-OFE release matrix

Ran the exact single-CPU release matrix through all five warm-ups and 30
measured batches per surface. Log:
`artifacts/terminal-heavy-gates/release_qualification_matrix_1_10_19_ofe_rerun3.log`,
SHA-256 `bc7ba1fccaed2c3b210e437fd77aa70bf3922231969c0570c19f4a7bdfc181c3`.
The tested working-tree digest was
`c5e9d102c502a550a76babb67ddab9c0e7378e48dc93a96de8d1567ff51e2b28`;
the exact release test binary digest was
`edb119071067a67c4173b71d4b92d546b911bbfec54129bcf0e8533b7150a504`.

All measured batches were comparable with byte-identical scientific result
hashes per surface and exact closure/counter identity. Results:

| OFEs | wall median/run | wall median/OFE-day | CPU median/run | hard peak RSS | result SHA-256 |
| ---: | ---: | ---: | ---: | ---: | --- |
| 1 | 5.333934174 s | 5.333934174 s | 5.330 s | 87,956 KiB | `ef29ad59b308a77552cb4577abf43c47f714cd89fa1e5c7f9b53c5f6b2264f50` |
| 10 | 12.4533514695 s | 1.24533514695 s | 12.440 s | 325,728 KiB | `8b14a92d11b6f6f5870c38f31a4962764cf66007427463d2fb30f58eed6b3433` |
| 19 | 22.8885138375 s | 1.20465862303 s | 22.860 s | 648,992 KiB | `4ed0064b2f2c3b85c79e3e063e90b0ec9d3c31b683d682bbf36e0f593633f99a` |

Scaling passed: `T10/T1=2.33474037423 <= 12` and
`T19/T10=1.83794008332 <= 2.2`. Time budgets failed at every required
surface, the Stage-3/LSE envelope fraction was `0.86454`, `0.75713`, and
`0.76423` versus `<=0.40`, 10/19-OFE hard RSS failed, and the aggregate
RSS-return check failed. Native vegetation/ET, Lane D, remaining-runner
fractions, ordered per-OFE Lane-D calls, scientific closure, and deterministic
identity passed.

The emitted final record also reported missing canonical physical-map evidence.
Source inspection proved the fixture's four adaptive receipts are terminal-
regime supports: its canonical ordinary-covered width histogram is empty, so
requiring four ordinary map records was a harness classification defect. The
qualification code and prospective package text now bind ordinary-map record
cardinality to the canonical-covered histogram total and report a terminal-only
surface as not applicable without suppressing terminal direct/split counts or
any time/memory failure. The retained matrix remains valid measured performance
evidence and a final `FAIL`; it is not relabeled as a pass.

## Latest one-OFE exact-reuse checkpoint

After the completed matrix, contract-first solver work retained exact
same-evaluation leaf maximum-demand reuse. The later four-column hydraulic
probe experiment failed exact-head target-plus-total retention and was removed.
The latest post-revert CPU-0 release result is `4,920,992 us`, with potential
`352,898 us` and carrier physical evidence `1,018,388 us`. Source, outlet,
storage, clamp, and all 48/56/20/32/4 workload counts remain exactly unchanged.
This improves the diagnostic one-OFE point but does not supersede or relabel
the completed 1/10/19 matrix; that matrix remains the required measured
performance/RSS `FAIL` until rerun by a future qualifying increment.
