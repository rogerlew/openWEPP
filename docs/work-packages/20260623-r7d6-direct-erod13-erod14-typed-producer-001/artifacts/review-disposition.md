# Review Disposition

Status: executed-held.

## Review A

- Static: direct runtime now has a typed WB16 peak-duration span and typed
  EROD13/EROD14/EROD15 span with inputs, direct compute, state mutation,
  downstream operands, and shadow projection. `DirectFrameExecutor` runs the
  peak span after R4A runoff partition and before storage/projection/erosion;
  the erosion span reads the direct peak shadow projection instead of requiring
  `peakro`/`watdur` from a compatibility-shaped seed surface.
- Static: runner direct day-input construction passes explicit
  `DirectPeakRunoffInputs` from the raw hyetograph, `efflen`, `ealpha`, `m`,
  and irrigation runtime rate; EROD13/EROD14 peak fields are placeholders that
  are filled from direct WB16 state during the erosion span.
- Finding A1: initial R7D6 seed used `erod14_class_count = ofe_count`, causing
  `erosion.erod14.class_count` direct-domain failure on H2637. Disposition:
  accepted and fixed; the seed now uses the five HBP/SC-SED particle classes.
- Finding A2: initial direct publication still required `peakro`/`watdur` from
  the seed surface before direct spans ran. Disposition: accepted and fixed by
  `DirectPeakRunoffInputs` plus the direct WB16 peak-duration span.
- Finding A3: initial direct WB16 port failed zero-`remax` H2637 days even
  though the existing hydrology kernel has an explicit floor/zero-duration
  branch. Disposition: accepted and fixed by matching the WB16 zero-`remax`
  branch.

## Review B

- Ran: focused R7D5/R7D6 orchestrator tests passed.
- Ran: runner direct-production tests passed with compatibility-edge counters
  at `0`.
- Ran: H2637 direct production exits `0`; WAT is byte-identical; PASS sediment
  fields are parity-clean.
- Finding B1: R7D6 introduced non-parity PASS `tdep = 0.3` from a fabricated
  MOFE03 Wave-2 default `erod14_lddend`. Disposition: accepted and fixed by
  changing the default to `0.0` unless direct seed authority provides a real
  deposition value.
- Finding B2: PASS/HBP still differ on `peakro` because direct WB16 now emits
  producer-authoritative values while compatibility emits zero. Disposition:
  follow-up. This is recorded as
  `HOLD-R7D6-PASS-HBP-PEAKRO-COMPATIBILITY-ZERO-RESIDUAL` and scaffolded into
  R7D7.
- Finding B3: line-count governance remains unresolved for touched production
  file `crates/openwepp-runner/src/hillslope/04_direct_publication.rs`
  (`3243` lines). Disposition: follow-up/blocking for full closure; R7D6 is
  held and does not claim full root closure.

## Finding Disposition

- Accepted and fixed: class-count seed cardinality, missing direct WB16
  peak-duration producer, zero-`remax` WB16 branch, fabricated
  `erod14_lddend = 0.3` default.
- Follow-up: PASS/HBP `peakro` compatibility-zero residual and
  `04_direct_publication.rs` 3000+ line-count split.
- Rejected: none.
