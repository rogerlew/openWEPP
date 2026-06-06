# Kernel Profile Compliance Checklist

Status: complete

Evidence mode: static+ran

Static:

- HPHYS0298 is kernel-affecting because it amends canonical snow/runoff/water
  balance contracts and adds a contract-derived guard test.
- No production kernel/runtime physics code was changed.
- Production closure remains `HOLD` because the historical HPHYS0298
  `hrsnow` verdict is superseded by HPHYS0299 corrected depth-vs-depth
  authority. HPHYS0298 alone does not authorize winter-forcing migration or
  downstream WB13/WB17/WB18/WB19 compensation.

Ran:

- `cargo test --test hphys0298_paired_lineage_partition_contract -- --nocapture`
- `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture`
- `bash tools/release/check_authority_suite_antievasion.sh`

## Checklist

- [x] Canonical `SC-*` files updated:
  `SC-SNOWFREEZE-001`, `SC-RUNOFFPART-001`, and `SC-WATBAL-001`.
- [x] Required schema sections remain present in touched contracts.
- [x] Algorithm steps and branch tables updated for touched diagnostic
  cut-points.
- [x] Guard/error mapping updated for missing paired-lineage trace surfaces,
  source provenance, observe identity, and downstream-compensation prohibition.
- [x] Unit-governance map completed for touched dimensional surfaces, with a
  retrospective correction that HPHYS0298 `hrsnow` paired baseline snowfall
  depth with openWEPP water-equivalent output and is therefore
  non-authoritative for migration.
- [x] Test-vector obligations reflected in
  `tests/integration/hphys0298_paired_lineage_partition_contract.rs`.
- [x] Package remains `HOLD` because corrected unit/provenance authority starts
  in HPHYS0299, not the historical HPHYS0298 ledger.

## Profile Disposition

HPHYS0298 satisfies the kernel-process profile for the diagnostic/contract
scope it executed, with the retrospective caveat that its original all-window
`hourly-forcing` migration verdict is superseded. It does not satisfy
production physics closure because its `hrsnow` evidence was a
depth-vs-water-equivalent comparator artifact.
