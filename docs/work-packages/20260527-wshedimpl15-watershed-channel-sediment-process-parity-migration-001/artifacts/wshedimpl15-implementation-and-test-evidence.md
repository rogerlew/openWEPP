# WSHEDIMPL15 Implementation and Test Evidence

Status: complete
Evidence mode: static+ran
Date: 2026-05-27

## Static
- Runtime seam (`runtime_inputs.rs`) now projects channel sediment-control
  symbols from parsed watershed channel payloads:
  `ishape`, `ienslp`, `chnz`, `chnnbr`, `chntcr`, `chnedm`, `chneds`,
  `ctlz`, `ctln`.
- WS10 channel kernel (`lib.rs`) now:
  - requires projected channel sediment controls with typed fail-closed guards,
  - derives/publishes baseline conversion scaffold symbols:
    `ws10_channel_{id}_{chz,nbarch,crsh,depmid,depsid}`,
  - preserves existing `qsed` / `tc` publication-family behavior from WSHED06
    (no claim of full process-parity closure).
- Integration tests updated for WS15 scaffold vectors and expanded fixture
  symbol seeding for second-channel topology paths.

## Ran
1. `cargo fmt` -> pass
2. `cargo test -p openwepp --test ws11_channel_routing_physics_equivalence_contract` -> pass
3. `cargo test -p openwepp --test ws10_watershed_kernel_contract --test ws12_impoundment_physics_equivalence_contract` -> pass
4. `cargo fmt --check` -> pass
5. `cargo clippy --workspace --all-targets -- -D warnings` -> pass
6. `cargo test --workspace` -> pass
7. `cargo deny check` -> pass (`advisories ok, bans ok, licenses ok, sources ok`)
