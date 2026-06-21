# R6H Worker Handoff

Status: complete.

## Stable Hold

`HOLD-R6H-WAT-PMET-LAYER-CARRY-ULP-PARITY`

R6H cleared `HOLD-R6G-WAT-PMET-DAY-STATE-CARRY-BUILDER-ABSENT` by replacing
the precomputed PMET day-input vector with an interleaved direct day/lane
builder. The remaining WAT residual is exactly `Es` on day 2:

- direct `Es=0.7677601843722605` mm;
- compatibility `Es=0.7677601843722608` mm;
- `Total-Soil` and `SoilWaterTotal` are bit-identical.

Follow-on package:
`docs/work-packages/20260621-r6i-direct-pmet-layer-ulp-parity-001/package.md`.

## First Actionable Item If Held

Run R6I. First localize the first direct-vs-compatibility bit divergence in
the direct ET layer-state carry feeding EVAPPM `wfevp`/`etkr`, then correct the
direct arithmetic/order-of-operations path with contract or pinned-baseline
authority. Do not relax WAT byte identity.

## Rejected Shortcut

Do not fill PMET `Es`, storage totals, WAT id, or lane-specific operands from
WB13 rows, compatibility runtime surfaces, writeback payloads, writer rows, or
output rows. Those values are parity comparators only after direct artifacts
are built.
