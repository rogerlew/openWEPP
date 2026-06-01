# SOILAUTH02 Disposition

Status: complete  
Evidence mode: Static + Ran
Decision: GO

## Scope
SOILAUTH02 objective is satisfied for the declared mismatch set.

## Closure Summary
- Closed all SOILAUTH01 mismatch items `SA01-M001..SA01-M004`.
- Reconciled contract/spec/parser behavior for canonical producer envelopes:
  - `9002/9003/9005` policy-first ordering accepted;
  - quoted `7778/9002/9003/9005` headers accepted with optional omitted
    trailing `avke` normalized to `0.0`;
  - per-OFE restrictive-row normalization accepted for
    `7778/9002/9003/9005` with identical-row enforcement;
  - single-quoted + double-quoted tokenization accepted where lossless.
- Added SOILAUTH02 contract-derived test lane and canonical apostrophe-bearing
  policy-token fixture.
- Added fixture hash/provenance lock sidecars for canonical reconciliation
  fixtures.

## Residual Notes
- Full workspace test gate currently fails in unrelated FC authority suite:
  `auth05_level4_constitutive_authority_hardening_contract`
  (`thetfc_0001` mismatch); this failure is outside SOILAUTH02 declared
  write-set/objective.
