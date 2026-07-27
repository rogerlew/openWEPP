# Intent Plan

Status: `PROSPECTIVE FREEZE / HOLD CANDIDATE`

Evidence class: `Static: prospective; Ran: authority/input intake only`

## Authorized intent

This package may perform a deterministic, probability-prior-free calibration
in declared process order using only admitted Hubbard Brook observations.
Harvard Forest, downstream consumer outputs, current parameter values, Bill
Elliot values, and legacy agreement are never selection authority. Production
code, contracts, protected fixtures, and source observations are read-only.

## Prospective execution decision

No result-bearing candidate is admitted to execution. The controlling
contracts provide validation domains, but do not provide finite calibration
search bounds for the complete first-stage GSI vector:

- both temperature thresholds are only finite and ordered;
- VPD thresholds are finite, ordered, and non-negative, with no upper bound;
- photoperiod thresholds are finite, ordered, and bounded by 0 and 24 hours.

The admitted prior-free design requires a reproducible bounded grid or
ensemble. The package forbids inventing scientific bounds, treating current
values as bounds, or widening domains retrospectively. Consequently, the
initial grid and deterministic finite enumeration cannot be frozen without a
new prospectively admitted search-domain authority object. This is an
authority boundary, not an optimization failure.

Later process stages do not have sufficient separating authority.
`CAL03-OBS-HB-001` admits total aboveground biomass for the biomass-partition
sum, but cannot separately identify `Bf,max` and `Bs`.
`CAL03-OBS-HB-005` admits a bounded mature leaf-on LAI range, but cannot
identify `xmxlai` until upstream `fe` and GSI are frozen. No quantitative
selection authority is admitted for `fe`, `Cs`, or `bb`; the canopy records
remain qualitative/proxy screens. Timing cannot substitute for magnitude,
partition, LAI, or canopy-cover authority.

## Frozen behavior if the blocker is lifted prospectively

- Enumeration: lexicographic by the six YAML GSI field names, then numeric
  ascending values; no seed.
- Objective: the frozen equal-year interval RMSE in
  `objective-and-observation-operator.md`.
- Missing crossing: candidate objective `+infinity`; retain candidate, failed
  years, and failed records.
- Retry: no scientific retry; one retry only for a classified infrastructure
  failure, preserving the first failure.
- Boundary hit: report `BOUNDARY_HIT`; never widen in this package.
- Refinement: none until explicit finite bounds, initial grid, prospective
  refinement triggers, maximum refinements, and stopping rules are admitted
  and independently approved.
- Stage advance: only after the current stage has accepted ranges and an
  upstream freeze identity.
- Reopening: only by a prospectively reviewed amendment; never because of
  Harvard or downstream results.
- Holdout: remains sealed until a nonempty accepted calibration ensemble,
  analysis tool, complete configuration, and exact command are checksum-frozen.

## Selected terminal gates

Run authority/input checksums, exact timing-window rebuild, role/disjointness
checks, prospective dual review, empty candidate/failure consistency,
stage-order/hold consistency, holdout-seal proof, documentation lint, diff
hygiene, prompt state, write-set reconciliation, dual terminal review, finding
disposition, and dual terminal verification.

Candidate execution, objective reconstruction from candidate traces,
deterministic candidate rebuild, holdout opening, and downstream evaluation
are `BLOCKED` by the prospective authority boundary and cannot be represented
as passed.

## Terminal reconciliation

Before disposition, enumerate every changed/untracked path, compare it with the
declared write set, verify that candidate and holdout ledgers remain empty,
checksum all relied-upon package artifacts, and bind the final inventory in
`execution-inventory.csv` and `gate-evidence.md`. Any executed candidate,
opened Harvard result, protected-input change, or path outside the write set is
a closure-blocking violation.
