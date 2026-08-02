# Line-Count Governance

Status: `PASS with recorded pre-existing exception`.

Evidence mode: `[Ran]`.

| File | Terminal lines | Disposition |
|---|---:|---|
| `09_snow_density.rs` | 1,990 | Below the 2,000-line ceiling after helper extraction; only 10 lines of headroom remain. |
| `00a_snow_frost_authority_impl.rs` | 699 | Below the ceiling. |
| `00c_day_input_builder_impl.rs` | 2,456 | Pre-existing oversized runner trace builder; EB-04V adds only the typed additive serialization block. |
| `SC-SNOWFREEZE-001.md` | 3,368 | Canonical cumulative contract; not production source. |
| `run_density_diagnostics.py` | 922 | Package-local analysis tool. |

The implementation moved attribution and downstream adjustment into focused
helpers rather than expanding the density entry point indefinitely. Splitting
the pre-existing runner JSONL formatter in this diagnostic package would widen
the write set without reducing scientific or runtime risk, so that structural
debt is recorded rather than opportunistically refactored. Named follow-up
intent: `RUNNER-TRACE-MAINT-01`, owned by runner maintenance, will split the
formatter when its next semantic edit occurs or the file reaches 2,500 lines,
whichever comes first.

Named density follow-up intent: `SNOW-DENSITY-MAINT-01` will extract the large
in-module test block or diagnostic helpers at the next semantic edit or when
`09_snow_density.rs` reaches 2,000 lines, whichever occurs first.
