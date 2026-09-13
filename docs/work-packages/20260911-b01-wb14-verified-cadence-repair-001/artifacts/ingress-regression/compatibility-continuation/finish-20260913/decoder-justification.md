# Captured parent decoder justification

Static: the context test uses the canonical parent-restart decoder,
`DirectWb14ParentWorkingState::from_restart_bytes`, at
`crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_ingress_context_tests.rs:110-114`.
It is available to this crate-local test because its declaration is
`pub(crate)` in `surface_liquid_ingress_coordinator.rs:723-730`. That decoder
parses the canonical JSON restart form and then calls
`validate_nested(configuration)`; the same encoder validates before serializing
at lines 714-720.

Static: this test does duplicate the limited configuration decoding for the
immutable owner-seed. `DirectSurfaceLiquidConfigurationRestartV1::restore` at
`crates/openwepp-persisted-restart-v1/src/surface_liquid.rs:212-245` would
otherwise provide that construction and digest check. The orchestrator
`Cargo.toml` has no dependency on `openwepp-persisted-restart-v1`, while the
persisted-restart crate imports the orchestrator at
`surface_liquid.rs:2-7`. Reuse therefore requires an out-of-scope manifest or
dependency change, both expressly forbidden by this checkpoint.

Static: the duplicated configuration decoding is confined to the two immutable
fixtures. The test deserializes the packet's typed beginning state and ingress
input with their respective `Deserialize` implementations at context-test
lines 97-105. `b01_hex_f64` at lines 25-29 uses `u64::from_str_radix` and
`f64::from_bits`, preserving the recorded binary64 values without decimal
conversion. The test builds the configuration with
`DirectSurfaceLiquidConfiguration::new` at lines 82-90 and checks its digest
against the captured beginning state at lines 154-157. The immutable owner-seed and
packet hashes remain
`d1c467265dc5c718cc6d1a18fd1fbf71289e3ad5760967f9556f33109283f515` and
`d8f1fe07d73ac09a6eb002b1a49a714cad7a0af1bb58ffab7612282ea002cc76`.
