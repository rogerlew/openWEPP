# Collector and identity QA review B

Static + Ran: independent source review on 2026-09-06, initially four and then
five offline collector tests PASS, a small in-memory negative-control audit, and
the corrected full retained A trace plus eleven structural mutations executed. No build,
Rust test, comparator, simulation or comparative measurement was run by this
reviewer. Review A was not consulted before these findings were submitted.

## Findings

| ID / severity | Path at first inspected cut | Finding / disposition |
| --- | --- | --- |
| COL-B1 / P2, resolved statically and offline | `artifacts/reproduction/audit_stencils.py:89` | The first cut accepted 24 successful `Complete` probes without any actual `Evaluation`. The corrected audit enforces scope hierarchy/order, exact parentage, current iteration and stencil/probe joins, unique final map joins, outcome consistency and propagated first-error stopping. Complete and component-replay probes each own one shared `Evaluation`; identity anchors own none, matching R's source semantics. Independent physical/leaf savings remain separately required. Full A trace PASS and all eleven structural mutations REJECTED. |
| COL-B2 / P2, resolved statically and offline | `artifacts/reproduction/run_series.py:173` | Initial validity checked only five phase records per iteration and any process sample. The correction now requires exact phase/iteration sequence, monotonic same-PID records, memory fields, runner interval brackets, actual active samples and observed pinned affinity. Five tests PASS including duplicate/foreign-PID/clock/affinity/empty-active negative controls. |
| COL-B3 / P2, resolved statically | `artifacts/reproduction/run_series.py:257` | The separately supplied source manifest was initially only stored. The correction now joins its `source_identity` to admitted identity and its checkout to the admitted `/source_commit` expectation before series freeze. Binary/sidecar provenance remains separately checked. |

Paths above are relative to this package. These were evidence-validation
corrections, not engineering rejections. Initial findings were independently
formed; reviewer A's subsequently authored mutation controls were then read and
independently executed.

## Accepted design and prior correction closure

The manifest allowlist is narrow: one invocation timestamp; explicit run paths
and run-root path keys; and exact arm binary path/hash, sidecar path/hash and
source commit. Provenance exceptions must equal admitted arm values. Every
remaining manifest leaf and the named input/output/control/closure fields remain
in the exact JSON comparison. Sorted JSON with `allow_nan=False` preserves
binary64 signed-zero distinctions. Both arms must share the normalized scientific
identity while each arm preserves its own admitted mechanism counts. Identity
freezing expressly does not claim scientific gate completion.

Expected executable hashes, environment digest, source manifests and identities
are frozen once. Each process verifies the executable before launch and after
exit. Exclusive series-freeze and raw-log creation prevent silent replacement.
All raw process samples, phase records and logs are retained. `wait4` lifetime
HWM and CPU, runner wall/CPU and active sampled RSS remain distinct quantities.

Prior CB-1 is statically resolved: child uses Linux CLOCK_MONOTONIC and emits
both runner boundaries; the parent includes only status reads wholly within
that interval. Prior CB-2 is statically resolved: resolved-evaluator success is
now recorded after the boundary and energy guards at the completed substep tail.
Observer tests bind failure-tail semantics to those real source guards; this
reviewer did not run the Rust tests.

The stencil oracle independently reconstructs binary64 h, coordinate domains,
lawful signs, one-sided boundary stencils and expected classes from actual base
bits. It does not copy runtime eligibility output or assume centered stencils.
It explicitly limits graph eligibility claims to the separately reviewed complete
descriptor/direct-edge oracle. The corrected lifecycle joins now supplement the
mathematical enumeration with actual evaluator/probe execution evidence.

## Non-blocking QA follow-ups

- Rename `test_every_retained_scientific_leaf_is_exact` or exercise every leaf:
  it currently mutates two top-level fields and one manifest leaf. Include an
  explicit signed-zero regression so that important promise remains executable.
- Keep the optional extension linked to the identical original source, binaries,
  collector and environment and analyze all 24 pairs together. Distinct exclusive
  series names alone do not prove extension identity.
- The stencil validator uses Python `assert`; execute under normal assertion
  semantics and reject optimized Python if this becomes a reusable admission API.
  No `PASS` should be possible merely through `PYTHONOPTIMIZE`.

## First inspected collector identities

| Path | SHA-256 |
| --- | --- |
| `run_series.py` | `518438a14c85b9de2c2dc43bb101d0d868f26f1bd7b2e9b7d0193d19e9526835` |
| `admit_identity.py` | `b067719667f83868f0aee2189342b21284d23d6cf159311e96be9c1e45731fa1` |
| `audit_stencils.py` | `14e24d640ccc83259562103bb43d157cb1d24c7a563cfae8fb50e3cbc9edfc4b` |
| `test_collectors.py` | `70cc43dc082a9631900a8a5fd661662214e46a6914ec06a264e05099c73b16e8` |

