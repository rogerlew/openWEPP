# R6F Current Failure Reproduction

Status: complete.

## Inherited Failure

R6F inherited `HOLD-R6E-HBP-DIRECT-PROCESS-PARITY-MISMATCH`.

Focused reproduction at R6F start showed HBP payload parity failed because the
direct near-zero runoff publication did not emit the compatibility
`peakro`/`watdur` pair.

## Current Failure

After R6F correction, HBP byte identity passes on the current near-zero runoff
fixture. The current fail-closed cutover
marker is:

`HOLD-R6F-WAT-DIRECT-PROCESS-PRODUCER-AUTHORITY-GAP`

The CLI cutover candidate reaches the WAT gate and refuses to write public
outputs.

## Commands

| Command | Result | Evidence |
|---|---|---|
| `cargo test -p openwepp-runner r6f_cutover_candidate_hbp_identity_exposes_wat_producer_gap -- --nocapture` | Passed | Proves current-fixture HBP byte identity and reduced WAT field deltas. |
| `cargo test -p openwepp-runner r6f_cutover_candidate_reaches_hbp_identity_then_fails_wat_producer_authority -- --nocapture` | Passed | Proves stable R6F WAT hold marker under runner cutover. |
| `cargo test -p openwepp-runner --test r6_direct_publication_cutover_cli_contract -- --nocapture` | Passed | Proves CLI fail-closed behavior and no public output writes. |

## Reduced WAT Fields

Direct and compatibility WAT rows now agree on first-row `P`, `RM`, `Q`, and
`QOFE`. They differ on:

- `wepp_id`
- output simulation `year`
- `Es`
- `Total-Soil`
- `SoilWaterTotal`
- `ProfileDepth`
- `ProfilePorosityCap`
- `ProfileFCStore`
- `ProfileWPStore`

The remaining fields require production typed direct process inputs for ET,
storage, and profile projection. R6F added the direct-runtime slots and carry
state; R6G must add the production parsed-input producer.
