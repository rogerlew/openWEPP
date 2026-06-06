# Complete Balance Identity Audit

Status: queued

Evidence mode: not-run

Not-run:

Before attributing the emitted-ledger residual, audit the complete identity:

- precipitation and irrigation inputs;
- `Q`, `Ep`, `Es`, `Er`, `Dp`, `latqcc`, and `Tile` outputs when populated;
- `Total-Soil`, `SoilWaterTotal`, `Snow-Water`, and populated interception
  storage deltas;
- run-on/run-in terms such as `UpStrmQ` and `SubRIn`;
- confirmation that `SoilWaterTotal` already includes `frozwt`.

If the residual collapses under the complete identity, close as validated
non-defect rather than relaying attribution.
