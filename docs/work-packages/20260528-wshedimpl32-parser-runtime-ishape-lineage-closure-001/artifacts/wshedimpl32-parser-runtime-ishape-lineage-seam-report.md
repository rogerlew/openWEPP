# WSHEDIMPL32 Parser/Runtime `ishape` Lineage Seam Report

Status: complete  
Evidence mode: static+ran  
Date: 2026-05-28

## Static
- Parser contract closure:
  - strict mode now accepts naturally eroded class (`ishape=3`) in addition to
    rectangular/triangular classes (`1`, `2`),
  - compatibility mode now normalizes legacy out-of-domain values (`ishape>3`)
    to naturally eroded class (`3`) with explicit warning continuity.
- Runtime seed closure:
  - WS10 channel seed path now enforces explicit `ishape` domain guard
    (`1..=3`) before runtime symbol publication,
  - parser-projected naturally eroded class now passes unmodified into
    `ws10_channel_{id}_ishape` runtime state symbols.
- Lineage result:
  - parser strict/compat behavior and runtime kernel ingress now share a single
    explicit naturally eroded class contract (`ishape=3`) with typed failure
    for out-of-domain projection.

## Ran
- `cargo test --test infile_watershed_channel_parser_contract` -> pass
- `cargo test -p openwepp-watershed-orchestrator watershed_channel_runtime_seed_projects_naturally_eroded_ishape` -> pass
- `cargo test -p openwepp-watershed-orchestrator watershed_channel_runtime_seed_rejects_out_of_domain_ishape` -> pass
