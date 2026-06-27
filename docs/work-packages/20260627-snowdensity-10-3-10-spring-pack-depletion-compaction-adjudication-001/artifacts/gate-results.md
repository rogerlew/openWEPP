# Gate Results

Date: 2026-06-27

| Command | Result | Notes |
| --- | --- | --- |
| `.venv/bin/python tools/snowfreeze_observed/spring_pack_depletion_compaction_adjudication.py` | PASS | Generated JSON/Markdown cap-feasibility artifacts from the 10.3.8 coupled WAT report. |
| `cargo fmt --check` | PASS | Formatting check passed. |
| `cargo test --test snowdensity10_3_10_spring_pack_depletion_compaction` | PASS | 3 tests passed. |
| `cargo clippy --test snowdensity10_3_10_spring_pack_depletion_compaction -- -D warnings` | PASS | Focused clippy gate passed. |

## Execution Summary

- Candidate baseline: `coe_liquid_holding_capacity_v1`.
- Density cap authority: `SC-SNOWFREEZE-001` `522 kg m^-3`.
- March/April failures: `282`.
- Compaction-only feasible failures: `190`.
- Depletion-required failures: `49`.
- Recommended next process:
  `SPRING-COMPACTION-DENSIFICATION-CANDIDATE`.
