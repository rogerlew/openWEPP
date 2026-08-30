# Terminal verification A

Status: **FAIL — the amended one-day qualification passes, but terminal package
closure is not yet supported by exact-current gate evidence**

Evidence mode: `Static + Ran`.

Verification snapshot: shared dirty working tree anchored at
`2a9ca2d845bb4f128441ab01f79b341033a31c7d` on 2026-08-29. Concurrent work
continued during verification, so this report identifies retained evidence by
content hash and does not represent the dirty tree as a clean commit.

## Verdict

The retained v7 optimized real-runner evidence is a technical **PASS** for the
owner-amended exact-60 one-day objective. Its parent/support chronology, count
math, rejection accounting, conservation ledgers, receipt residuals, committed
qualification snapshot, downstream consumer, archive fold, and output
transaction all pass. The accepted widths also prove that 60 seconds is a
fallback floor rather than the ordinary stable step.

Terminal **package closure is FAIL** at this snapshot. The required
post-repair exact-current full-workspace campaign has not passed, the attempted
replacement campaign was interrupted before test execution, two package
integration tests changed after v7, and the required review/diff/line-count
artifacts are not reconciled. These are gate-legitimacy failures, not a
reclassification of the successful v7 physical evidence.

## Ran evidence

- Inspected and hashed
  `/tmp/adaptive_microstep_amendment/one-day-final-v7-opt.log`:
  `5406d0d8afcf74c0cd3d85858af39d57498cb0b103e6233053b349e463729b2b`.
  The target
  `cqr_stage3_one_day_qualification_with_telemetry` passed 1/1 in 357.55 s.
- Inspected and hashed the companion `.time` file:
  `cfa364b6870f19ba2759a5f573ebb468aaf677554d5732e9189df6f0af5608c8`.
  The compilation-inclusive optimized command exited 0 after 9:21.39 wall
  time, with 2,844.09 s user, 53.05 s system, and 5,894,016 KiB maximum RSS.
  This cold-build RSS is not accepted as a model-residency measurement.
- Independently parsed all 48 `STAGE3_PARENT_TELEMETRY` and 48
  `STAGE3_PARENT_PHASES` records rather than copying the package summary. The
  parent ordinals are exactly `0..47`; every parent is 1,800 s; supports are
  contiguous from 0 through `86_400_000_000_000 ns`; and both parent-width and
  accepted-width sums equal exactly 86,400 s.
- Independently reconstructed count identities:
  `497 accepted + 206 rejected = 703 direct trials`; the 497 accepted-width
  occurrences exactly equal the accepted count; `190 phase + 0 event + 0
  phase-and-event + 16 other = 206` rejected trials; and 975 summed owner joins
  equal the final 975 publication supports. The final publication event count
  is 61. The separate limiting-cause audit (`155` fixed-point nonconvergences
  and `16` comparison rejections) is not incorrectly substituted for the
  mutually exclusive rejection-accounting identity.
- Reconstructed accepted widths are `19x60 s`, `112x120 s`, `354x180 s`,
  `3x360 s`, `6x420 s`, `1x900 s`, and `2x1800 s`. Their count is 497 and
  their exact duration sum is 86,400 s. Thus 478/497 (96.18%) are strictly
  larger than the exact 60-second fallback floor.
- Reconstructed physical closure from the raw terminal audit: 1,578 ledgers;
  maximum absolute mass residual
  `1.77635683940025046e-15 kg m^-2 <= 1e-9`; maximum absolute energy residual
  `1.39698386192321777e-9 J m^-2 <= 1e-6`; receipt-reseal energy residual
  `9.98625182546675205e-10 J m^-2 <= 1e-9`; and receipt temperature residual
  `4.37694325228221714e-12 K <= 1e-8`. The receipt energy result is close to
  its bound but remains on the admitted side.
- Audited current changed/untracked Rust paths selected by the package's Stage
  3, snow, LSE, surface-liquid, publication, receipt, restart, and runner
  surfaces: 123 files inspected, zero at or above the 3,000-line hard bound,
  and 24 at or above the 2,000-line WARN threshold. The largest was 2,991
  lines. This proves absence of a hard line-count blocker in that scoped set;
  it does not replace the required exact owned-file manifest and WARN
  rationales.
