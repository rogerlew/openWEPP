# Line-Count Governance

Evidence mode: **Ran**.

No touched Rust file reaches the `3,000`-line mandatory-split threshold.

| File | Lines | Disposition |
|---|---:|---|
| orchestrator infiltration reconciliation | 2,353 | warning band; follow-on split debt retained |
| orchestrator runoff reconciliation | 2,504 | warning band; follow-on split debt retained |
| runner day-input formatter | 2,454 | warning band; EB-04W suffix extracted |
| new accumulation/melt trace helper | 111 | focused helper |
| snowbench CoE adapter | 1,168 | below warning band; includes non-100-density regression |

The new JSON serialization was extracted to
`00f_snow_accumulation_melt_trace.rs` so the already-large formatter did not
absorb another ledger. A future mechanical refactor may split the two
orchestrator files, but that non-science rewrite is outside EB-04W and is not
needed to validate the additive diagnostic path.
