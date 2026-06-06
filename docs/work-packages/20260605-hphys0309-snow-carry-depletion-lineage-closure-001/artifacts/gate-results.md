# Gate Results

Status: complete

Evidence mode: ran

Static:

- Broad validation was run after diagnostic generation and contract-test
  repairs.

Ran:

- `cargo fmt --check` passed.
- `bash tools/release/check_authority_suite_antievasion.sh` passed.
- `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture`
  passed.
- `cargo test --test hphys0308_snowd_branch_state_ordering_contract -- --nocapture`
  passed after version-pin test repair.
- `cargo test --test hphys0309_snow_carry_depletion_lineage_contract -- --nocapture`
  passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo test --workspace` passed.
- `cargo deny check` passed with existing duplicate/unmatched-license warnings:
  `getrandom`, `hashbrown`, `twox-hash`, `ISC`, and `Unicode-DFS-2016`.
- `git diff --check` passed.
- HPHYS0309 artifact bytecode check passed with no `__pycache__` or `.pyc`
  files present.

Post-review rerun:

- `python -m py_compile docs/work-packages/20260605-hphys0309-snow-carry-depletion-lineage-closure-001/artifacts/hphys0309_snow_carry_depletion_lineage.py`
  passed.
- `python docs/work-packages/20260605-hphys0309-snow-carry-depletion-lineage-closure-001/artifacts/hphys0309_snow_carry_depletion_lineage.py`
  regenerated HPHYS0309 artifacts.
- `cargo fmt --check` passed.
- `cargo test --test hphys0308_snowd_branch_state_ordering_contract -- --nocapture`
  passed.
- `cargo test --test hphys0309_snow_carry_depletion_lineage_contract -- --nocapture`
  passed.
- `jq` confirmed `58` rows, route counts `45/13`, lead-state counts `56/2`,
  and no null `openwepp_key_depth_after_m` values.

Final closeout rerun:

- `python -m py_compile docs/work-packages/20260605-hphys0309-snow-carry-depletion-lineage-closure-001/artifacts/hphys0309_snow_carry_depletion_lineage.py`
  passed.
- `python docs/work-packages/20260605-hphys0309-snow-carry-depletion-lineage-closure-001/artifacts/hphys0309_snow_carry_depletion_lineage.py`
  regenerated HPHYS0309 artifacts.
- `cargo fmt --check` passed.
- `bash tools/release/check_authority_suite_antievasion.sh` passed.
- `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture`
  passed.
- `cargo test --test hphys0308_snowd_branch_state_ordering_contract -- --nocapture`
  passed.
- `cargo test --test hphys0309_snow_carry_depletion_lineage_contract -- --nocapture`
  passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo test --workspace` passed.
- `cargo deny check` passed with existing duplicate/unmatched-license warnings:
  `getrandom`, `hashbrown`, `twox-hash`, `ISC`, and `Unicode-DFS-2016`.
- `git diff --check` passed.
- Artifact hygiene checks found no package cache/bytecode files and no stale
  queued/not-run placeholders or transient `/tmp` fixed-comparator authority.
- `jq` confirmed `58` rows, route counts `45/13`, lead-state counts `56/2`,
  and `0` authorized production edits.
