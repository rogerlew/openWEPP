# Verification A

Evidence mode: Static/Ran.

- Static: contract v94 includes `INV-SNOWFREEZE-066`, `OBL-SNOWFREEZE-P-041`,
  and the 10.3.7 addendum.
- Ran: `.venv/bin/python tools/snowfreeze_observed/winter_thaw_melt_response_correction.py`
  produced `WINTER-THAW-MELT-RESPONSE-CANDIDATE-IMPROVES`.
- Ran: `.venv/bin/python tools/snowfreeze_observed/winter_thaw_melt_response_correction.py --skip-runs`
  regenerated the report after the active-ledger conservation summarizer fix.
- Ran: `.venv/bin/python tools/snowfreeze_observed/winter_thaw_melt_response_coupled_gate.py`
  produced `WINTER-THAW-COUPLED-WAT-IMPROVES`.
- Ran: `cargo test --test snowdensity10_3_7_winter_thaw_melt_response_correction -- --nocapture`
  passed.

Verification result: PASS for package-specific correction and diagnostic gates.
