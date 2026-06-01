# SOILAUTH02 Worker Handoff

Status: complete  
Evidence mode: Static

## Scope
Handoff target: `SOILAUTH03` anti-drift guard implementation.

## Immediate Next Actions
1. Add machine-checkable guards that enforce canonical `.sol` envelope
   obligations:
   - policy-first/header-first policy-row acceptance invariants,
   - quoted-header optional `avke` normalization invariants,
   - per-OFE restrictive-row identicality + normalization invariants,
   - single/double quote-tokenization invariants.
2. Add fixture-lock guard checks that validate
   `tests/fixtures/infile/soil/fixtures.sha256` and
   `tests/fixtures/infile/soil/fixtures.provenance.yaml` consistency.
3. Add release-lane checks to prevent unreviewed drift on canonical producer
   conformance fixtures.

## Residual Follow-On Outside SOILAUTH02 Scope
- Investigate/close `auth05_level4_constitutive_authority_hardening_contract`
  FC mismatch lane (`thetfc_0001`) if needed for workspace-wide green status.