## Corrected cut and execution

Ran: `.venv/bin/python` on `test_stencil_mutations.py` with the retained
`raw/A-trace-01/lse-0.json` and `--full-trace`, exit 0. Positive 1345-event
complete-map prefix PASS; all eleven mutations REJECTED after independent counter
recomputation. Full A trace PASS: 400 maps, 800 solves, 2400 iterations, 2000
sweeps, 78800 complete probes, 28000 identity probes and zero dropped events.
No runtime was re-executed to obtain these results.

| Corrected artifact | SHA-256 |
| --- | --- |
| `run_series.py` | `bd07963a0728481004ad2c08805dd5ef7fff847fc7c118587f38a799330bd2aa` |
| `admit_identity.py` | `b067719667f83868f0aee2189342b21284d23d6cf159311e96be9c1e45731fa1` |
| `audit_stencils.py` | `dcd09ca26e272af41a25bfb5bf90f237c38bfc51f4bd2e3607ee164fc461a973` |
| `test_collectors.py` | `6984fa1bd0210baa17102ad465abb3142b6e223409c103657fa61b74469e9491` |
| `test_stencil_mutations.py` | `c3f600afd6cdff6dcb2fdea3bb00d78571af3997add8c527ad011052edacd932` |
| retained A LSE trace | `5ff2ccfec4bb7c555c52551a9d9d792c286e2c0e1f969067633194ba85288d92` |

QA disposition: GO for corrected collector/protocol sufficiency. No open
collector blocker remains at this cut. Actual F/R identities, traces, independent
physical-work counts, correctness/science and exact terminal evidence remain
required before their comparative admission. No mechanism speedup, science,
measurement or package-completion pass is claimed by this collector review.

## F construction-telemetry qualified collector cut

Static source review and Ran lightweight offline QA; no build, simulation or
comparative timing. Instruction discovery returned root/work-package AGENTS.
Exact new cut:

| File | SHA-256 |
| --- | --- |
| `run_series.py` | `0f29393a6e0817173454619be609de1b8455f443aebc807493c9bf60446558e1` |
| `admit_identity.py` | `5484f4e318356cd0df1f5774c280224e4d92678a0c3302071f5836dfe06cbe61` |
| `test_collectors.py` | `e2690394fb4d95920f643fd825e94c89770430420ab5cee0cb278751e575510d` |

No blocker found. The source implements the separately prospectively reviewed
`F-day-frame-review-b.md` relation, not a volatile exception: exactly one named
manifest leaf becomes a tagged `D−2*N*P` comparison value. Rule keys/mechanism
are closed; all operands must be actual non-boolean uint64 integers, N in1/10/19,
P positive and product no greater than D. Python's integer multiplication does
not overflow; that upper-bound check also bounds the mathematical product and
nonnegative remainder to uint64. N=1 retains exact1205/400 or805/200 pairs.

The rule is constructed from actual complete/error-free/no-drop provider and
carrier rows, requires exact row shapes and balanced counts, and is frozen per
arm. Every later record must match the whole frozen rule before normalization,
so changing P and D together cannot move a sample to another admitted pin.
The normalization also binds the exact raw counter and exact JSON pointer.
All other manifest/scientific fields remain exact. Runtime sampling additionally
compares complete carrier and LSE audit snapshots against that admitted arm;
the new relation does not replace those mechanism/independence assertions.

Ran `.venv/bin/python .../reproduction/test_collectors.py`:8 tests PASS.
Independently recomputed both `A-identity-1-v2.json` and `F-identity-1-v2.json`
from the retained actual admission logs: identical comparison identity, raw
day-frame values still1205/805. Ran24 additional read-only controls:
invalid/malformed/bool/float/zero/overflow operands, missing/extra counter
fields, extra rule keys, relocation of the exception, migration to the other
primary pin, a second changed scientific counter, and N10/19 unequal-residual
poisons. Every invalid case rejected or retained an unequal identity as
appropriate. Synthetic scale arithmetic checks are NOT actual scale admission.

QA GO this exact collector cut for the reviewed conditional comparison.
Successful authenticated native fixture/cardinality, detailed actual group/
role/support/custody preservation, independent physical-call elimination,
full physical/owner/closure/error/output parity and per-scale admission remain
separate mandatory gates. No missing scientific evidence can be supplied by
this normalization or by matching its residual. Old raw manifests and earlier
collector identities remain retained.
