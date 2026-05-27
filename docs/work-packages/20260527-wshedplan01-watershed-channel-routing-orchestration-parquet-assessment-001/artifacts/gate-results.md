# Gate Results

Status: complete

Evidence mode: static+ran

Date: 2026-05-26

## Static
- Package scope is assessment + queue authoring; no production watershed kernel
  edits were made.
- Contract-first queue sequencing is explicitly encoded for follow-on
  code-authoring packages.
- Baseline dependency correction applied: `chndet.for` reference replaced with
  `chnrt.for`.

## Ran
- `cargo test --test ws10_watershed_kernel_contract --test ws11_channel_routing_physics_equivalence_contract --test ws12_impoundment_physics_equivalence_contract --test cli04_runner_wat_parquet_contract_derived_tests`
- `cargo test -p openwepp-runner --test watershed_cli_behavior_contract`
