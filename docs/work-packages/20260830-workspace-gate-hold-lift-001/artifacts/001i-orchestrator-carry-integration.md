# WGHL-FULL-001I orchestrator exact-carry receiver integration

Status: receiver/owner handoff complete; real DirectV10 installation and
persisted-restart endpoint remain the next authorized slice.

Evidence mode: `Static + Ran`

## Implementation

Static: the successor-only soil-thermal V2 path now reconstructs accepted
soil-internal storage deltas and infiltration energy directly from the physical
LSE candidate and typed surface-liquid receipts. Separately sealed
top-boundary credits are joined in canonical OFE/layer/kind/ordinal order. Each
finite binary64 operand is passed to the exact-dyadic LSE core without
tolerance, `nextafter`, forced ULP, producer residual, or intermediate scalar
collapse.

Static: `SoilThermalExpectedAcceptedOperandSetV2` is an independent expected
source set created before the soil credit receipt. Its canonical digest binds
the accepted operands and authoritative temperature/heat-capacity projections.
Receipt replay therefore cannot validate against flattened operands supplied
by the receipt under test. Exact infiltration cancellation is reconstructed
independently from surface receipts and requires `Q_surface + (-Q_soil) = 0`
in `ExactDyadicEnthalpy`.

Static: `SoilThermalAcceptedCandidateV2` remains clone-only. Restart,
checkpoint, latest-credit, expected-source, and aggregate orchestrator seals
are constructed and independently replayed before canonical V2 bundle bytes
can be emitted. A poisoned nested receipt, expected-source digest, native seal,
or aggregate seal fails closed. No V2-to-V1 downgrade exists. The only V1-to-V2
entry is the checked core migration with zero carry; focused evidence confirms
the source V1 bytes remain unchanged.

Static: outer LSE V1 and authoritative LSE V2 configuration identities use
their respective complete validators. Any other outer identity is refused.
Beginning/ending owner identity, transaction/predecessor/support, OFE/tile/
layer topology, units, basis, operand order/cardinality, debit-credit digest,
exact high/carry, projected temperature, receipt chain, and restart/checkpoint
joins remain bound.

Static: this slice deliberately does not install the V2 owner into
`DirectV10RealConsumerShadow`, does not change the persisted-restart crate, and
does not claim the real WAT5 endpoint. Because the receiver result is an
unpublished clone, every receiver or sealing failure leaves the beginning V2
owner byte-identical. Atomic complete-owner installation and injected failures
after enclosing closure/finalization belong to the next cutover slice.

## Focused and authority evidence

Ran:

```text
nix develop -c cargo nextest run -p openwepp-hillslope-orchestrator \
  -E 'test(soil_thermal_exact_carry_v2)' --no-fail-fast
```

- terminal run: `614adccd-5a75-43ab-9209-530ea227e84f`
- result: `PASS`, 6/6; 1,159 skipped
- covers the canonical WAT5 sub-ULP operand, exact retained carry, native and
  orchestrator seals, canonical bundle validation, multi-operand exact
  cancellation, reorder/substitution poison, top-boundary support/identity,
  source V1 byte freeze, authoritative LSE V2 admission, unauthorized identity
  refusal, and byte-identical beginning-owner rollback

Ran:

```text
nix develop -c cargo nextest run \
  --test surface_liquid_hydrology_custody_authority_contract \
  --test land_surface_energy_balance_authority_contract \
  -E 'test(/^version_(15|fifteen)_/)' --no-fail-fast
```

- run: `34bfd4ff-109e-4899-8859-de81090cd876`
- result: `PASS`, 4/4; 25 skipped
- covers both canonical contract-version-15 source and production-symbol
  obligations

Ran: `nix develop -c cargo check -p openwepp-hillslope-orchestrator --tests`
passed after the terminal source changes.

## Affected crate and source-quality disposition

