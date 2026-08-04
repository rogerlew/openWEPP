# Public API Parity

Status: `PASS / explicit unpublished-workspace migration reconciled`

Evidence mode: `Static: Cargo metadata and repository consumer search`

The owning orchestrator crate is `publish = false`; all discovered consumers
are repository-owned. The existing compute entry point remains and preserves
verbose diagnostics. The capture-aware entry point and compact ledger/outcome
types are additive exports; field consumers migrate mechanically within the
declared write set. User-facing CLI/runfile/output schemas remain protected.

Additive exports are `DirectSnowDiagnosticCapture`,
`DirectSnowSolidToLiquidLedger`, `DirectSnowLiquidDispositionLedger`,
`DirectSnowStage3Outcome`, and `DirectSnowVerboseDiagnostics`. The existing
verbose compute entry remains; the capture-aware entry is additive.

Repository-owned direct field consumers migrated from duplicated scalars and
the combined Stage-3 diagnostic to the new ledger/outcome/payload ownership.
The intentional Rust API/layout changes are:

- duplicated public transition scalars and eager diagnostic fields become the
  immutable `mass_transition_ledgers` bundle plus accessor methods;
- `DirectSnowCouplingInputs.mass_transition_ledgers` is
  `Box<DirectSnowMassTransitionLedgers>`, making
  `DirectSnowCouplingInputs::zero` non-`const`;
- `DirectDayFrame.snow_coupling_shadow_projection` is
  `Option<Box<DirectSnowCouplingShadowProjection>>` to preserve the retained
  live-frame size ceiling; and
- the established by-value
  `DirectSnowCouplingDownstreamOperands::from_state_and_hourly_routed_melt`
  signature remains unchanged.

All consumers migrated atomically and warnings-denied all-target checking
passes. The result carrier is `656 B`; constructor/live-frame sizes are
`4112/15552 B`, both within their retained ceilings. Because the crate is
`publish = false`, this is an explicitly reviewed workspace break rather than
an external released break. Real CLI, runfile, schema-v4, WAT, and HBP/PASS
surfaces remain protected pending the exact paired receipt recorded alongside
this artifact.
