# Parser Runtime Equivalence

Static: the package did not edit parser code or PL runtime contracts. The
production change decomposes target-module private projection stages while
preserving symbol names, typed error variants, thresholds, and public APIs.

Ran: `cargo test -p openwepp-hillslope-orchestrator runtime_inputs::tests::management`

Exit code: `0`; result: `26 passed`.

Ran: `cargo test --test parser_runtime_seam_integration management_runtime_surface`

Exit code: `0`; result: `10 passed`.

Ran: `cargo test --workspace`

Exit code: `0`; result: workspace unit, integration, and doc tests passed.

Numeric-equivalence statement: extraction preserved the original arithmetic
expression grouping for residue-depth projection and initial live-canopy
assimilation. No formula, constant, threshold, public symbol, or guard policy was
intentionally changed.
