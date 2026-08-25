# V12 bounded implementation evidence

Base: `2d34d1082ae0d6e324a33a2e2b9159f99f5f8ace`.

The exact eleven-file implementation adds a sealed generic evidence mode while
preserving the production parent and lower wrapper signatures. `NoEvidence`
uses unit state and unit provider state. `CaptureEvidence`, its vectors and the
parent capture wrapper exist only under crate `cfg(test)`. The same capture
state is forwarded through all real parent terminal-subslab attempts; the
wrapper returns `(Result<_, DirectSnowStage3V11AttachmentError>, CaptureState)`
without propagating the physical error with `?`.

The real parent test
`interior_terminal_event_capture_reproduces_below_carrier_domain` passed and
established the exact nested `BelowCarrierDomain` error, the rejected
1.875/0.9375/0.9375 pair, coarse energy bits `0x40949afbc1928120`, refined
energy bits `0x40942e218363bae1`, signed-difference bits
`0xc03b368f8bb18fc0` (displayed magnitude `27.2131278332233 J/m2`), a distinct
0.9375/0.46875/0.6 floor admission, equal exact provider counts before/after,
no observed provider support below 600 ms, zero terminal hydrology liquid,
zero WB14 terminal credit, zero surface terminal ingress, and unchanged parent,
consumer, clock and Stage-3 caller state.

Executed gates:

- focused lower capture test: PASS;
- real parent capture test: PASS (18.42 s);
- production `cargo check -p openwepp-hillslope-orchestrator`: PASS;
- test-library compile `cargo test -p openwepp-hillslope-orchestrator --lib --no-run`: PASS;
- `cargo fmt --all -- --check`: PASS.

The legacy event-and-remainder success-named test remains unfulfilled and was
not relabelled. Consequently no final v21/v11/v139/v6 review is opened here.
