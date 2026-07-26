# Finding Disposition

Status: `COMPLETE`

| Finding | Disposition | Evidence |
| --- | --- | --- |
| CAL-04 had no exact model-to-observation operators or aggregation. | `ACCEPTED_CORRECTED` | CAL-04 now admits only Hubbard P3 upward and Harvard leaf-fall downward 0.5 crossings of trace `/gsi/gsi21`; equal-year interval RMSE and diagnostics are frozen in `cal04-partition-and-objective.md`. |
| Unscoreable budbreak, leaf-size, color, and combined Hubbard P1 entered the first extraction. | `ACCEPTED_CORRECTED` | The extractor and ledgers now retain only scoreable Hubbard P3 and Harvard leaf-fall records. Original objects remain retained without invented mappings. |
| HF003 fall 1992 conflicts with EML no-campaign wording. | `ACCEPTED_CORRECTED` | All Harvard fall 1992 values are excluded explicitly by extractor and metadata. |
| HF324's stated 28 joins were not reproducible from a retained artifact. | `ACCEPTED_CORRECTED` | `cal05-hf324-plot-matching.csv` and `tools/extract_cal05_matching.py` retain and deterministically rebuild keys, periods, units, counts, means, and `use.not=1`. |
| Operator delivery needed distinction from artifact preparation. | `ACCEPTED_CLARIFIED` | `operator-assistance-log.md` records the exact user-facing delivery channel and pending response; no third party was contacted. |
| Package and gate artifacts were nonterminal. | `ACCEPTED_CORRECTED` | Package, admission review states, gates, final disposition, catalog, and roadmap are reconciled before terminal verification. |
| CAL-05 fine wood remains pooled with bark/reproductive material. | `ACCEPTED_HOLD` | CAL-05 remains `PARTIALLY_LIFTED / PENDING_OPERATOR`; no decomposition fit is authorized. |
| CAL-04 observations lacked exact member/trace binding and aggregation across dimensions absent from the native model. | `ACCEPTED_CORRECTED` | Hubbard and Harvard deciduous fixture/runfile/trace identities are frozen; a single annual composite crossing is compared to within-year observations and objective years receive equal weight. |
| Hubbard 2025 observations exceed the protected fixture period. | `ACCEPTED_CORRECTED` | Extractor excludes years after 2024; the source object remains intact. |
| Missing modeled crossings had no fail-closed objective behavior. | `ACCEPTED_CORRECTED` | Any required missing crossing invalidates the candidate with infinite objective while retaining failed counts. |
| Full diff hygiene failed on checksum-preserved source CRLF. | `ACCEPTED_CORRECTED` | Directory-local Git attributes classify retained CSV/XML source objects as binary; exact bytes remain unchanged and the full check passes. |
