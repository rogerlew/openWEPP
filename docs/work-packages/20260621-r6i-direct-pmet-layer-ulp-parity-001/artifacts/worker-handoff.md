# R6I Worker Handoff

Evidence class: Static plus Ran.

## Closed

`HOLD-R6H-WAT-PMET-LAYER-CARRY-ULP-PARITY` is closed.

R6I added typed direct frost layer carry projection and proved current-fixture
HBP/WAT identity. The R6G and R6H WAT hold markers no longer fire.

## Remaining R6 Blocker

The next blocker is manifest publication cutover:

`manifest direct projection is not wired to the production manifest writer`

Current CLI cutover behavior:

- `DirectPublicationFrameCutover` still fails closed.
- No public output files are written on the failure path.
- HBP byte identity and WAT identity already pass before the manifest blocker.

## First Actionable Follow-Up

Close the manifest writer cutover blocker by wiring the direct manifest
projection into the production manifest writer with parity proof.

Required follow-up gates:

- Manifest provenance/checksum parity between direct projection and production
  manifest writer.
- No compatibility manifest wrapping as direct authority.
- Preserve HBP and WAT identity gates added by R6I.
- Preserve CLI fail-closed behavior until manifest and remaining PASS/loss
  publication gates are proven.
- Re-run `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, and `cargo deny check`.