Ran: the full orchestrator crate completed 1,162 tests in run
`d5e0ffec-a02c-45bc-a72f-a1b26e9631ee`: 1,157 passed, five failed, and three
were skipped. None of the exact-carry tests failed. Four failures were in the
concurrently changed covered-vapor adaptive path and reported typed
`covered vapor active-set projection` structure/coordinate refusals. The fifth
was an unrelated stack overflow in
`latest_accepted_stage3_state_changes_next_wb14_proposal`. This is retained as
shared-head `HOLD`, not represented as an exact-carry pass.

Ran: warnings-denied all-target/all-feature and lib-only Clippy were attempted.
They are not terminally clean: the shared crate currently reports hundreds of
pre-existing/concurrent diagnostics. The initially observed 001I-local
unreadable/excessive-precision literals, items-after-statements, and
too-many-lines diagnostics were corrected. A terminal warnings-denied scan of
the owned production paths reports only inherited diagnostics before the 001I
regions in `canonical_owner_bytes.rs` and `real_hydrology_execution.rs`; no
001I-added diagnostic remains. This is a truthful parent-package `HOLD`, not a
Clippy pass.

Ran: individual rustfmt, `git diff --check` on all owned changed paths, and a
diff-only scan for `nextafter`, tolerance, forced ULP, microstep, and diagnostic
additions passed. No production diagnostic surface or persisted diagnostic was
added.

Final line-count disposition:

| File | Lines | Disposition |
|---|---:|---|
| `v9_real_consumer_shadow_soil_thermal.rs` | 1,054 | pass |
| `v9_real_consumer_shadow.rs` | 2,944 | existing `WARN`; below hard limit |
| `canonical_owner_bytes.rs` | 2,316 | existing `WARN`; below hard limit |
| `land_surface_energy_shadow/mod.rs` | 1,267 | pass |
| `land_surface_energy_shadow/real_hydrology_execution.rs` | 2,004 | `WARN`; exact physical operand helpers add four lines beyond threshold and should move to the DirectV10 cutover shard before further growth |
| `land_surface_energy_shadow/receiver_validation.rs` | 2,480 | existing `WARN`; below hard limit |
| `soil_thermal_exact_carry_v2_tests.rs` | 313 | pass |

Terminal owned-source SHA-256 identities:

```text
76af9cf10aa477a1c18467391cbef8db266f2c931f5550232892334896fcd44f  v9_real_consumer_shadow_soil_thermal.rs
c30891b127707fe5fd7931c01c925fc3fcdc6c332442b5d46d6ba3b8f465803c  v9_real_consumer_shadow.rs
0860e3a1c40c828bca27bf5b3cee7458b80f233acd3f636f3c594e2d1dafff9f  canonical_owner_bytes.rs
4b1f197065e423f8e27b3ba6dc9851f4f8fcd733b4657734b16c499370933fc6  land_surface_energy_shadow/mod.rs
d5fabfd3c7aee472055ac5904530447237a60d38065213052eee934940239d1d  land_surface_energy_shadow/real_hydrology_execution.rs
870d6cd9c4142628e2e07f04ce923a9e473ba20803247d084680dc09b90a0ea6  land_surface_energy_shadow/receiver_validation.rs
d0331d0d8b6014720c29fe375260e39d6512a37b38350eeb1466c413c206869d  soil_thermal_exact_carry_v2_tests.rs
```

## Handoff symbols

- `SoilThermalExpectedAcceptedOperandSetV2::try_new` / `validate`
- `SoilThermalAcceptedCandidateV2`
- `aggregate_soil_thermal_ending_v2`
- `aggregate_soil_thermal_physical_ending_v2`
- `soil_thermal_top_boundary_operands_v2`
- `validate_soil_thermal_v2_surface_cancellation`
- `seal_soil_thermal_accepted_candidate_v2`
- `validate_soil_thermal_orchestrator_seals_v2`
- `canonical_soil_thermal_v2_bundle_bytes(beginning, candidate, seals)`
