# PL14S Contract Implementation Evidence

Status: `completed`
Evidence mode: `Static`

## Static
- Scope
  - Completed Phase A contract/tooling authority updates for PL14S Tier-A
    semantic-parity replay and reusable legacy comparison suite posture.
  - Retained explicit erosion exclusion for this lane.
- Canonical contract files amended
  - `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
  - `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  - `docs/specifications/science-contracts/index.md`
- Contract authority changes
  - `SC-SYSTEM-001`
    - Added `INV-SYSTEM-017` (PL14S semantic replay diagnostics completeness).
    - Added `OBL-SYSTEM-P-006` semantic diagnostic publication obligation.
    - Added semantic provenance guard/disposition row and
      `TOL-SYSTEM-007` semantic tolerance-profile authority.
    - Bumped `contract_version` from `14` to `15`.
  - `SC-WATBAL-001`
    - Added `INV-WATBAL-017` (WB13 semantic replay diagnostics invariant).
    - Added guard/disposition mapping for semantic report completeness.
    - Added `OBL-WATBAL-P-005` semantic evidence publication obligation.
    - Extended WB13 contract-test vectors with PL14S semantic evidence vector.
    - Bumped `contract_version` from `26` to `27`.
  - `science-contracts/index.md`
    - Updated `SC-SYSTEM-001` and `SC-WATBAL-001` registry notes to record
      PL14S semantic replay authority and preserved companion governance posture.
- Legacy comparison suite authority alignment
  - `tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py`
    - Added explicit report schema version (`pl14s-semantic-wat-v1`).
    - Added duplicate row-key hard-fail behavior for `.dat` and `.parquet`
      inputs (no silent overwrite).
    - Added structural diagnostics: baseline-only/candidate-only columns and
      investigation columns used/missing.
  - `tools/legacy_comparison_suite/run_pl14s_legacy_suite.py`
    - Added strict single-match guard for baseline `H*.wat.dat` discovery.
    - Added semantic report content validation before provenance emission.
    - Added provenance schema marker (`pl14s-legacy-suite-v1`) and semantic
      summary payload.
    - Preserved explicit strict-comparator required/skipped posture by
      candidate format.
  - `tools/legacy_comparison_suite/README.md`
    - Documented PL14S guard posture, report/provenance schemas, and parquet
      replay usage path.
- Phase sequencing note
  - This artifact captures Phase A authority/design implementation only
    (contracts and comparator tooling guard posture).
  - No kernel production-code edits were performed.

## Ran
- Phase A artifact scope does not include executable commands.
- Follow-on execution evidence is recorded in:
  - `pl14s-contract-test-implementation-evidence.md`
  - `pl14s-comparator-run-provenance-manifest.md`
  - `pl14s-implementation-and-test-evidence.md`
