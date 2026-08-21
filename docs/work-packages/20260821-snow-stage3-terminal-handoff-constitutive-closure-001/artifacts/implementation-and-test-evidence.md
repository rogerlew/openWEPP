# Implementation and test evidence

Status: `PARTIAL INCREMENT PASS / CLOSURE BLOCKED`.

Implemented seams:

- exact typed Stage-3 support duration extraction and terminal candidate rerun;
- typed attachment with cloned-candidate installation boundary;
- historical runner state renamed and isolated from the new constitutive
  attachment;
- typed owner-byte projection and narrow coupled-time accessors;
- receiver topology validation and independent terminal-liquid arithmetic;
- default-off scheduler source boundary.

`Ran:` current canonical and package-focused commands:

```text
nix develop --command cargo nextest run --test \
  land_surface_energy_balance_authority_contract \
  surface_liquid_hydrology_custody_authority_contract \
  snow_stage3_legacy_predecessor_bridge_contract \
  snow_stage3_persistent_accumulation_shadow_contract \
  snow_stage3_terminal_receiver_authority_contract \
  snow_stage3_turbulent_operator_reconciliation_contract \
  snow_stage3_wind_source_custody_contract
```

Initial run: `56 passed, 0 skipped`, run ID
`a1aba459-59c8-494a-892f-e4076d7c04b0`. Terminal repeated-binary run:
`48 passed, 0 skipped`, run ID
`265df81e-bf4c-4a00-a096-a7af76d66bab`.

```text
nix develop --command cargo nextest run -p openwepp-hillslope-orchestrator --lib \
  snow_stage3_shadow persistent_support_evaluator_runs_one_admitted_parent_support \
  terminal_event_request_is_state_bound_and_censors_remaining_time
```

`5 passed`, terminal run ID `809ae6c9-d9e6-449b-a560-4f2dc849f2e3`
(prior run ID `75d4629a-e634-4e71-bb3f-7d0bd05c5857`).

```text
nix develop --command cargo nextest run --test \
  snow_stage3_v11_constitutive_boundary_contract
```

`3 passed, 0 skipped`, terminal run ID
`9da6ed44-a523-47b9-8f21-e7ce9697a1a6`.

The Stage-3 observability guard passed `6/6` (run ID
`94c49315-3da1-4d49-a28c-995759ddc323`), and the current SnowEnergy EB03
guard passed `11/11` (run ID `ea8efddc-bba1-4ffe-a480-9fb807574312`).

`Ran:` `cargo check` passed for the affected orchestrator/runner/persisted-
restart packages and `cargo fmt --all -- --check` passed. The core Stage-3
solver is `2,982` lines after extracting the support API module, below the
3,000-line refactor threshold. These commands validate compilation and
formatting, not the missing snow-covered consumer, runner consumer path,
restart, scenarios, or comparator.
