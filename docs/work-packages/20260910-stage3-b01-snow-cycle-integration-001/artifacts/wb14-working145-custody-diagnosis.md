# WB14 working145 custody diagnosis — blocked admission record

## Verdict

**INCOMPLETE/BLOCKED — BUDGET UNESTABLISHED.** This is not a runtime correction,
restart result, conservation acceptance, review approval, or production-readiness
claim.

## Checkpoint and identity

- Requested checkpoint: establish the exact working145 extended-cycle WB14
  continuation failure and, only if evidence permits, prepare a bounded repair
  specification.
- Immutable package/governance revision named by the adopted authorization:
  `e9e7511c35ce50ac03b85f8ba70c31d0342513c4`.
- Recorded experimental source: `working145`, full-source SHA-256
  `b6fb949e967af911bd3d508c02e023bf0a349b30e3c0697631276b351b50e88f`.
- Recorded frozen runner SHA-256:
  `a175fd68b9ef47b25772cf1f6af7aae2e3406e70365a3a39460eed6a7722b3ff`.
- Recorded raw archive: `raw-stop-point145.tar.gz`, expected size `953251918`
  bytes and SHA-256
  `5365a6e47b67d23663f668cbe5c1d6c1b063cf599abcd8be0c37115135c38985`.
- Recorded protected cold checkpoint SHA-256:
  `362cd4e4cbfcfdaa5e2cc409291feb170c541f6cc7233d8981a3d8b5a279acad`.

## Ledger admission finding

The adopted authorization carries forward—not resets—the default ceiling of two
unsuccessful correction cycles or 60 active minutes. Its prerequisite is a
recoverable record of the original checkpoint boundary, prior unsuccessful
correction cycles, prior active minutes, excluded waits, and remaining allowance.

**Static:** `package.md` records the 2026-09-11 owner stop point, working145's
warm/cold/extended authentic attempts, and its 1595.325263679 s extended-cycle
failure at the WB14 guard. It also says the new guard/caller operands have not
been analyzed. It does not record a custody-diagnosis checkpoint start, its
previous correction-cycle count, active minutes, excluded wait intervals, or a
remaining allowance. The working145 label cannot be used to infer those values.

**Ran (offline only):** repository instruction discovery and text extraction
searched the package record for `budget`, `allowance`, `active minutes`,
`unsuccessful correction`, `cycle count`, `correction145`, and `stop-point145`.
Exit was successful; the only relevant results were physical/model numerical
budgets and stop-point references, not a continuation-budget ledger. No model,
build, test binary, archive extraction, source reconstruction, runner, or
checkpoint operation was executed.

Consequently the remaining allowance cannot be calculated without inventing
history. The authorization requires return at this point; therefore no
working145 file/line, guard predicate, caller operands, archive members, or
canonical WB14 obligation was inspected for a causal conclusion.

## Evidence limits and next owner decision

The package's current summary is only a lead: it reports that working145 captured
exact guard/caller operands at an unchanged guard, while the candidate at
385920 s was provisional and no day 4 was accepted. It does **not** establish an
inactive-prefix/parent-custody cause. Competing explanations (cursor, owner/parent
join, candidate-versus-committed state, and source/driver composition) remain
unexamined.

Owner decision needed: provide or explicitly set the carried-forward custody
diagnosis ledger (checkpoint boundary, failed correction cycles, active minutes,
excluded waits, and remaining allowance), or explicitly revise the allowance.
Only then may a new bounded diagnostic continuation bind the retained source and
archive and reconstruct the WB14 boundary. No repair or regression is proposed
from the uninspected operands.

## Revised authorization v2 — source-bound diagnosis

The owner-adopted v2 instruction grants a one-time supplementary 60 active
minutes and two further correction cycles for this same diagnostic checkpoint.
The historical budget remains **UNOBSERVED** and the above blocked admission at
commit `1fa4ea253134bf3f9dbac36a719c35fd301bc0ae` remains historical fact.
Supplemental ledger at this disposition: approximately 30 active minutes,
zero correction cycles, no recorded waiting interval, and approximately 30
minutes remaining. No authentic execution was allowed or performed.

### Identity and evidence bound

**Static / Ran (offline only):** the retained `correction145-source.json`
records working145's base as `B01-original final016 retained composition`, policy
14, and the recorded full-source SHA-256
`b6fb949e967af911bd3d508c02e023bf0a349b30e3c0697631276b351b50e88f`.
Its relevant manifest hashes match the inspected retained files:

| File | Manifest and inspected SHA-256 |
| --- | --- |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_ingress_preflight.rs` | `b0d362563f6fda1ae49cac2be9b011bbb3d83f497f399e2b6c29524f98af1ae3` |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_ingress.rs` | `1826924b199e8a1ff7a42776ac99713b0b1bb01ab8207a1ea508cc1539096c29` |

The retained `evidence-review145-archive-results.json` also reports a passing
900-file replay edge from working144 to working145. This executor did not rerun
the full-source identity algorithm, so the recorded full-source hash is bound by
that retained evidence rather than independently recomputed here.

The local `correction145-runner` SHA-256 matches its source manifest and the
retained tail7 `started.json`: `a175fd68b9ef47b25772cf1f6af7aae2e3406e70365a3a39460eed6a7722b3ff`.
The tail7 run metadata binds the authentic case `gradual_warm_tail7`, one OFE,
physical collection, collector SHA-256
`824d60b7667c118cd6c011fad5312ddaec85a4ebf00ce5646a48e79b3d20b2aa`, and
the runner command. The five inspected tail7 records (`run.json`, `started.json`,
`receipt.json`, `stdout.log`, and `stderr.log`) match their raw-index hashes.
The 3,699,597,023-byte observation file was not whole-file rehashed; its recorded
index hash is `5e98534c908f1c0531a0c73164488d12f961af167ee7b4d37a85223360d1fc54`.
Offline memory-mapped extraction found the two retained diagnostic records at
byte offsets 3,699,563,855 and 3,699,575,700, respectively. No archive, runner,
build, test binary, model, or checkpoint was executed.

