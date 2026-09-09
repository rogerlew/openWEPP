Ran: frozen source-cut3 physical line counts via wc -l, including completed
provider exclusions, cost module and canonical reader recovery.

WARN: isolated LSE src/transaction.rs is 2,954 lines (A 2,952). Only the cfg-only
accepted-operand hook is added; no transaction logic expansion or refactor is
needed for this bounded observation. New audit implementation lives separately.
Any growth to 3,000 nonexempt lines requires a split before closure.

WARN: primary tests/integration/land_surface_energy_balance_authority_contract.rs
is 2,021 lines after terminal formatting (measured cut3:2,013). Scope is the new
v34 binding and current metadata reconciliation;
historical expected-red bindings remain intact. Keep future version-specific
binding additions in a dedicated test module rather than growing this history.

WARN: isolated vegetation_boundary_authority_contract.rs is2,843 lines. Its
bounded recovery restores existing primary reader/section routing only, with
unchanged assertions. Future added version-specific checks should use separate
modules; no additional assertion implementation or broad refactor in this cut.

All other touched isolated Rust files are below2,000 lines: core479, core tests261,
oracle803, cost654, capture1668, audit809, solver802, error358, numerics537,
Covered solve1483, V3 solver341, lib347, V3 bridge237, V3 transaction944,
runner harness780. Reader integration files: SurfaceLiquid918, LaneD1515,
terminal receiver287, solver architecture161, shared carrier683, helper28.
Primary current-version tests are LaneD1515 and shared carrier683; owning LSE
binding2021 matches terminal isolated J. No touched file reaches3,000; no generated
exemption or waived split threshold is claimed. Recount if source changes again.
