# Contract Implementation Evidence

Status: completed/HOLD
Evidence mode: static

Static:

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md` advanced to `contract_version: 14` with `INV-SNOWFREEZE-016`, requiring pre-day/post-day SWE, depth, density, settle-count, and delta evidence before assigning H1/H7/H39 spring residual ownership.
- `SC-SNOWFREEZE-001` guard map, alias map, boundary disposition, and revision history now include HPHYS0270 daily snowpack carry-state closure.
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` advanced to `contract_version: 98` with `INV-WATBAL-056`, requiring daily snowpack carry-state evidence before WB13 `RM`/`Snow-Water`, WB17 `Ep`, or aggregate-storage residual ownership is asserted.
- `SC-WATBAL-001` guard map, alias map, and revision history now include HPHYS0270 daily snowpack state diagnostics.
- `docs/specifications/science-contracts/index.md` records HPHYS0270 contract index summaries for `SC-SNOWFREEZE-001` and `SC-WATBAL-001`.

Ran:

- `bash tools/release/check_authority_suite_antievasion.sh` returned `0`.
- `cargo test --test auth11_required_suite_obligation_guards_contract` returned `0`.
