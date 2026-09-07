# Science and output parity

Status: `PHASE 4 COMPLETE — PARITY PRESERVED`

Evidence mode: `Static + Ran`

The admitted R1 control changed only observation posture: Arm A ran the
retained release binary, and Arm B ran that same binary with an external
observer. No solver, equation, tolerance, event, restart, receipt, publication,
or acceptance source path changed. The observer script is package-local raw
evidence and is not linked from production code.

| Protected item | Admitted result | Evidence |
|---|---|---|
| Source/outlet/storage/clamp closure | Exact in all 24 arms: source `0.8488061229561478 m³`, outlet `0.8471105124736579 m³`, end-window storage `0.0016956104824910018 m³`, clamp `0.0 m³` | `raw/current_memory_results.jsonl`, offline reconstruction |
| Workload topology | One hydrology lane; two vegetation occupancies/strata; six soil/thermal layers | `raw/authentic_owner_seed.json`, source map |
| Support/trial lifecycle | 48 parent supports (44 snow-free, 4 covered); 20 direct, 32 split-child, 4 accepted microsteps, 56 accepted publication supports; one committed day | Every admitted probe row |
| Qualification state | Complete counters and balanced scopes in every arm; all 24 arms exit 0 | JSONL and raw per-arm logs |
| Output identity | HBP/WAT/PASS/parquet/manifest validation and the committed snapshot remained exact; no observation arm altered science values | Runner probe and paired reconstruction |

The current control proves observation-on/off parity for the retained baseline
only. It does not prove parity for historical revision-31 or revision-61
candidates: neither candidate was reconstructed into an equivalent executable,
and neither was run in the retained checkout. Historical revision-31 panic and
revision-61 RSS triples remain archival expected-red evidence, not current
regressions.

No source write was required after exact consumer mapping; all package changes
are documentation/raw evidence. A future lawful positive fixture or event audit
must preserve the same protected operands and output identity before any replay
claim is considered.
