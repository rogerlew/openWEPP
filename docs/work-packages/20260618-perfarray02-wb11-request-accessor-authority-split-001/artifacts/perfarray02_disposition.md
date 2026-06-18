# PERFARRAY02 Disposition

Evidence: Static + Ran.

## Verdict

NO-GO for ADR-0023 ratification and broad array-authoritative migration from this pilot.

## Why

The package succeeded technically:

- array-capable request/accessor seam landed;
- real WB11 runoff pilot ran behind a flag;
- structural proofs passed for the scoped pilot;
- OFE5 and H2637 identity passed.

The package failed the load-bearing performance gate:

| Target | Required | Measured |
| --- | ---: | ---: |
| <=10x floor | `<=386 us/OFE-day` | `817.810 us/OFE-day` |
| 5x stretch | `<=193 us/OFE-day` | `817.810 us/OFE-day` |

Boundary seed/materialize was separately measured at `1685.023 us/OFE-day` and is not
included in the floor verdict.

## ADR-0023 Input

Do not ratify ADR-0023 as proposed from PERFARRAY02 evidence. The request/read seam alone
does not reach the required budget, and Stage-C removal of boundary conversion would still
leave the measured array-native segment at about `21.16x` legacy no-UI.

## Recommendation

Stop broad migration. If continuing, scope a new decision around kernel output/writeback
shape: the remaining plausible lever is avoiding logical kernel payload construction and
logical-name resolution inside the piloted phase itself. That is not PERFARRAY02 scope.

## Post-review closure (2026-06-18, operator-approved — option A)

The perf arc is **closed**. Per operator decision, **all PERFARRAY02 pilot code was discarded**
(the invasive flag-gated scheduler/state_access/runoff plumbing + the `perfarray02_timing`
module) **and the PERFARRAY01 Stage A `ArrayHotState` shell was reverted** — production now
carries **zero array-authoritative code**; the committed record is docs-only. `cargo check
--workspace` clean and `openwepp-kernel-contract` tests pass (23) after removal.

**ADR-0023 is not ratified and the array-authoritative redesign is abandoned.** The perf
program concludes at the **PERFIDX04 / PERFIDX06 endpoint of 73.12×** (a real, bit-identical
~31% wall-clock reduction from the 978.55 s PERFHO01 baseline). The measured reason ≤10×/5× is
unreachable: removing the symbol-keyed machinery does not cheapen the **kernel's own per-OFE-day
computation + its still-logical writeback output** (array-native runoff = 817.8 µs/OFE-day =
21.16× legacy, over the 386 µs ≤10× budget). The only untried lever — array-native kernel
*output* — is more invasive and unproven; not pursued. Recorded to agent memory so the
read-side / id-table / seam arc is not re-run.
