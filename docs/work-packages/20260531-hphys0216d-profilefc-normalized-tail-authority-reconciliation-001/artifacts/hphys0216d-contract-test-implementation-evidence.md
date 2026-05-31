# HPHYS0216D Contract-Test Implementation Evidence

Status: completed
Evidence mode: Static + Ran

## Contract-derived tests updated
1. `tests/integration/hphys0202_profile_fc_wp_lineage_contract.rs`
   - verifies package/contract authority sections include HPHYS0216D.
   - verifies run-time `ProfileFCStore` equals layer aggregation + FC tail and
     reconciles with projected normalized-profile FC storage.
2. `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`
   - verifies `wb13_profile_fc_tail_mm` is emitted and equals normalized minus
     parser-layer FC aggregation.
3. `crates/openwepp-runner/src/hillslope/mod.rs` tests
   - missing `wb13_profile_fc_tail_mm` hard-fails WB13 publication.
   - FC publication uses layer+tail authority, not direct seed publication.

## Ran evidence
- `cargo test --test hphys0202_profile_fc_wp_lineage_contract` -> pass
- `cargo test -p openwepp-runner hphys0216 -- --nocapture` -> pass
- targeted runtime-input tail vector -> pass

## Guard posture
- No fallback path added.
- Missing/non-finite/negative tail symbols remain typed hard failures.
