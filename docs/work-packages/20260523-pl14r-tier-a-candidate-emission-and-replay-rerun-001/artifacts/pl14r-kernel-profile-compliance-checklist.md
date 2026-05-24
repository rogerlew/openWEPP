# PL14R Kernel Profile Compliance Checklist

Status: `complete`
Evidence mode: `Static + Ran`

## Procedure/Profile Compliance

1. Canonical `SC-*` authority updated: `met`
- Evidence: `SC-SYSTEM-001 v8`, `SC-WATBAL-001 v16`, registry-note updates in
  `science-contracts/index.md`.

2. Required schema sections for changed behavior are present and updated: `met`
- Evidence: invariant/guard/disposition updates for `INV-SYSTEM-014` and
  `INV-WATBAL-014` in both canonical contracts.

3. Contract-derived PL14R tests implemented before replay/harness production
   edits: `met`
- Evidence: `pl14r_tier_a_replay_rerun_contract` target and test file;
  replay/harness production source edits were not required.

4. Pre-implementation contract gate executed and recorded: `met`
- Evidence: `artifacts/pl14r-preimplementation-contract-gate.md`.

5. Typed failure posture for missing replay metadata/include surfaces/provenance
   is explicit and tested: `met`
- Evidence: `INV-SYSTEM-014`, `INV-WATBAL-014`, and PL14R tests for
  missing include surfaces and required-hash hold behavior.

6. Required replay include surfaces were both present in candidate lane:
   `not met`
- Evidence: `h5_plot_comparator.json` shows
  `raw.only_baseline_count=1`, `raw.only_baseline_examples=["H5.plot.dat"]`.
- Contract consequence: explicit `HOLD` retained per
  `INV-SYSTEM-014` / `INV-WATBAL-014`.

7. Required repository gates executed: `met`
- Evidence:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`
