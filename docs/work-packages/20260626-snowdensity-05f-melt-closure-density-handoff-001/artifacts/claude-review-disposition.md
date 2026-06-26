# Claude Review Disposition

Evidence class: Static + Ran.

Input review: `artifacts/claude-review.md`.

## Disposition

- F1 accepted: 05E diagnostic replay used `cancov = 0.0`; this is now recorded
  as a regime-limited evidence caveat. Operator clarification pins the
  validation management to coniferous forest with winter `cancov` about `0.9`,
  so the replay is known non-representative.
- F2 accepted: 05E diagnostic replay used PySnobal-bridge radiation; this is
  now recorded as a regime-limited evidence caveat.
- F3 accepted: the 05F residual-risk register now names the causes, not only the
  symptom.
- F4 accepted: SNOWDENSITY-06 now has an entry gate requiring live per-day
  canopy cover and native/proven shortwave before rubric evidence carries a
  density or activation verdict.
- F5 closed: local Brock-2000 constant verification confirms the constants used
  by `08_snow_albedo.rs`.

## Verification Notes

- Ran source scan confirmed `DEFAULT_CANOPY_COVER_FRACTION = 0.0` in
  `snowbench_coe_melt.rs`.
- Ran source scan confirmed 05E uses PySnobal-bridge `net_solar` reconstruction.
- Ran `pdftotext references/copyrighted/brock2000.pdf - | rg ...` and confirmed
  the Brock equations/limits behind the code constants.

## Outcome

No revert. The 05F interface freeze remains valid, but its acceptance evidence
is explicitly regime-limited until the SNOWDENSITY-06 entry gate fixes or proves
the harness regime.
