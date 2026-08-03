# SNOWBIRD-SNOTEL-CLIGEN-SWE-RESPONSE

Status: `complete / mediated response improves but remains insufficient / no correction`

Date: `2026-08-03`

Plan class: `Result-bearing forcing sensitivity / calibration evidence`

## Purpose

Build observed-mode CLIGEN `.prn` inputs from the Snowbird SNOTEL record,
generate controlled `.cli` variants, and quantify effects on peak SWE and melt
chronology in the real Snowbird baseline snowbench consumer.

## Frozen Experiment

Run one original-fixture reference and four CLIGEN-observed variants over
1990-01-01 through 2024-12-31:

- `cligen_control`: original fixture precipitation, Tmax, and Tmin quantized to
  CLIGEN observed-input units;
- `snotel_p`: substitute guarded SNOTEL precipitation at the `.prn` input;
- `snotel_t`: substitute SNOTEL Tmax/Tmin at the `.prn` input only when both are present and
  ordered;
- `snotel_pt`: substitute both `.prn` input groups according to those independent guards; and
- `original_fixture`: unchanged `barred-pro` climate reference.

Missing or inadmissible SNOTEL values fall back explicitly to the exact-date
original fixture value; CLIGEN's `9999` generation sentinel is not used. All
four `.prn` files therefore have identical complete chronology and isolate
which observed variables are substituted. Precipitation is the same-water-year
consecutive-date cumulative difference with the frozen `-1e-9 mm` rule from the
predecessor. `.prn` precipitation uses hundredths of an inch and temperatures
integer Fahrenheit, matching CLIGEN observed-mode authority.

The P/T factorial is isolated at `.prn` input columns. CLIGEN mediates those
inputs into realized storm duration, peak timing/intensity, dewpoint, and
interactions; no precipitation-only or temperature-only realized-forcing claim
is made. All variants use the same Alta `ut420072.par`, CLIGEN binary, RNG burn, and
monthly-mean-preserving interpolation. The original reference is necessary to
measure the effect of observed-mode quantization and regenerated secondary
weather fields before attributing SNOTEL substitutions.

## Authority, Roles, And Claims

- SNOTEL observations are `CALIBRATION` evidence already used in the snow arc;
  independent-validation count is zero.
- Production, tests, fixtures, observations, contracts, defaults, selectors,
  and snow physics are read-only.
- Generated `.prn`, `.cli`, run outputs, and traces live under a package-named
  `target/` tree; package artifacts bind their hashes and summarize results.
- No production forcing correction, transferability, provider admission,
  independent validation, or snow-model promotion is authorized.

## Write Set

- This package tree and its package-named `target/` output tree.
- `docs/ROADMAP.md`, `docs/planning/snow-surface-energy-balance-roadmap.md`, and
  `docs/work-packages/README.md`.

## Exit Criteria

- Freeze exact inputs, binaries, operators, variants, period, and hashes before
  result-bearing execution.
- Verify generated `.prn` substitutions and `.cli` chronology/identity
  independently; bind all outputs.
- Execute all five cells through the real snowbench consumer and independently
  reconstruct peak-SWE ratio, peak-date offset, melt-out offset, effective
  input, retained storage, pre-peak loss, and mass closure.
- Run direct syntax/JSON/Markdown/diff/protected-path/bytecode gates.
- Complete dual independent review, finding disposition, and dual exact-current
  verification.

## Delegation

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent read-only science/data-governance
reviewers and two read-only terminal verifiers. Expected outputs are compact
package-local evidence; write access is read-only.

## Progress

- [x] (2026-08-03) User authorized the successor experiment.
- [x] (2026-08-03) Frozen factorial, fallback, units, claims, and write set.
- [x] (2026-08-03) Generated four controlled observed-mode CLIGEN climates and
  executed all five Snowbird cells through the real consumer.
- [x] (2026-08-03) Rejected the initial run for review-found provenance and
  interpretation defects; froze and executed corrected terminal-v2 evidence.
- [x] (2026-08-03) Completed dual fresh terminal-v2 re-review with `PASS`.
- [x] (2026-08-03) Completed dual terminal verification and closeout.
