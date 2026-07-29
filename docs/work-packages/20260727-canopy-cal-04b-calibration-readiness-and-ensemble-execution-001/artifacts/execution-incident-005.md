# Execution Incident 005

Status: `CORRECTION DESIGNED / IMPLEMENTATION AND REVIEW REQUIRED`

Evidence class: `Static`

The retained 2026-07-27 attempt passed preparation, both builds, the twelve-case
native production-consumer proof, and synthetic trace production. The first
synthetic reconstruction then failed before Hubbard population:

```text
Error: "hidden candidate lacks one crossing per year"
```

This is the required identifiability test for recovering a known hidden
threshold vector. The hidden case did not produce the required single crossing
in every synthetic year, so the recovery claim cannot be evaluated. This is a
science-design defect in the synthetic case or threshold parameterization, not
a tooling or execution-authority hold.

The surviving local evidence is:

| Object | SHA-256 | Retained observation |
| --- | --- | --- |
| `/home/workdir/cal04b-objects/synthetic_reconstruct.log` | `758f5304e1f9065c7576201fb89d9bebe9e9c146a35ccd56ee9c0cef3f7a3369` | exact primary error |
| `/home/workdir/cal04b-objects/synthetic_gsi.log` | `a43ea6916411b5da795e25d00ec75d41f75dbc1e87de70d4cf05bdb601592c9d` | producer PASS followed by reconstructor failure |

The generic `.err` file at the same root was subsequently overwritten by a
later failed retry and is not claimed as evidence for this incident. The two
logs above were read statically; Order 2 ran no CAL command.

Disposition: retain Harvard sealed, execute no candidate population, and repair
the synthetic-recovery design only under explicit CAL-04B science authority.
The next CAL attempt must use a fresh execution root and first prove the hidden
case produces the required crossings and is recovered by both reconstructors.

## Resumed Diagnosis — 2026-07-28

Evidence class: `Ran + Static`.

A read-only reconstruction of the retained `SYN04B02` binary trace proved that
the failure precedes either reconstructor's objective arithmetic:

| Candidate | Maximum instantaneous GSI | Maximum 21-day GSI | Eligible upward crossings |
| --- | ---: | ---: | ---: |
| `GSI-0001` | 0 | 0 | 0 |
| `GSI-5557` | 0.01569917298208302 | 0.009482411200617866 | 0 |
| `GSI-9261` | 0 | 0 | 0 |

The original VPD forcing
`600+350*sin(2*pi*(ordinal_day-100)/365)` places high VPD in the same spring/
summer interval that temperature and photoperiod become permissive. For
`GSI-5557`, the three indicator factors therefore never overlap enough for the
diagnostic GSI to approach 0.5. This is a synthetic-fixture phase error, not a
production-kernel or threshold-domain defect.

The read also found that `synthetic_trace.rs` constructs one `GsiState` outside
the year loop and both reconstructors retain `previous` across year
boundaries. That contradicts CAL-04B's binding per-year native cold-start rule
and its yday 60–180 crossing window, even though it did not cause the retained
zero-crossing failure.

## Prospective Correction

This is a transparent post-failure fixture redesign, frozen before corrected
native execution. It is not result-blind. No further forcing, candidate, or
acceptance tuning is permitted after this amendment without a new recorded
science-design disposition.

The package-local synthetic design is amended before code:

1. Reverse only the synthetic VPD seasonal phase to
   `600-350*sin(2*pi*(ordinal_day-100)/365)`. The execution-only range remains
   exactly 250–950 Pa and is labeled `ASSUMED_FOR_EXECUTION`; it is not an
   observation, authoritative phenology forcing, calibration input, prior, or
   physiological bound.
2. Construct a separate empty native `GsiState` for every candidate-year.
3. Reset crossing memory at each year boundary and count upward crossings only
   on yday 60–180. Retain every crossing count in both reconstructions.
4. Require the hidden truth to have exactly one eligible crossing in every
   year; missing or multiple hidden crossings fail closed. Competitors use the
   first eligible crossing for scoring while retaining the full count.
5. Retain `GSI-0001` and `GSI-9261` as missing-crossing boundaries. Add
   `GSI-0064`, selected by the frozen rule "lexicographically first non-hidden
   grid candidate with exactly one eligible crossing per year outside the
   hidden ±2-day interval." It crosses at yday 159 and must have a finite
   objective greater than zero, so recovery exercises interval distance and
   equal-year aggregation rather than passing only through boundary failures.
6. Bump the trace schema, bind the complete design SHA-256 and semantic fields
   into the producer identity and both reconstruction receipts, and reject old
   traces.
7. Add a reset-sensitive negative test that distinguishes per-year cold starts
   from cross-year carry independently of this periodic fixture's boundary
   values.

A post-failure prospective equation replay predicts `GSI-5557` maximum 21-day GSI
`0.9261127047471505` and one eligible upward crossing at yday 146. Both boundary
competitors remain identically zero, while `GSI-0064` has one eligible crossing
at yday 159 and maximum 21-day GSI `0.6457410653452017`. This replay is design
evidence only; the corrected native trace and two independent reconstructors
must still prove the claim from a fresh execution root before Hubbard
population execution.

## Prospective Review Disposition

- Reviewer A: `PASS`, no blocking findings.
- Reviewer B: `HOLD`, five blocking findings covering post-result labeling,
  finite competition, crossing counts, reset observability, and design/schema
  custody.
- Parent disposition: all Reviewer B findings are accepted and incorporated
  into the prospective correction above. Implementation remains blocked until
  both reviewers re-review the amended design.
