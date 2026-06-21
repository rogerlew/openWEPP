# Implementation Test Evidence

Status: executed-hold.
Evidence mode: Static + Ran.

## Commands Run

Ran:

- `cargo fmt --check`
- `cargo test -p openwepp-runner r6_ -- --nocapture`
- `cargo test -p openwepp-runner --test r6_direct_publication_cutover_cli_contract -- --nocapture`
- `cargo clippy -p openwepp-runner --all-targets -- -D warnings`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`
- `markdown-doc lint --path docs/ROADMAP.md --path docs/work-packages/README.md --path docs/work-packages/20260621-r6-direct-publication-cutover-001/artifacts/worker-handoff.md --path docs/work-packages/20260621-r6b-direct-publication-parity-manifest-cutover-001 --format json`
- `git diff --check`
- Manual CLI cutover reproduction with `--direct-publication-frame-cutover`

## Manual CLI Result

Ran: the direct-publication cutover CLI returned exit status `1` and emitted:

```text
CLIHILL-E-011 runtime surface failure for direct_publication_cutover:
HS-SIMOUT-E-001 R6-DIRECT-PUBLICATION-PARITY
R6B-DIRECT-PUBLICATION-TYPED-OPERANDS-ABSENT HBP byte identity failed:
direct=1654 bytes compatibility=1654 bytes
```

The output directory had no public output files after the failure.

## Not Run

- direct frame population acceptance tests;
- anti-alias fixtures;
- independent reconstruction tests;
- manifest direct-provenance tests;
- H2637 default-disabled timing/protected-output comparison;
- direct-publication endpoint/RSS benchmark.

These are blocked by
`HOLD-R6B-DIRECT-PUBLICATION-TYPED-OPERAND-BRIDGE-ABSENT`.

## Focused Coverage Added

Ran: `r6b_absent_operand_detector_suppresses_marker_for_nonzero_direct_operands`
proves the diagnostic marker is suppressed when scalar, optional, or erosion
material exists in the direct publication frame.

## Final Closure Signal

Ran:

- `cargo fmt --check`: PASS.
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- `cargo test --workspace`: PASS, including 114 `openwepp-runner` lib tests
  and the new R6B negative detector test.
- `cargo deny check`: PASS.
- Scoped `markdown-doc lint`: PASS, 28 files scanned, 0 errors, 0 warnings.
- `git diff --check`: PASS.