- Inspected `/tmp/adaptive_microstep_amendment/full-workspace-final.log` and
  `.time`. The attempted exact-current full-workspace replacement was
  terminated by signal 2 during compilation after 2:58.46 and executed no
  tests. It is **not** positive closure evidence, irrespective of the timing
  wrapper's displayed exit-status field.

No additional cargo command was started by this verifier because the canonical
full-workspace run and other package gates already held the shared cargo/build
resources. Bounded independent work was limited to raw-log reconstruction,
hashing, source/contract inspection, line counts, and gate-ledger inspection.

## Static evidence

- Current `SC-SNOWENERGY-001@25` binds the exact
  `60_000_000_000 ns` floor, requires ordinary stable supports substantially
  larger than the floor, retains the 96-iteration fail-closed cap, and binds
  `TOL-SNOWENERGY-005` at `1e-9 J m^-2` and `1e-8 K` without changing the
  independent physical-ledger tolerances. Current companion identities are
  `SC-SNOWFREEZE-001@140`, `SC-LANDSURFACEENERGY-001@11`,
  `SC-COUPLEDTIME-001@9`, `SC-SURFACELIQUID-001@13`, and
  `SC-VEGETATION-001@30`.
- The current one-day test invokes the normal
  `DirectProductionExecutor` through
  `execute_hillslope_run_with_runtime_policy`, requires the run result,
  validates the committed qualification snapshot, independently reconciles
  rejection categories, checks the unchanged mass/energy and receipt bounds,
  requires a converged exact-60 support within `1..=96` iterations, and
  requires the output pass, loss, and manifest files. The v7 PASS therefore
  reaches a real production consumer and transaction rather than stopping at
  producer-only telemetry.
- Current runner source stages committed-day evidence, performs a durable
  content-digest-checked append, appends the archive manifest, acknowledges the
  exact archive record only after that append, finishes private storage, and
  publishes outputs and manifest through the rollback-capable output
  transaction. Output-transaction tests retain mid-publish, missing-manifest,
  rollback-cleanup, backup-retention, and successful evidence-promotion
  postures.
- The one-day runner test and the named canonical contracts predate the v7 log.
  However,
  `tests/integration/snow_stage3_adaptive_compositional_contract.rs` and
  `tests/integration/surface_liquid_hydrology_custody_authority_contract.rs`
  were modified at 00:46:59, after the v7 log at 00:44:30. V7 remains direct
  evidence for its production target, but it cannot by itself establish an
  exact-current package-wide test state.
- The two-day runner residency test proves bounded archive/count/size/RSS
  progression. The publication-rotation unit test constructs a synthetic
  already-rotated history. Static search did not find a real two-day
  uncompacted-control versus rotated-history equality oracle covering exact
  day evidence, ending owner, chronology, and materialized WB14 replay. The
  existing tests therefore do not prove that stronger equivalence claim.

## Gate-legitimacy disposition

The owner amendment legitimately pauses seasonal/archive-memory and generic
per-step optimization; those paused objectives are not treated here as
failures. The following non-paused closure requirements remain unsatisfied:

1. `package.md` declares Critical risk and requires terminal canonical clean
   full-workspace correctness. The gate ledger's only completed exact-current
   campaign is historical run
   `8ec6202e-fafa-454a-8fc9-f9f2e621d149`, which failed with 3,465 passed, 107
   failed, 10 timed out, and 48 skipped. The attempted replacement executed no
   tests.
2. `package.md` still has Checkpoint F unchecked. At verification time the
   review findings, terminal diff reconciliation, and owned-file manifest are
   queued; line-count governance has not supplied the 24 scoped WARN
   rationales; and final disposition explicitly remains in progress.
3. V7 is accurately retained as a successful one-day artifact, but later test
   changes and the absence of a completed exact-current campaign make the gate
   ledger's phrase “exact-head final row” stronger than the evidence supports.
4. The required real uncompacted-versus-rotated publication-history equality
   oracle is absent; synthetic bounded-state validation and a real one-day
   handoff do not substitute for it.

Final result: **FAIL package closure**. The owner-amended exact-60 canonical
one-day technical qualification is **PASS** and should remain retained as such.
