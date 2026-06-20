# Scope Selection

Static:

Selected scope: full direct executor lifecycle only.

The current direct executor constructs one `DirectRunFrame`, runs the R3C
run-level transfer span once, and then constructs one `DirectDayFrame` per
lane for day `0`. R5A widens this to every `(day_index, lane_index)` pair and
adds explicit persistent state handoff/commit semantics.

Accepted direct phase math remains unchanged:

- R3A input accounting;
- R3B water ledger;
- R3C lane transfer ledger;
- R4C/D/G/I-L/M/N/O/PQZ existing direct spans.

Deferred phase ownership remains out of scope:

- `StorageBounds`;
- `DecompositionTransition`;
- `ResiduePartitionTransition`;
- `AnnualGrowthTransition`;
- `PerennialGrowthTransition`.

Those deferred phases must be visible as canonical lifecycle hold/no-op status
counts, not compatibility calls.
