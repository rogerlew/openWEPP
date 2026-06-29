# H2637 Performance Evidence

Status: `HOLD-H2637-ENDPOINT-NOT-RUN`

Evidence class: Ran + Static.

The package implemented the opt-in Stage 3 diagnostic candidate but did not run
a real H2637 endpoint timing. Therefore no ADR-0025 promotion/performance claim
is made.

Ran performance-relevant guards:

- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test -p openwepp-hillslope-orchestrator r7b_constructor_type_size_layout_is_bounded -- --nocapture`:
  passed.
- `cargo test --workspace`: passed.

Direct-runtime size evidence from the focused size guard:

```text
DirectRunConstructorInputs=72
DirectLaneConstructorInputs=1024
DirectDayConstructorInputs=4016
DirectRunFrame=256
DirectLaneFrame=1136
DirectDayFrame=12248
```

The first implementation stored full Stage 3 diagnostics inline and tripped the
constructor size guard (`DirectDayConstructorInputs=4120`). The retained
implementation stores Stage 3 direct-runtime trace carry as
`Option<Box<DirectSnowStage3Diagnostics>>`, so disabled/default days remain
allocation-free and the hot day/frame layout is back under the existing bounds.

Disposition:

- H2637 endpoint performance remains a required gate before any Stage 3
  promotion or default activation.
- This package is closed as opt-in/non-promoted; missing H2637 endpoint timing is
  an explicit hold reason, not a hidden pass.
