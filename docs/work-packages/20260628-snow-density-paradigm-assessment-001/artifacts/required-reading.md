# Required Reading Notes

Evidence class: Static.

## Package And Governance

- Read `docs/work-packages/20260628-snow-density-paradigm-assessment-001/package.md`.
  The package is design/architecture scope only: comparison, recommendation, and
  ADR-candidate draft. Production code, fixture, schema, default, frost, and
  contract amendments are explicitly out of scope.
- Read `docs/work-packages/AGENTS.md`. Closure requires direct evidence for each
  current-scope gate, dual reviews, finding disposition, dual verification,
  line-count governance, and a final disposition. The package does not authorize
  delegated subagents, so review artifacts are local review passes rather than
  delegated agent outputs.
- Read `docs/specifications/science-contracts/AGENTS.md` because the package
  reasons about snow process authority. No science-contract edit is in scope.

## Snow-Density Lineage

- Read `docs/planning/snow-frost-fidelity-strategy.md` section 10.3 step 8.
  SNOWDENSITY-10.3.18 scored supported mechanisms, legacy flags, and PySnobal on
  the cross-SNOTEL forcing-robust rubric. `harder_pomeroy_partition` was the
  strongest supported lever at `15` robust fails / `179` score.
- Read SNOWDENSITY-10.3.19 completion notes in the same strategy file. The
  no-env default is now the activated melt+density bundle composed with
  `harder_pomeroy_hourly`, preserving explicit `legacy_rst` rollback.
- Read SNOWDENSITY-10.3.20 completion notes and
  `docs/work-packages/20260628-snowdensity-10-3-20-sublimation-stage-b-unlock-001/artifacts/claude-review-mechanism-family-exhausted.md`.
  Stage A sublimation, partition+sublimation composition, and Stage B
  surface-layer sublimation did not beat the current default. The artifact
  concludes the SNOBAL/CoE/Anderson bulk family is exhausted and later levers
  must be new mechanism classes under ADR-0028.
- Read SNOWDENSITY-10.3.21 strategy notes and
  `docs/work-packages/20260628-snowdensity-10-3-21-post-partition-residual-decomposition-001/artifacts/post-partition-residual-decomposition.md`.
  The current default remains `15` / `179`; residuals are
  signature-concentrated but site-diffuse. Densification trajectory accounts for
  `9/15` robust fails, humid-New-England depth-SWE geometry for `2/15`, and
  mountain timing under-persistence for `4/15`.

## Decision Authorities

- Read ADR-0011. Architecture-first, top-down science contracts remain the
  preferred path; legacy source and observed evidence inform contracts but do
  not replace them.
- Read ADR-0017. Legacy and PySnobal are comparator flags, not optimization
  targets.
- Read ADR-0025. The array-native hot path constrains any snow-density design
  that would expand runtime state or add high-cost per-cell iteration.
- Read ADR-0026. The winter column is the accepted snow/frost state boundary;
  snow and frost remain typed sub-states, and frost can read prior snowpack
  state without collapsing into one process.
- Read ADR-0027. The existing opt-in `physics_bulk` lane admits bulk SWE/depth/
  density process modernization but does not authorize arbitrary multilayer
  state or exact equation choices.
- Read ADR-0028. Observed-data admission is allowed when scientific authority is
  under-specified if physics are defensible, forcing-robust rubric evidence
  improves, fixture fitting is avoided, comparators remain flags, and
  conservation is non-negotiable.

## Literature And Reference Context

- R-58, Sturm et al. 2010, is the primary regime-divergent bulk-density
  authority. It models bulk density as a function of depth, day of year, and
  snow climate class, with class-specific `k1` and `k2` parameters.
- R-59, Sturm et al. 1995, is the classification authority for the six seasonal
  snow classes and the wind/precipitation/temperature decision tree. The local
  reference is still a track-down item; NSIDC-0768 is the gridded successor.
- R-40, Vionnet et al. 2012 Crocus/SURFEX, is the local redistributable primary
  authority for detailed multilayer snowpack structure with settling,
  metamorphism, and albedo.
- R-48 Marks 1999 and R-35/R-52 Anderson/WEPP winter hydrology remain the current
  SNOBAL/Anderson/CoE lineage for bulk compaction and winter hydrology, but the
  recent package sequence found that family no longer supplies a winning global
  lever.

