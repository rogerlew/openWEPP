# PL15 Semantic-Parity Direction Assessment

Status: `complete`
Evidence mode: `Static + Ran`
Assessment: `UNRESOLVED / HOLD (scope-narrowed only)`

Static:
- Semantic parity (not bitwise parity) remains canonical project target.
- Tier-A strict replay surfaces remain high-confidence acceptance-direction
  signals.
- Claude pre-closeout physics review identifies unresolved kernel-scope gaps
  (`KERNEL-GAP-001..012`) that constrain literal parity claims.

Ran:
- PL14 strict replay artifacts show:
  - `H5.wat.dat`: `strict_pass=false` with structural mismatch.
  - `H5.plot.dat`: `strict_pass=false` with candidate artifact absence.

## Direction Verdict

`UNRESOLVED / HOLD`

Why:

1. Direct openWEPP-vs-legacy strict Tier-A replay exists, but both required
   Tier-A surfaces remain failing.
2. Failure signatures are structural/artifact-level and remain high-confidence
   blockers under Tier-A policy.
3. Claude pre-closeout review confirms unresolved physics-comprehensiveness
   gaps in production kernel coverage (critical: infiltration and within-day
   hyetograph integration; plus open growth/decomposition and other domain
   kernels), so a broad parity claim would be overstated.
4. No formal risk-acceptance approval reference exists for unresolved blockers
   in this package execution.

## PL09 GAP-002 Carry-Forward Note

PL09 `GAP-002` (`H5.wat.dat` structure mismatch) is not resolved in PL15.
PL14 direct replay confirms the mismatch persists (`line_count_baseline=1123`,
`line_count_candidate=5`), so this is not treated as a worked-around closure.

## Chosen Disposition Path

From `claude-pl15-pre-closeout-physics-review.md` options, PL15 adopts path
`(2) remain on HOLD`, and issues actionable follow-on queue disposition for
`KERNEL-GAP-001..012`.
