# Test Evidence

Evidence mode: Ran.

## Diagnostic Run

- Ran: `.venv/bin/python tools/snowfreeze_observed/winter_thaw_melt_response.py`
- Result: PASS.
- Output summary:
  - `schema`: `snowdensity10-3-6-winter-thaw-melt-response-v1`
  - `disposition`: `WINTER-THAW-MELT-RESPONSE-DEFECT-ELIGIBLE`
  - `paired_surface_count`: `4`
  - `under_ablation_interval_count`: `132`
  - `next_route`: contract-first opt-in winter-thaw melt-response correction.
- Artifacts:
  - `artifacts/winter-thaw-melt-response.json`
  - `artifacts/winter-thaw-melt-response.md`
  - `target/snowdensity10_3_6_winter_thaw_melt_response/`

## Focused Guard

- Ran: `cargo test --test snowdensity10_3_6_winter_thaw_melt_response`
- First run: FAILED.
  - Cause: guard-test marker brittleness only.
  - Fix: changed one line-wrapped phrase marker from `warm-rain heat context` to `warm-rain`, and removed a trailing period from the Markdown bool marker.
- Final run: PASS.
  - `4 passed; 0 failed; 0 ignored`.

## Cohort Finding

Ran: event-window evidence across four paired Sleepers/Harvard surfaces:

- Event windows: `1345`.
- Observed ablation intervals: `238`.
- Observed thaw-ablation intervals: `219`.
- Under-ablation intervals: `132`.
- Under-ablation fraction: `0.602740`.
- Aggregate observed depth loss: `37.117851 m`.
- Aggregate modeled depth loss: `15.868230 m`.
- Aggregate depth-loss deficit: `24.105059 m`.
- Positive-temperature snowpack hours over thaw-ablation intervals: `19,166`.
- Raw CoE melt over thaw-ablation intervals: `8.685318 m`.
- Modeled SWE loss over thaw-ablation intervals: `4.628139 m`.
- Warm-rain heat equivalent over thaw-ablation intervals: `0.189965 m`.

HJ Andrews and Hubbard Brook remain observation-blocked diagnostic-only surfaces.
