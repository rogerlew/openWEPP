# simimpl13-replay-parity-full-closure-criteria

Status: phase-d-complete
Evidence mode: Static + Ran
Date: 2026-05-25

## Static
- Criteria below define the minimum measurable conditions for declaring
  hillslope replay/parity implementation fully closed and promotable.

## Ran
- Criteria are derived from SIMIMPL11 replay evidence and SIMIMPL13 gap audits
  (`span`, `comparability`, `tooling`, and `test blind spots`).

## Closure criteria
| criterion_id | promotable condition | measurement / artifact requirement | current status |
|---|---|---|---|
| `SIMIMPL13-CRIT-001` | Candidate timeseries span matches required replay window semantics for the target fixture lane. | Candidate `H*.wat` keyed row count aligns with baseline replay window expectations; evidence in strict + semantic artifacts. | fail |
| `SIMIMPL13-CRIT-002` | Candidate and baseline row-key domains overlap for all required replay keys. | Semantic report: `only_baseline_count=0`, `only_candidate_count=0`, `common_row_count>0` for required lane. | fail |
| `SIMIMPL13-CRIT-003` | Dat strict comparator lane passes for required dat replay surfaces. | Strict comparator JSON `strict_pass=true` with required include surfaces present. | fail |
| `SIMIMPL13-CRIT-004` | Semantic comparator passes for dat and parquet candidate surfaces. | Semantic JSON `semantic_pass=true`; no unresolved column or key-domain blockers. | fail |
| `SIMIMPL13-CRIT-005` | Required investigation-column set is complete in semantic reports. | `investigation_columns_missing=[]`; `baseline_only_columns=[]`; `candidate_only_columns` dispositioned. | fail |
| `SIMIMPL13-CRIT-006` | Replay publication provenance remains simulation-owned and runner-executed. | Manifest confirms `scheduler_kernel_executed=true`, `publication_source=scheduler-kernel`, `wb13_publication.source=simulation-owned`. | pass |
| `SIMIMPL13-CRIT-007` | Contract-derived tests cover span, key-domain, and comparator-mapping closure paths. | New/updated tests explicitly fail when span collapse, key mismatch, or parquet alias drift reappears. | fail |
| `SIMIMPL13-CRIT-008` | Governance evidence bundle is reproducible and complete. | Provenance manifests include command, binary/tool hashes, comparator outputs, and disposition linkage for each replay lane. | partial |

## Hold policy
- Replay/parity promotion remains `HOLD` until all hard criteria (`CRIT-001`
  through `CRIT-007`) are `pass`.
- `CRIT-008` may remain `partial` only with explicit risk acceptance; absent
  approval reference, disposition remains `HOLD`.

## Phase D conclusion
- SIMIMPL13 defines a concrete promotability target.
- Current evidence satisfies provenance ownership but fails span/key/comparator
  closure requirements.
