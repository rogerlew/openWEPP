# ADR-0027: Opt-in physics-bulk snow model for snow-density remediation

**Status:** Accepted
**Date:** 2026-06-25 UTC
**Deciders:** Roger Lew (operator direction), Codex (draft)
**Builds on:** [ADR-0017](0017-re-pin-operational-distrust-comparator-is-flag-not-target.md),
[ADR-0024](0024-reference-implementation-intent-authority.md),
[ADR-0026](0026-stateful-winter-column-sub-solver.md)
**Science authority:**
[`SC-SNOWFREEZE-001`](../specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md)
**Evidence package:**
[`SNOWDENSITY-01`](../work-packages/20260625-snowdensity-01-evidence-reconciliation-001/package.md)

## Context

SNOWFROST-FIDELITY and SNOWDENSITY-01 established that current openWEPP and
pinned legacy WEPP share the same structural snow-density/depth problem for the
SNOTEL comparison. The maximum as-built openWEPP-vs-legacy density delta in the
H evidence is only `4.351046738461008 kg m^-3`, while both profiles route to
structural density/depth failure. Legacy bit-parity therefore cannot remediate
the snow-depth blocker that currently prevents frost-depth attribution.

PySnobal/SNOBAL runs provide useful diagnostic profile evidence, but they are
not a drop-in openWEPP runtime dependency and are not an acceptance target under
ADR-0017. The snow/frost runtime architecture already has a stateful winter
column boundary under ADR-0026, so the appropriate migration shape is a typed
snow sub-solver candidate rather than another compatibility retrofit.

## Decision

Authorize a deliberate-legacy-divergence snow model lane named
`physics_bulk`, selected only through:

```text
snow_model = legacy_wepp | physics_bulk
```

Binding rules:

- `legacy_wepp` remains the default production behavior, compatibility flag
  profile, and rollback path.
- `physics_bulk` is opt-in candidate scope only until a later package ratifies
  equations/constants, validates profile evidence, and explicitly changes
  activation status.
- `physics_bulk` must not use site-specific tuning. SNOTEL, paired snow-depth
  observations, legacy WEPP, and PySnobal classify cells and failure modes; they
  do not fit per-site constants.
- The first implementation target is an offline Rust snowbench physics core,
  followed by offline adjudication, then runtime opt-in only if the contract
  gates pass.
- Candidate process families are limited to bulk snowpack SWE/depth/density,
  temperature/wind-dependent fresh-snow density, Anderson-1976/SNOBAL-style
  metamorphism, overburden and wet-snow compaction, liquid retention/release/
  refreeze, and internal mass/energy closure.
- This ADR does not authorize replacing frost physics, tuning `ssd`, default
  activation, or deleting the legacy snow runtime.

## Consequences

Positive:

- Snow-density remediation can move toward observation-scored physics without
  pretending legacy is a correctness target.
- The no-site-tuning rule is explicit before candidate equations are written.
- The path composes with ADR-0026: snow remains a typed winter-column sub-state
  with independent `SC-SNOWFREEZE-001` obligations.

Negative / cost:

- Future packages must maintain two snow paths until the candidate earns
  runtime promotion.
- Candidate equation selection requires additional literature binding and
  hydrology review before production coupling.
- Improvements that are visible only in forcing-limited magnitude cells cannot
  justify promotion by themselves.

Required gates before runtime promotion:

- `SC-SNOWFREEZE-001#INV-SNOWFREEZE-051` and
  `OBL-SNOWFREEZE-P-026` pass.
- Offline `physics_bulk` state proves independent SWE, physical depth, density,
  retained-liquid, and thermal-state closure.
- v74/v75 snow-frost rubric profiles show forcing-robust improvement or a
  documented non-promotion reason.
- `legacy_wepp` remains available as default and rollback until a later ADR or
  contract amendment explicitly changes that status.

## Non-decisions

This ADR does not select exact equations/constants, implement production
runtime physics, approve PySnobal as a runtime dependency, re-open frost
bit-parity work, change direct-runtime defaults, or relax ADR-0017's comparator
distrust posture.
