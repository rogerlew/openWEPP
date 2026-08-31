# 001I V11 covered DirectSoilThermalResident V2 migration

## Implementation

Static: the covered carrier now branches on the sole resident soil owner.
The V1 arm retains the prior `SoilThermalSnapshot` acceptance and digest path.
The V2 arm reads the native typed view, prepares the exact support, reconstructs
canonical physical and top-boundary operands, and advances an unpublished
`SoilThermalTrialStateV2`. It does not issue or install an accepted receipt.

Static: accepted finalization reconstructs the complete ordered child operand
set against the authenticated V2 beginning, calls the accepted exact-carry
receiver once, seals once, and installs once. Child supports must form the
complete contiguous parent partition. Candidate ordinals are reconstructed in
child/support order before canonical topology validation. A failure before the
atomic install leaves the resident beginning unchanged.

Static: the terminal snow--soil ledger accepts a typed V1/V2 layer view. V1
uses the unchanged binary64 arithmetic, JSON candidate seal, and V1 receipt
digest. V2 sums high term, exact carry, and the terminal heat credit exactly,
uses the authoritative exact temperature projection, and binds owner, state,
transaction, layer, high term, and carry into the candidate identity. The
accepted endpoint serializes native V2 OFE state into its installed receipt
join; no V2-to-V1 snapshot, projection, cache, or downgrade is present.

No public API, persisted diagnostic, constitutive equation, fixed-point solve,
60-second floor, tolerance, or V1 receipt schema was changed by this shard.

## Compile and focused evidence

Pre-implementation `cargo check` exposed 16 resident field/type errors in the
assigned carrier/finalization paths. After the typed migration:

```text
nix develop -c cargo check -p openwepp-hillslope-orchestrator
```

- result at the then-stable shared source: `PASS`
- log: `/tmp/wghl-v2-covered-cargo-check-final2.log`

```text
nix develop -c cargo nextest run -p openwepp-hillslope-orchestrator \
  terminal_bottom_soil_trial --no-capture
```

- run: `687e085f-14eb-42aa-9879-dc1ae6dcf256`
- result: `PASS`, 2/2; 1,177 skipped
- covers unchanged V1 Crank--Nicolson closure and fail-closed scalar-volume
  poison
- log: `/tmp/wghl-v2-covered-focused-ledger-2.log`

```text
nix develop -c cargo nextest run -p openwepp-hillslope-orchestrator \
  terminal_bottom_soil_v2_trial_retains_exact_carry_and_owner_identity \
  --no-capture
```

- run: `48cb6077-8fe6-419c-b8b0-c4503b12c026`
- result: `PASS`, 1/1; 1,178 skipped
- covers nonzero exact carry, current transaction lineage, V2 owner/state
  binding, candidate seal divergence under identity poison, and receipt replay
- log: `/tmp/wghl-v2-covered-focused-ledger-v2.log`

The shared DirectV10 V2 suite also passed its four resident, receipt-free,
next-support carry, and poisoned-install rollback tests during combined run
`68eb4eb0-abf4-4b68-ba9b-9210887b7953`; that combined run was not terminal
because a newly added source-shape assertion was corrected afterward.

## Exact-head validation status

```text
nix develop -c cargo check -p openwepp-hillslope-orchestrator --lib
```

- exact-head result: `PASS`
- log: `/tmp/wghl-v2-covered-cargo-check-exact-head-final.log`
- SHA-256: `aef532c10fca0857ec9ed6ea4273c3a0510cd6ea526ffb3858a7f0ad2e9c2033`
- diagnostics from the assigned paths: none; warnings are from concurrent V33
  and retained fixed-point work

```text
nix develop -c cargo nextest run -p openwepp-hillslope-orchestrator \
  -E 'test(/v10_soil_thermal_v2_tests/) | test(/terminal_bottom_soil/) | \
      test(v2_carrier_composition_is_trial_only_and_receipt_free) | \
      test(v2_finalization_has_one_accepted_receiver_and_one_install)' \
  --no-capture --no-fail-fast
```

- run: `263bf305-240b-4dd6-93f4-173184c20371`
- result: `PASS`, 9/9; 1,169 skipped
- covers single native resident, exact carry into the next support, poisoned
  atomic-install rollback, receipt-free child composition, exactly one accepted
  receiver/install, unchanged V1 terminal closure and poison, and V2 terminal
  exact-carry/owner-state identity binding
- log: `/tmp/wghl-v2-covered-focused-exact-head-final2.log`
- SHA-256: `8105a8b5c71e1fcedf93d356cb4b24261517a111ef04f2cab7d554227b9f02cf`

```text
nix develop -c cargo nextest run -p openwepp-hillslope-orchestrator \
  -E 'test(/v11_covered::covered_carrier_phase_tests::/) | \
      test(/v11_covered::owner_finalization::terminal_custody_lane_set_tests::/)' \
  --no-capture --no-fail-fast
```

- run: `cebbf8ce-0874-481d-bdde-65f43221da34`
- result: `PASS`, 12/12; 1,166 skipped
- covers the broader unchanged carrier immutability, trial identity, lane-set,
  terminal receipt, and finalization custody shard
- log: `/tmp/wghl-v2-covered-broader-touched-shards.log`
- SHA-256: `ef0ab1e8743524386be1cc49252fc6abb407c98daadbe40e214a0d95750a1a6d`

`rustfmt --edition 2024` completed on all four assigned Rust paths.
`git diff --check` passed for the assigned paths and this evidence. No
production diagnostics or new public API were introduced.

## Line-count posture

At this evidence point:

| File | Lines | Disposition |
|---|---:|---|
| `v11_covered/carrier_phase.rs` | 1,323 | warn, below hard limit |
| `v11_covered/owner_finalization.rs` | 2,703 | warn, below hard limit |
| `v11_covered/physical_outcome_ledger.rs` | 1,364 | warn, below hard limit |
| `v11_covered/open_snow_terminal_accepted_endpoint.rs` | 741 | pass |

These are existing tightly coupled included shards. A mechanical split during
the resident cutover would mix file movement with receipt-chronology changes;
decomposition remains parent-package follow-up.
