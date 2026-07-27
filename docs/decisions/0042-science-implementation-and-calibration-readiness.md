# ADR-0042: Implement authoritative science and preserve calibration readiness

**Status:** Accepted
**Date:** 2026-07-27 UTC
**Deciders:** Roger Lew, Codex

## Context

openWEPP often has adequate authority for process equations, invariants, units,
state transitions, and guard behavior while observations remain sparse,
correlated, interval-censored, scale-mismatched, or insufficient to identify
every parameter. Treating those data limitations as a reason not to implement
authoritative science confuses implementation correctness with empirical
parameter estimation. Conversely, filling evidence gaps with convenient
defaults, surrogate physics, invented physiological bounds, or synthetic-data
claims would overstate scientific support.

## Decision

1. Science authority, data authority, and calibration readiness are distinct:
   - science authority governs equations, algorithms, units, invariants,
     domains, guards, and parameter meaning;
   - measured-data authority assigns prospectively separated calibration,
     independent-validation, and diagnostic roles and governs empirical
     parameter estimation, predictive validation, and transferability claims;
   - calibration readiness governs whether typed parameters, observations,
     objective operators, deterministic execution, diagnostics, and failure
     reporting support calibration when suitable data are available.
2. When authoritative process science exists, openWEPP implements it even when
   calibration data are absent or insufficient. Data limitations restrict
   empirical claims; they do not justify a production implementation hold.
3. When calibration is scientifically applicable but data are insufficient,
   the implementation must demonstrate calibration readiness and disposition
   every row in the repository readiness matrix, including:
   - typed and enumerable parameter/configuration surfaces;
   - a unit- and scale-defined observation operator;
   - deterministic candidate execution and objective reconstruction;
   - local sensitivity and identifiability analysis;
   - synthetic parameter-recovery tests where structurally meaningful;
   - boundary, failure, saturation, covariance, and equifinality reporting;
   - an explicit inventory of additional observations needed for stronger
     identification or validation.
4. Synthetic recovery proves only that the implementation and calibration
   machinery can recover information present in suitable synthetic data. It is
   not empirical calibration, real-world parameter identification, external
   validation, or transferability evidence.
5. Search bounds or fixed values introduced only to execute a readiness
   demonstration are labeled execution assumptions. They are not probability
   priors, physiological bounds, observations, or calibrated results.
6. Calibration and science posture use three orthogonal package fields:
   - `science_implementation_status`: `IMPLEMENTED`, `NOT_IMPLEMENTED`, or
     `AUTHORITY_MISSING`;
   - `calibration_evidence_status`: `EMPIRICALLY_CALIBRATED`,
     `CALIBRATION_READY_DATA_LIMITED`, `NOT_CALIBRATION_READY`, or
     `NOT_APPLICABLE`;
   - `identifiability_status`: `IDENTIFIED`, `PARTIALLY_IDENTIFIABLE`,
     `NONIDENTIFIABLE`, `NOT_ASSESSED`, or `NOT_APPLICABLE`.
   These fields do not replace ordinary package disposition. Any unresolved
   required current-scope gate still forces `HOLD`.
7. ADR-0024 source-intent anchors and ADR-0028 observed-data admission remain
   valid routes to establish authority when their criteria are satisfied.
   `AUTHORITY_MISSING` and `HOLD` apply only when authority is absent or
   contradictory and no applicable admission route has succeeded.
8. `HOLD` also remains required for unimplemented authoritative in-scope
   science, invalid correctness gates, or a required calibration-readiness
   defect. Missing, sparse, or non-identifying measured data alone does not
   justify holding an otherwise authoritative implementation.

## Consequences

- A0/A1/A3 implementation correctness remains mandatory and is not weakened by
  unavailable A4 measured validation.
- Data-limited packages continue through authoritative implementation and
  calibration-readiness evidence instead of manufacturing calibration
  authority or stopping prematurely.
- Science contracts disclose calibration applicability, observation needs,
  identifiability limits, and prohibited claims.
- Work packages distinguish empirical calibration from readiness,
  sensitivity, synthetic recovery, and assumption-bounded execution.
- Calibration and independent-validation observations receive prospectively
  immutable, disjoint roles; reuse requires a prospectively justified and
  independently reviewed exception that does not claim independence.
- New observations can be admitted later without redesigning opaque or
  non-enumerable process implementations.
