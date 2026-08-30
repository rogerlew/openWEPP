# Terminal verification B

Status: **FAIL — package closure is not yet verified**

Evidence mode: `Static + Ran`.

Verification snapshot: working-tree source at commit
`2a9ca2d845bb4f128441ab01f79b341033a31c7d` on 2026-08-29. The shared tree
was dirty and concurrently active; the commands below exercised the source
visible at verification time.

## Verdict

The owner-amended exact-60 production path, its real downstream consumer, the
stable-support behavior, and the focused receipt/event/rollback guards pass.
Terminal **package closure fails**, however, because no post-repair
exact-current full-workspace correctness campaign is recorded and the required
real uncompacted-versus-rotated publication-history equivalence test is absent.
The focused results below must not be promoted over those missing closure
gates.

## Static evidence

- Real consumer/publication: `05_runner_execution_and_outputs.rs` invokes the
  atomic Stage3 V11 publication stream, streams rows to the real sink, stages a
  committed-day archive, durably appends the exact canonical record, verifies
  its content digest, appends the manifest entry, and only then acknowledges
  the exact record digest. The acknowledgement is therefore downstream of
  durable consumption rather than producer-only evidence.
- Receipt-history retention: archive staging leaves the full committed receipt
  and publication day resident. Acknowledgement clones the real consumer,
  validates the exact sealed capability, rebases the latest WB14 replay to a
  materialized checkpoint, clears resident support/event vectors, verifies
  zero residency and cumulative support/event counts, and commits the clone
  only on success. Omission, substitution, reordering, stale acknowledgement,
  mixed-day, partial-day, and pending-event postures fail closed.
- Rollback/event exactness: publication mutation is clone-then-commit;
  attachment archive acknowledgement similarly computes the next prefix and
  consumer before replacement. Receipt-chain validation binds identity,
  support, owner, phase, event, and order, while terminal event ordinals reject
  duplicate/reordered chronology without installing state.
- Old-path exclusion: production constructs the sole typed Stage3 V11 model
  triplet and calls both retired-snow-selector and retired-Lane-D rejection
  guards. The source guard confirms that the whole-run batch-retention path is
  absent from Stage3 V11 publication. No `unsafe` block was found in the
  inspected Stage3 V11, accepted-publication, WB14-retention, or runner
  publication surfaces.
- Stable-support performance: the final v7 log reports 497 accepts, 206
  rejects, 975 publication supports, 61 events, and accepted widths
  `19x60`, `112x120`, `354x180`, `3x360`, `6x420`, `1x900`, and `2x1800`
  seconds. Thus 478/497 accepted supports exceed the exact 60-second fallback
  floor. The optimized test body was 357.55 s versus the retained 485.858 s
  baseline (26.41% lower).
- Conservation/receipt bounds in final v7 remained unchanged and passed:
  maximum mass residual `1.77635683940025046e-15 kg m^-2` against `1e-9`,
  energy residual `1.39698386192321777e-9 J m^-2` against `1e-6`, receipt
  reseal energy `9.98625182546675205e-10 J m^-2` against `1e-9`, and
  temperature `4.37694325228221714e-12 K` against `1e-8`.

## Ran evidence

- Inspected `/tmp/adaptive_microstep_amendment/one-day-final-v7-opt.log` and
  `.time`: **PASS**, 1/1 real runner qualification, all 48 parents, body
  357.55 s, cold optimized-build wall 561.39 s, exit 0. The log reaches the
  committed snapshot, downstream publication consumer, archive fold, and
  output transaction assertions. The 5,894,016 KiB peak includes the cold
  optimized compilation and is not a model-residency measurement.
- Inspected the retained receipt chronology. The earlier
  `exact-receipt-five-parent-v7.log` is a failed diagnostic and is not positive
  evidence. Its later corrected replacement,
  `contracted-causal-reseal-five-parent.log`, is **PASS**, 1/1 in 105.54 s.
- `nix develop --command cargo test -p openwepp-hillslope-orchestrator accepted_publication --lib -- --nocapture`:
  **PASS**, 7/7. This includes cached/full chronology, pre-support and genesis
  event poisons, synthetic bounded rotation, WB14 materialization/delta poison,
  and publication capture budget coverage.
- Exact-floor stable support: **PASS**, 1/1 in 0.59 s; direct/split/accepted/
  rejected counts are `1/0/1/0`.
- Explicit ignored 1,800-second stable-support qualification: **PASS**, 1/1 in
  1.49 s; direct/split/accepted/rejected counts are `1/2/1/0`.
- Receipt-chain identity/support/owner/phase/event/order poison filter:
  **PASS**, 1/1.
- Complete event-index terminal/reappearance chronology poison filter:
  **PASS**, 1/1.
- Runner durable archive/no-batch-retention source guard: **PASS**, 1/1.
- Runner sole typed Stage3 V11 authority/no-retired-selector source guard:
  **PASS**, 1/1.

## Closure blockers

1. The gate ledger's only exact-current full-workspace critical regression is
   run `8ec6202e-fafa-454a-8fc9-f9f2e621d149`: **FAIL**, 3,465 passed, 107
   failed, 10 timed out, and 48 skipped. It is explicitly marked historical
   pre-repair evidence and unable to close the package. No post-repair
   exact-head full-workspace PASS is recorded.
2. Static search finds no real two-day uncompacted-versus-rotated comparison.
   The rotation module's bounded test constructs a synthetic already-rotated
   history and checks metadata/checkpoint materialization; it does not execute
   two sequential accepted publication days and prove exact per-day evidence,
   ending owner, publication chronology, and materialized WB14 replay equality
   against an uncompacted control. The one-day runner test proves the archive
   handoff and bounded result, but not that required equivalence oracle.
3. At this snapshot, `disposition.md` remains “closure review in progress” and
   the four review artifacts plus terminal verification A remain queued. Those
   package-required independent gates cannot be inferred from this verifier's
   focused passes.

The historical compacted qualification failures and pre-fix v7 failures are
properly retained rather than hidden. They are superseded for the final
one-day count/performance claim, but they do not supply the two missing closure
proofs above.
