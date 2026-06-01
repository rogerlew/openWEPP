---
suite_id: cas_l4_infile_soil_producer_contract_001
title: Soil Input Producer-Contract Anti-Drift Guard Suite
status: active
authority_level: 4
domain: infile
process_family: soil_producer_contract
sc_invariant_refs:
  - SC-INFILE-SOIL-001#G-SOL-006
  - SC-INFILE-SOIL-001#G-SOL-010
  - SC-INFILE-SOIL-001#G-SOL-011
external_citations:
  - citation_id: EXT-INFILE-SOIL-WEPPPY-001
    source_type: solver_doc
    title: Canonical wepppy soil producer envelopes
    locator: /workdir/wepppy/wepppy/wepp/soils/utils/wepp_soil_util.py
    version_or_edition: local workspace snapshot
    retrieved_utc: 2026-06-01
    notes: Canonical producer reference for policy-first ordering, quoted headers, and restrictive-row placement envelopes.
fixtures:
  - fixture_id: FX-SOIL-PRODUCER-9002-POLICY-FIRST-001
    path: tests/fixtures/infile/soil/compat_quoted_header_9002_policy_first.sol
    fixture_class: integration
    units_basis: infile_soil
    hash: deed1e62319fdd7381fe14482aff56c406a3a3049e3cea87011ad631b79dd5c5
    source_repo: /workdir/openWEPP
    source_commit: 42dcd326d326fef29c5141a3f41927e31a63adec
    source_path: tests/fixtures/infile/soil/compat_quoted_header_9002_policy_first.sol
    source_sha256: deed1e62319fdd7381fe14482aff56c406a3a3049e3cea87011ad631b79dd5c5
    transform_note: Canonical policy-first envelope with quoted header and omitted trailing avke normalization.
  - fixture_id: FX-SOIL-PRODUCER-7778-RESTRICTIVE-001
    path: tests/fixtures/infile/soil/compat_quoted_header_7778_per_ofe_restrictive.sol
    fixture_class: integration
    units_basis: infile_soil
    hash: 99127d0d9ad418b01bb01bc76f020a18151c801b8c19c9111457a9dfb1580e56
    source_repo: /workdir/openWEPP
    source_commit: 42dcd326d326fef29c5141a3f41927e31a63adec
    source_path: tests/fixtures/infile/soil/compat_quoted_header_7778_per_ofe_restrictive.sol
    source_sha256: 99127d0d9ad418b01bb01bc76f020a18151c801b8c19c9111457a9dfb1580e56
    transform_note: Canonical per-OFE restrictive-row normalization envelope for 7778.
  - fixture_id: FX-SOIL-PRODUCER-9002-DOUBLE-QUOTED-001
    path: tests/fixtures/infile/soil/canonical_9002_double_quoted_policy.sol
    fixture_class: integration
    units_basis: infile_soil
    hash: 78f15f645f87ee856fb3e99426b316c6a568a9368004060eb8f3b1766972cbd7
    source_repo: /workdir/openWEPP
    source_commit: 42dcd326d326fef29c5141a3f41927e31a63adec
    source_path: tests/fixtures/infile/soil/canonical_9002_double_quoted_policy.sol
    source_sha256: 78f15f645f87ee856fb3e99426b316c6a568a9368004060eb8f3b1766972cbd7
    transform_note: Canonical apostrophe-bearing quoted policy-token envelope.
  - fixture_id: FX-SOIL-PRODUCER-OBLIGATION-CASES-001
    path: tests/fixtures/infile/soil/soilauth03_guard_cases.json
    fixture_class: integration
    units_basis: token_count_and_symbol_order
    hash: d0fefdc0b33e0385532e51feeaa2369c062730f247f600731aed815505298d70
    source_repo: /workdir/openWEPP
    source_commit: 42dcd326d326fef29c5141a3f41927e31a63adec
    source_path: tests/fixtures/infile/soil/soilauth03_guard_cases.json
    source_sha256: d0fefdc0b33e0385532e51feeaa2369c062730f247f600731aed815505298d70
    transform_note: Machine-readable case obligations for required datver, arity, and token-order checks.
tolerances:
  mode: abs
  abs:
    value: 0.0
    comparator: "=="
  units: contract_obligation_failures
  notes: Required producer-contract obligations must pass exactly; any drift is a blocking failure.
gate_lane: required
failure_class: hard-fail
runtime_cost_class: integration
owner: openWEPP maintainers
provenance:
  authored_by: Codex
  authored_utc: 2026-06-01
  last_updated_utc: 2026-06-01
notes: SOILAUTH03 suite guards required soil symbol/order/arity obligations and fixture hash/provenance integrity.
---

# cas_l4_infile_soil_producer_contract_001 Soil Input Producer-Contract Anti-Drift Guard Suite

## Purpose

Provide a release-blocking anti-drift gate for `.sol` producer-contract
obligations so required canonical symbols, row-order/arity envelopes, and
locked canonical fixture provenance do not silently regress.

## Authority Links

- Producer contract: `docs/specifications/wepp-input-files/specs/soil-file.spec.md`
- Parser contract: `docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md`
- Machine obligations: `docs/specifications/external-authority/required-suite-obligations.json`

## Expected Behavior

1. Required canonical soil symbols remain present in both producer and parser
   authority surfaces.
2. Canonical datver fixtures preserve required row-order/arity envelopes:
   - `9002` policy-first + quoted-header forms,
   - `7778` per-OFE restrictive-row forms,
   - quoted policy-token handling.
3. Fixture lock/provenance sidecars remain valid and tamper-detecting.

## Gate and Failure Semantics

- Lane: `required`
- Failure class: `hard-fail`
- Failure action: block release-gate acceptance when any required producer
  contract obligation or fixture integrity check regresses.
