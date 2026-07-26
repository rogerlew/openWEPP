# Finding Disposition

Evidence class: `Living`

## Executor findings

| Finding | Disposition |
| --- | --- |
| Initial migrator outputs were `native_cropland`, so the first smoke run passed without activating GSI/CP2 and emitted no research trace. | `CORRECTED`: seven source forest lanes were promoted to schema-native forest using explicitly uncalibrated/categorical seed operands; two open controls remain cropland. Real trace execution now passes. |
| Retained corpus lacks an independent quantitative phenology timing holdout. | `ACCEPTED_CAMPAIGN_BLOCKER`: CAL-03 can preserve evidence, but CAL-04 fitting is `AUTHORITY_BLOCKED` until new authority is prospectively admitted. |
| No evidence-derived probability priors are retained. | `ACCEPTED_LIMIT`: search-domain sampling cannot be called a scientific prior. |
| No site-matched leaf/needle/fine-woody source composition is retained. | `ACCEPTED_CAMPAIGN_BLOCKER`: CAL-05 source adequacy/decomposition fitting cannot close from this corpus alone. |
| Uncalibrated deciduous/mixed seeds accumulate residue and do not reach practical equilibrium in 45 years. | `RETAINED_CONTRARY_EVIDENCE`: no parameter or physics change made. |
| Protected fixture periods are 45 years, so CAL-02's years 91–100 equilibrium cell is unavailable. | `NOT_EVALUABLE`: periods remain protected rather than being extended. |
| Review A/B: no direct JSONL/default-off/identity/I/O tests. | `CORRECTED`: focused runner tests now execute the real writer/schema and cover disabled configuration, missing identities, required-null/nonfinite rejection, and typed open failure. |
| Review A: enabled trace allowed null site/arm and required nonfinite values could serialize as null. | `CORRECTED`: configuration requires nonempty identities and a recursive pre-serialization validator permits null only for four declared optional fields. |
| Review A/B: annual net omitted a possible January 1 transition. | `CORRECTED`: pre-year stock is reconstructed from the first post-update stock and first-day fluxes; stock and flux nets must agree. A boundary-active regression test passes. |
| Review B: analyzer required lane-contiguous rows although production emits day/lane interleaving. | `CORRECTED`: uniqueness is global and chronology is checked independently per site/arm/lane; an interleaved two-lane regression test passes. |
| Review A: gate prose called 58 ledger rows a 55-record bijection. | `CORRECTED`: evidence now says 55/55 corpus IDs covered plus three explicit gap rows. |
| Review B re-review: stable-schema tests covered only selected aliases and gate counts were stale. | `CORRECTED`: both Rust and Python enforce the complete required string/numeric path set while preserving only four declared nullable paths; actual writer and analyzer tests pass. Gate counts and line disposition are current. |
| Review B terminal re-review: four nullable fields were type-checked only when present. | `CORRECTED`: both validators now require all four paths and accept only null or finite numeric values; a missing-nullable-field regression test passes. |
| Verification A: relational corpus omitted coordinates, license/terms, transformations, and complete missing semantics. | `CORRECTED`: every source object, including the internal protocol authority, now has a joined metadata row recording exact coordinate availability/binding, controlling terms, transformations, and missing semantics; checksums and object coverage pass. |
| Verification B: equilibrium accepted one transient window and run command/hash mixed debug/release identities. | `CORRECTED`: equilibrium now requires the candidate and every subsequent rolling ten-year window to pass, with a transient-pass regression; the run command now names the recorded release executable. |
| Verification B: authorization wording did not use the mandatory direct structure. | `CORRECTED`: package and kickoff prompt now state the exact subagent authorization, expected outputs, and write-access boundary. |

Independent review findings are appended before terminal verification.
