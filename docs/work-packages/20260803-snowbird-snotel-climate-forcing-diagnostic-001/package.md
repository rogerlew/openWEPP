# SNOWBIRD-SNOTEL-CLIMATE-FORCING-DIAGNOSTIC

Status: `complete / bounded dry-forcing evidence / no correction`

Date: `2026-08-03`

Plan class: `External-observation forcing diagnostic`

## Purpose

Test whether the retained Snowbird climate fixture from WEPPcloud run
`barred-pro` materially disagrees with colocated NRCS SNOTEL precipitation and
air temperature. This is a diagnostic of forcing representativeness, not a
forcing correction, calibration, provider admission, or snow-model verdict.

## Frozen Authority And Operators

- Comparator: `tests/fixtures/snotel_observed/snotel_snowbird_ut/p8.cli`, exact
  SHA-256 bound before execution.
- Observation: normalized NRCS AWDB station `766:UT:SNTL` record and its
  provenance receipt, exact SHA-256 bound before execution. Its role is
  `DIAGNOSTIC_ONLY` for this package.
- Interpret SNOTEL `PREC` as water-year cumulative precipitation. Form daily
  increments only for consecutive dates in the same water year; reject
  differences below `-1e-9 mm`; canonicalize only smaller floating residuals
  to zero; and do not bridge missing dates or water-year resets.
- Compare matched October-1-boundary through September-30-boundary intervals
  only when both cumulative records exist and the corresponding October 2
  through September 30 fixture interval is complete. This boundary-difference
  sensitivity is not labeled a complete water-year total.
- Compare daily fixture precipitation against guarded SNOTEL increments on the
  exact common dates; separately report wet-day occurrence and positive-event
  quantiles.
- Compare daily `tmax` and `tmin` directly on finite common dates. Report signed
  fixture-minus-SNOTEL bias, MAE, correlation, and seasonal summaries. Treat
  NRCS's documented SNOTEL temperature bias as a limitation.
- Use fixed calendar and water-year groupings without daylight-saving logic.
- Preserve fixtures, observations, production, tests, and all snow parameters
  read-only.

## Write Set

- This package tree.
- `docs/ROADMAP.md`.
- `docs/planning/snow-surface-energy-balance-roadmap.md`.
- `docs/work-packages/README.md`.

## Deliverables And Gates

1. Frozen comparison manifest and required-reading map.
2. Deterministic comparison tool and checksum-bound JSON receipt.
3. Readable scientific disposition separating measurement, representativeness,
   and causation limits.
4. Direct Python syntax, JSON, Markdown, diff, protected-path, overwrite, and
   bytecode-hygiene gates.
5. Dual independent review with finding disposition and dual exact-current
   terminal verification.

## Delegation

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent read-only science/data-governance
reviewers and two read-only terminal verifiers. Expected outputs are compact
package-local review and verification evidence; write access is read-only.

## Progress

- [x] (2026-08-03) User authorized the separate SNOTEL forcing diagnostic.
- [x] (2026-08-03) Frozen authority, operators, claim limits, and write set.
- [x] (2026-08-03) Executed the checksum-bound comparison and published the
  machine receipt and scientific disposition.
- [x] (2026-08-03) Remediated initial review findings and completed dual fresh
  exact-current review with `PASS`.
- [x] (2026-08-03) Completed dual exact-current verification and closed with
  bounded dry-forcing evidence, explicit limitations, and no correction.
