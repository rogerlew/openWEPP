# Disposition

Status: executed-hold
Evidence mode: static + ran

## Outcome

HPHYS0291 is executed-hold.

The package closes the same-day snow publication lifecycle defect:

- canonical `SC-*` contracts require producer-owned same-day fluxes for
  `snow.post_winter_rain_m` and `snow.routed_melt_m`;
- runoff reconciliation publishes both fields into the flux surface;
- WB13 consumes both fields from the flux surface only;
- trace/localization records both lifecycle fields from the flux surface only;
- state/default masking for these WB13 publication terms is rejected by tests.

The package remains HOLD for semantic parity:

- full H1..H39 runtime: `39/39`;
- full H1..H39 semantic parity: `0/39`;
- residuals remain in `RM`, `Q`, `Snow-Water`, `Total-Soil`,
  `SoilWaterTotal`, `Ep`, `Dp`, and `latqcc`.

## Validation

Ran:

- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`
- `bash tools/release/check_authority_suite_antievasion.sh`
- `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture`

Final gate root:

- `/tmp/hphys0291_final_gates_post_review_20260605T023206Z`

All final gates passed with `rc=0`.

## Reviews

- Agent A: MEDIUM and LOW findings accepted and fixed.
- Agent B: HIGH and MEDIUM findings accepted and fixed; LOW noted.
- Verification A: metadata-only MEDIUM finding accepted and fixed.
- Verification B: PASS with no findings.

No undispositioned findings remain.

## Continuation Recommendation

Next package should diagnose baseline-authoritative snow/liquid partitioning
before WB13 publication:

- snowpack retention/release state entering runoff reconciliation;
- winter/contin `wmelt` and rain-on-snow routing into runoff versus same-pass
  infiltration;
- infiltration/runoff partition capacity during high-melt days;
- downstream storage coupling after producer fluxes and partitioning are proven
  coherent.

Do not reopen WB13 `RM` inference. WB13 should remain a fail-closed consumer of
explicit producer fluxes.
