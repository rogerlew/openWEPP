# Bounded attribution and recomputation result

Ran: final Q series has two warmups per posture and six balanced fresh-process
pairs on CPU0. All 16 runs passed protected input/output/control/closure/work
identity and every on profile reconciled with signed accounting difference 0 ns.

Median runner time was 5.033 s off and 5.036 s on. Median paired compact-observer
perturbation was -0.034% wall and -0.098% process CPU. Raw on-arm exclusive time
is reported without overhead subtraction.

## Refined exclusive partition

| Region | Median seconds | Runner share | Meaning |
| --- | ---: | ---: | --- |
| Stage3 LSE parent-exclusive | 0.813 | 16.14% | Still-unresolved work inside the day-preparation scope after every named child below preempts it. |
| Carrier physical evidence | 0.720 | 14.30% | Named mixed physical/evidence phase; not wholly reusable. |
| Carrier completion/construction | 0.630 | 12.51% | Four named completion phases, excluding children. |
| Imported frozen runtime | 0.467 | 9.26% | Named runtime phase, excluding its measured children. |
| Outside every named scope | 0.375 | 7.44% | Honest outside-scope remainder, not called physics. |
| Snow-free parent remainder | 0.329 | 6.52% | The one deeper split of the former Stage3 parent region; excludes imported children. |
| Other Jacobian probes | 0.316 | 6.29% | Retained P solver child. |
| Carrier physical completion | 0.232 | 4.62% | Named carrier phase. |
| Required integrity/validation | 0.212 | 4.20% | Imported validation, reuse validation/reseal, and frozen acceptance only. |
| Imported accepted candidate | 0.178 | 3.54% | Named candidate acceptance/construction. |

Remaining individual regions are each below 2.62%; the full table and ranges are
in `raw/analysis-final-cut3.json`. The carrier-related exclusive union is 34.70%, but
it mixes required construction, physical work, evidence, custody, and known F
duplication and is not a removable-cost claim.

This satisfies the bounded attribution endpoint: the old 49.24% label is split;
outside-named time is 7.44%, and the remaining 16.14% parent-exclusive region is
ranked against multiple larger source-backed children. Complete semantic
attribution is intentionally not attempted.

## Recomputation result

The separate detailed run completed with 400 Provider and 400 Carrier calls, 200
Evaluator calls, 72 Outer calls, no errors, and no dropped records. Every
Evaluator contains two Provider calls with distinct recorded requests but the
same recorded output. This is the already-established F feed-forward pattern,
not a new single-pass discovery. F previously reduced Provider/Carrier calls
400->200 and measured the bounded one-OFE gain; this package does not rename or
reimplement it.

Across Provider records, 400 executions have 204 distinct recorded inputs. There
are 132 repeated-input groups covering 328 executions, nominally 196 repeats.
However, eight repeated-input groups produce multiple outputs. Therefore the
recorded request key omits effective dynamic state or authority needed to make a
reuse decision. The remaining 124 retrospectively single-output groups cross
different Evaluator and Outer invocations, including repeated DiscoveryProbe
roles interleaved with ExactEndpoint work. Result equality after execution does
not authorize collapsing those independently required evaluations.

The evidence therefore establishes neither a complete same-effective-input key
nor a distinct interchangeable recomputation opportunity. No removable cost can
be assigned to those 196 nominal repeats. The only proven material duplicate is
F, already classified as an existing mechanism. The conditional >=5% gate for a
new typed single-pass prototype does not pass, so no prototype or cache was
implemented.

## Decision

The characterization supports `NO_NEW_PROTOTYPE`, but the package finishes as
executed HOLD because its frozen warnings-denied scoped Clippy gate is not green.
Production remains HOLD.
The actual next candidates are the 16.14% Stage3 parent-exclusive region and
14.30% carrier physical-evidence phase, but each requires source-specific
semantic decomposition before implementation. The 4.20% required
integrity/validation category is a named subset of runtime validation, reseal,
and frozen-acceptance obligations. Existing compact mechanism-audit counters are
experiment-only and run in both timed arms, so their overhead remains unmeasured
within honest parent/outside remainders; detailed hashing is confined to the
separate untimed trace. The off/on perturbation isolates only the added compact
timing observer. This is not a total audit-cost estimate. Memory remains
observational.

Limitations: one audit-capable one-OFE test executable, one host/day, wall-time
instrumentation, no production CLI or scale qualification, and no claim that the
largest label is removable.
