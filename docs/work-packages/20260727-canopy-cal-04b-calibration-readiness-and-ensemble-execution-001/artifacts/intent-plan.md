# Intent Plan

Status: `EXECUTOR IMPLEMENTATION INTENT / PROSPECTIVE REVIEW REQUIRED`

Evidence class: `Static`

Intent is the combined implementation-path verification, calibration readiness,
empirical GSI timing calibration, and independent Harvard validation declared
in `package.md`. Production code, contracts, fixtures, and predecessor evidence
remain read-only.

The blocked prototype remains retired. The accepted rework controls are
implemented by the following package-local programs:

1. `prepare.py` authenticates non-holdout inputs, rebuilds the exact 9,261
   configuration inventory, computes CAL-04A saturation classes, and writes
   execution manifests without reading Harvard content.
2. `native-producer` reads the frozen configuration inventory, CAL-04A's
   checksum-bound plot-specific Daymet daily derivation, and source-EML plot
   latitudes. For every candidate and `(plot_id, year)` it creates a fresh
   `GsiState`, admits real yday 1–180 with no prefill or cross-year carry, and
   writes candidate/plot/year/day-keyed `f64` daily GSI values. Days 1–59 are
   retained warm-up only; crossing eligibility is frozen to yday 60–180. It
   writes no scores or accepted flags. The protected Hubbard `p10.cli` is
   native-path-proof comparison evidence only.
3. `reconstruct` independently parses the climate calendar, daily trace,
   configuration inventory, and admitted Hubbard observations. It derives
   first crossings, observation distances, annual components, diagnostics,
   objectives, failures, and accepted membership without producer scoring code
   or derived inputs.
4. `native-proof.py` runs copied Hubbard configurations through
   `openwepp-cli-hill --direct-production-executor`, reads the retained
   `openwepp-canopy-research-daily-v1` trace, and bit-compares `/gsi/gsi21`
   against direct-kernel expected output for native-default, interior,
   boundary, and saturated cases. An invalid unordered pair must fail before
   trace publication.
5. `synthetic-gsi` runs before Hubbard scoring. It generates a fixed daily
   forcing case from hidden interior `GSI-5557`, constructs closed timing
   intervals containing that truth, executes native production plus both
   reconstructors for `GSI-0001`, `GSI-5557`, and `GSI-9261`, and requires both
   reconstructed minimum sets to contain `GSI-5557` with identical components.
6. `readiness` executes the frozen `later-stage-design.csv` synthetic cases and
   full non-lossy upstream membership propagation, retaining recovered sets,
   sensitivities, boundaries, invalid failures, and equifinality.
7. `freeze.py` checksum-freezes accepted membership, tools, configurations,
   commands, receipts, and expected Harvard identities without reading Harvard.
8. `holdout.py` exclusively creates the durable opening token before its
   first Harvard read, refuses rerun, invokes native holdout trace production
   and independent scoring, and has no calibration-artifact write target.

## Trace Schema

The external Hubbard object is a binary stream plus a checksum-bound canonical
identity sidecar:

- magic/version declared by the implemented multi-lane schema;
- little-endian `u32` candidate count;
- explicit plot-lane and plot-year calendar cardinalities;
- candidate-major, then canonical plot/year/day-major little-endian `f64`
  GSI21 values.

The sidecar binds trace SHA-256/bytes, candidate-inventory SHA-256, canonical
plot/year/day calendar SHA-256, Daymet derivation SHA-256, geometry SHA-256,
the exact nine ordered `plot_id`/lane/latitude identities, site
`hubbard_brook`, arm `deciduous`, annual cold-start and yday 60–180 eligibility
rules, exact counts, producer source/binary hash, command, and typed
failure-ledger hash. Candidate and plot/year/day order are independently
rebuilt and their hashes must match before values are read.
Expected size is checked exactly. The retained object is compressed with
`zstd` only after both reconstructions and its raw/compressed checksums, byte
counts, paths, commands, and tool identity are recorded.

## Result Schemas

- `candidate-annual-components.csv`:
  `(candidate_id,year,observation_count,annual_mse,annual_rmse)` with
  plot-specific crossings represented in the observation components;
- `candidate-observation-components.csv`:
  `(candidate_id,record_id,plot_id,year,species,crossing_doy,lower_doy,`
  `upper_doy,distance_days,squared_distance)`;
- `candidate-ledger.csv`: frozen scaffold columns plus reconstructed objective;
- `failure-ledger.csv`: every missing crossing or typed execution failure;
- `accepted-calibration-ensemble.csv`: reconstructed membership only.
- `saturation-evidence.csv`: exact population/window/min/max/range/result per
  candidate and family;
- `producer-failure-ledger.csv`: every typed producer failure; complete valid
  grid execution requires zero rows.

Completeness is exact across 9,261 candidates, nine Hubbard plot lanes, 313
observed plot-years, all 324 Daymet plot-years, and all 932 calibration
observations.

## Execution Boundary

Implementation and focused non-result tests may begin only after two
prospective reviewers accept this intent and source design. The exact source
paths, commands, environments, workdirs, I/O, and DAG are frozen in
`executor-command-plan.csv`. Hubbard population execution then uses the
authorized `comparator_suite_runner`. Harvard remains sealed until the accepted
ensemble and complete canonical freeze manifest receive two independent
receipts stored outside that manifest.

Before Harvard access, the nonempty accepted membership and exact analysis,
configuration, input, and command identities must be checksum-frozen in the
holdout-opening record. Harvard uses the same equations and objective with the
fall crossing and cannot change any calibration choice.

Later-stage axes are deterministic readiness demonstrations. Values used only
to execute them are `ASSUMED_FOR_EXECUTION`; sparse evidence may constrain a
combination but cannot be represented as unique operand identification.

Terminal gate selection:

- Daymet custody checksum verification;
- package validator;
- package Markdown lint;
- `git diff --check`;
- dual prospective review before result-bearing execution;
- comparator-role population execution;
- dual terminal scientific review and dual terminal verification.

Selected implementation gates are Rustfmt, package-local Cargo tests, Python
unit tests, scaffold validation, Markdown lint, and diff check. Selected
result gates additionally include native consumer proof, exact trace-size/hash
checks, independent objective/membership reconstruction, readiness recovery,
dual freeze verification, one-time holdout state validation, and terminal dual
review/verification. No production source or workspace Rust campaign is
selected because production code remains read-only.
