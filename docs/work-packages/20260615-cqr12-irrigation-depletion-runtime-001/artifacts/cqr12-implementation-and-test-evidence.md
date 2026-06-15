# CQR12 Implementation And Test Evidence

Status: complete.

Static: implementation decomposed
`seed_hillslope_runtime_surface_from_irrigation_depletion` into private helpers
for system type mapping, header symbols, period iteration, period header/date
symbols, trigger validation, sprinkler fields, and furrow fields.

Static: public functions were preserved:

- `build_hillslope_runtime_surface_from_irrigation_depletion`
- `seed_hillslope_runtime_surface_from_irrigation_depletion`

Static: characterization added `15` focused `cqr12_*` tests in
`tests/integration/irrig10_irrigation_runtime_kernel_contract.rs`, covering:

- sprinkler and furrow fixture projection;
- existing runtime-surface preservation;
- non-finite minimum depth;
- maximum depth below minimum depth;
- zero element id;
- depletion trigger ratio outside `[0.0,1.0]`;
- negative date fields;
- sprinkler rate, depth-ratio, and nozzle domains;
- furrow end element, supply rate, supply duration, and fill-ratio domains.

Ran:

- `cargo test --test irrig10_irrigation_runtime_kernel_contract cqr12 -- --nocapture`: exit `0`, before production refactor, `15` passed.
- `cargo test --test irrig10_irrigation_runtime_kernel_contract cqr12 -- --nocapture`: exit `0`, after production refactor, `15` passed.

Note: the initial focused characterization surfaced a fixture-order assumption;
the sprinkler fixture's first period has `aprati=1.0` and `deplev=0.50`. The
test assertions were corrected before production refactor.
