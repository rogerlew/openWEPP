# Kernel Profile Compliance Checklist

Status: complete

Evidence mode: static+ran

Static:

- HPHYS0298 is kernel-affecting because it amends canonical snow/runoff/water
  balance contracts and adds a contract-derived guard test.
- No production kernel/runtime physics code was changed.
- Production closure remains `HOLD` because first divergence localizes to
  hourly snow/rain forcing before downstream WB13/WB17/WB18/WB19 consumers.

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
- [x] Unit-governance map completed for touched dimensional surfaces:
  `hrrain`, `hrsnow`, raw melt, routed melt, `RM`, `Q`, and aggregate storage
  surfaces are explicitly recorded in millimetres or documented publication
  units.
- [x] Test-vector obligations reflected in
  `tests/integration/hphys0298_paired_lineage_partition_contract.rs`.
- [x] Package remains `HOLD` because diagnostic source partition is complete
  but production hourly-forcing parity is not closed.

## Profile Disposition

HPHYS0298 satisfies the kernel-process profile for the diagnostic/contract
scope it executed. It does not satisfy production physics closure because no
baseline-authoritative winter hourly snow/rain forcing migration was applied.