### Exact failure and custody reconstruction

**Inspected execution evidence:** the authentic tail7 `run.json` reports
`SURFACELIQUID-E-008 ... WB14 day or interval continuation mismatch`; execution
is `FAIL`. Its paired records
`surface_liquid_wb14_cadence_failure` and
`surface_liquid_wb14_cadence_caller_failure` contain the same operands:

| Operand | Observed value |
| --- | --- |
| requested input | day 4, interval 22, transaction 255, 60 s |
| parent support | `[385200000000000, 387000000000000)` ns |
| first physical child support | starts at `385920000000000` ns |
| parent working `accepted_until_ns` | `385920000000000` ns |
| parent-child/final flags | `true` / not final |
| beginning and candidate cursor | OFE `ofe-1`: day 0, next interval 0, cumulative supply/infiltration both `0.0` |
| parent-local WB14 working cursor | same zero physical cumulatives; `next_child_ordinal=0`; no physical receipts |
| inactive-prefix proof | parent support above; prefix end `385920000000000` ns; authenticated parent/owner/receipt/proof digests present |
| mutation | `state_mutated=false`; beginning and working state SHA-256 both `0368e37bfa660864f3ef1f5f39351d0adde68fa525287d517b1fa45f354f4951` |

At [surface_liquid_ingress_preflight.rs:100](/tmp/openwepp-b01-cycle-20260910/working145/crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_ingress_preflight.rs:100),
`validate_cadence` classifies that byte-identical state as `initial`; it therefore
expects `(day=0, interval=0)` ([lines 119–132](/tmp/openwepp-b01-cycle-20260910/working145/crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_ingress_preflight.rs:119))
and rejects the requested `(4,22)`. Its limited parent-local exception requires
the *persistent* continuation itself already to equal the requested day and
interval+1, which the inactive-prefix posture intentionally does not satisfy.

The reached caller
[surface_liquid_ingress.rs:1065](/tmp/openwepp-b01-cycle-20260910/working145/crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_ingress.rs:1065)
passes only `resource.beginning_state()` to that guard before it validates the
supplied `parent_working_state`. The latter is present, matches input day/interval
and support, and contains the inactive-prefix proof. The later parent validation
([lines 1116–1147](/tmp/openwepp-b01-cycle-20260910/working145/crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_ingress.rs:1116))
is unreachable. Thus this is neither a stale accepted physical cursor nor a
wrong owner/parent join: zero cursor/ordinal/receipts are required by the first
physical child after the represented-snow prefix. It is a guard/caller ordering
and custody-context omission.

**Static authority:** `SC-SURFACELIQUID-001` INV-012 requires child-only
parent-local advance and one final persistent advance; INV-013 requires exact
parent/child/receipt identity; INV-035 expressly authorizes an authenticated
inactive prefix to advance chronology *without* WB14 physics, changing beginning
cumulatives, or creating a receipt, with the first physical child ordinal zero.
`SC-SNOWENERGY-001` INV-087 states the same prefix/first-snow-free rule.
`SC-COUPLEDTIME-001` §5 requires accepted-only atomic owner installation and
byte-identical inactive carries. These obligations rule out cursor fabrication,
replaying the prefix, a guard relaxation based on absence alone, or any state
reset.

### Minimal repair specification — do not implement in this checkpoint

**DIAGNOSIS COMPLETE — REPAIR PROPOSED.** At the existing ingress entrypoint,
make cadence validation select a parent-local expected cursor only when the
already supplied `DirectWb14ParentWorkingState` has passed its existing exact
schema, input day/interval, support, configuration/model/parameter/lane,
candidate/beginning, `accepted_until_ns`, and inactive-prefix-proof joins. In
that one posture, derive admission from the authenticated parent and prefix end,
while retaining the persistent zero cumulatives and physical ordinal zero. All
ordinary and already-advanced paths continue to use the persistent continuation.
Do not mutate the prefix, synthesize a receipt, advance the persistent cursor, or
weaken `validate_cadence` for an unproved/missing parent state. The likely narrow
locations are `surface_liquid_ingress.rs` around lines 1065 and 1116, plus the
cadence helper's explicit input/context contract in
`surface_liquid_ingress_preflight.rs` around lines 100–162.

Proposed later failing-then-passing regressions:

1. Reproduce the exact captured parent day 4/interval 22, support, zero
   cursor/ordinal/cumulatives and valid inactive-prefix proof; it admits the
   60-second first physical child while leaving the persistent beginning bytes
   unchanged and creating no prefix receipt.
2. Remove, replace, reorder, or change the proof prefix end; it rejects before
   ingress/WB14/state mutation.
3. Poison parent day, interval, support, owner/configuration/model/lane or the
   `accepted_until_ns` child-start join; it rejects before mutation.
4. Supply zero cursor/ordinal without the exact proof, or attempt a duplicate or
   noncontiguous child; it remains a cadence rejection, not an inferred inactive
   prefix.
5. Follow an accepted final child and verify exactly one persistent day cursor
   advance; a second finalization/replayed child rejects.

No independent transfer/conservation closure is claimed: this failure occurs
before local ingress/WB14 arithmetic, and the inspected parcels only establish
the candidate input rather than a completed physical transfer.
