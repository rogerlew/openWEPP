# Rust-to-frozen V10 leaf-gas parity

Status: `PASS / review pending`

The ordinary Rust unit test
`actual_v10_leaf_gas_matches_frozen_nighttime_vectors` reads the committed
`nighttime-ci-vectors.json` and executes the actual LSE-V2/V10
`leaf_trial_state` path for `+0.0`, `-0.0`, `0.1`, `1.0`, and `50.0` absorbed
PAR. It checks exact categorical branches (`ExactZeroPar`,
`RespirationDominated`, `PositiveAssimilation`) and compares `Ci`, `Ag`, `An`,
`Rd`, and `rs` under the existing `1e-12 * max(1, abs(reference))`
representation comparison. A separate zero-area call checks `Inactive`.

Ran:

```text
OPENWEPP_TASK_ID=v10-restart-phase1 nix develop -c cargo test \
  -p openwepp-land-surface-energy \
  actual_v10_leaf_gas_matches_frozen_nighttime_vectors -- --nocapture
```

Result: PASS, 1/1. The preceding ambient-shell attempt failed before
compilation with `cargo: command not found` (exit 127); it was rerun in the
repository's pinned Nix environment without changing the test.

Ran:

```text
OPENWEPP_TASK_ID=v10-restart-phase1 nix develop -c cargo nextest run \
  --test v10_nighttime_authority_contract --profile quick --no-fail-fast
```

Result: PASS, 3/3, Nextest run
`4d959066-609e-47d7-90e8-68129a799be5`. The independent Python regeneration
test remains separate from the direct Rust path.
