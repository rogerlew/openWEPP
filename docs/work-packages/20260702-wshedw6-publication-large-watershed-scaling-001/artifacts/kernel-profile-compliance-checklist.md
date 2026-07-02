# Kernel Profile Compliance Checklist

Status: `passed`

Evidence mode: `Static:` source/contract review plus `Ran:` focused tests and
full-fixture scaling evidence.

| Requirement | Disposition | Evidence |
| --- | --- | --- |
| Contract-first sequencing | PASS | `pre-implementation-contract-gate.md` and `publication-operand-lineage.md` recorded touched operands before final evidence. |
| No surrogate or provisional physics | PASS | Unavailable typed publication operands emit nulls; no channel-balance values are fabricated from routed runoff, impoundment outflow, or `cbase`. |
| Typed guard preservation | PASS | Public CLI keeps typed `WatershedNetworkFrame` dispatch and `WatershedPublicationFrame` publication. |
| Real consumer proof | PASS | `consumer-path-evidence.md` proves public `openwepp-cli-watershed` calls `write_typed_publication_parquet_outputs`. |
| Conservation reconstruction | PASS | `conservation-reconstruction.md` independently reconstructs source geometry area from committed source runfiles/slope files, then checks normalized `Q` and the `runvol`/`chanwb Inflow` projection. |
| Schema preservation | PASS | `output-contract-evidence.md` and scaling JSON prove all `14` required parquet outputs keep schema identity across job counts. |
| Full fixture closure | PASS | `onshore-xenophobia` full `1305`-hillslope fixture and `carnivorous-adobo` full `32`-hillslope fixture both ran from committed inputs. |
| Final gates | PASS | Full final command results are recorded in `gate-results.md`. |
