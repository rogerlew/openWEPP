# Symbol-Keyed Survivor Edges

Evidence mode: Static.

This artifact records any symbol-keyed type that remains after deletion because
it is a real intake/output serialization edge rather than an executable
alternate runtime.

## Realized Survivors At Hold

The public compatibility runtime selector is removed, but the symbol-keyed
support surface remains compiled:

- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs` and
  `day_frame.rs` remain present.
- `HillslopeWritebackSurface`, `HillslopeKernelRequest`,
  `KernelWritebackPayload`, and `SymbolRegistry` remain exported by the
  orchestrator/kernel-contract crates.
- Runner support modules still compile scheduler lifecycle, WB13 scheduler
  publication, HPHYS trace, symbol-registry audit, indexed-shadow, and legacy
  scheduler tests.

The attempted deletion of the unreachable scheduler execution branch showed that
these are not independent references: removing the branch makes the support
surface dead as a unit and produces hundreds of warnings before the clippy gate.
The next deletion package should remove or replace that whole support boundary:

- delete legacy scheduler tests that only validate the removed runtime;
- preserve only direct-native typed publication/seed tests;
- move any genuinely still-needed symbol-keyed serialization helpers out of the
  executable scheduler namespace and document them as I/O adapters;
- then delete `scheduler.rs`, `day_frame.rs`, and carrier exports that no longer
  have a real I/O-adapter role.
