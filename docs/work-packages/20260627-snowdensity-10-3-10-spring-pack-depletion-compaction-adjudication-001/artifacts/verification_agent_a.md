# Verification A

Evidence class: Ran

## Commands

- `.venv/bin/python tools/snowfreeze_observed/spring_pack_depletion_compaction_adjudication.py`
- `cargo fmt --check`
- `cargo test --test snowdensity10_3_10_spring_pack_depletion_compaction`
- `cargo clippy --test snowdensity10_3_10_spring_pack_depletion_compaction -- -D warnings`

## Result

All commands passed.

## Verified Counts

- March/April failures: `282`.
- Compaction-only feasible failures: `190`.
- Depletion-required failures: `49`.
- Under-persistence failures: `43`.
