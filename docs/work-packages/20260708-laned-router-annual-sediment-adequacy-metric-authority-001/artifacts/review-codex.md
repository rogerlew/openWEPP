# Review - Codex

Evidence mode: Static.

## Findings

### P1 - Handoff is premature while required gate/disposition evidence is incomplete

`package.md` requires `artifacts/gate-results.md` plus analyzer py-compile, replay command, `git diff --check`, Markdown/doc lint, and contract/profile/BEI checks before closure (`package.md:127`, `package.md:136`). That artifact is absent in the current artifact directory. The same package exit criteria require review/verification findings dispositioned and a worker handoff before `EXECUTED-COMPLETE-METRIC-AUTHORITY` (`package.md:161`), but `final-disposition.md` still says review/verification artifacts are pending and that any blocking finding must reopen the package (`final-disposition.md:16`). Before handing off renewed `dx5` ratification, add the missing gate evidence and update final disposition after review/verification are actually present.

## Metric Authority Checks

No finding on the rev-44 metric substance. The SC-OFEROUTE diff keeps production active mesh fixed at `10 cells/OFE`, keeps target-`dx` diagnostic/non-promotional, and records rev 44 as metric authority only (`docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md:127`, `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md:232`, `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md:578`). The analyzer applies one fixed material-year fraction (`0.05`) and the existing tolerance across all pass-sediment columns, with material-year, vector, dry-reference, and low-contribution reporting paths explicit (`artifacts/analyze_annual_sediment_metric.py:24`, `artifacts/analyze_annual_sediment_metric.py:25`, `artifacts/analyze_annual_sediment_metric.py:64`).

The replay preserves magnitude control and reporting: the WA `tdep:4` low-contribution excursion remains reported at `0.0221316838`, while material years pass (`0.00173788779`) and the annual-vector L1 relative value is `0.000612007475` (`artifacts/annual-sediment-metric-replay.md:41`; `artifacts/annual-sediment-metric-replay.json:12568`; `artifacts/annual-sediment-metric-replay.json:12641`). The predecessor attribution supports the scope of the metric change: the miss is one low-mass erosion day with identical pass water magnitudes/source mass and sub-threshold routed-shape movement, not active-router numerics or daily water-magnitude drift (`docs/work-packages/20260708-laned-router-wa-sediment-reference-adequacy-attribution-001/artifacts/wa-sediment-attribution.md:17`, `docs/work-packages/20260708-laned-router-wa-sediment-reference-adequacy-attribution-001/artifacts/wa-sediment-attribution.md:47`).

Residual risk: I did not rerun the analyzer or contract/doc gates. The next disposition update should attach those command logs rather than relying on the replay artifacts alone.
