# Finding disposition

Status: `PHASE 5 COMPLETE — TERMINAL HOLD / V31 DISCRIMINATOR CLOSED; V61 ATTRIBUTION OPEN`

Evidence mode: `Static + Ran`

Every review and verification finding is dispositioned below. Evidence limits
are recorded as limits, not converted into causal claims.

| Finding | Disposition | Rationale / evidence |
|---|---|---|
| v31 earliest runtime limiting predicate | **Closed by direct debugger capture; release target still fails** | Audit-return capture from the exact historical binary decoded all 2,000 authentic sweeps before aggregation: 200 rows `48/14/10/24`, 1,800 rows `54/14/16/24`, zero `58/14/16/28`; every row completed and not failed. Boundary-aware one-sided stencils at the closed beta/temperature domain omit signed probes, producing the logical/complete shortfall. Lifecycle capture shows one authentic audit followed by a distinct forced-complete oracle audit. No assertion bypass or binary mutation. |
| Authentic replay frequency/topology | Accepted as bounded, workload-sensitive result | Real runner control proves N=2/S=6 and 48/56/20/32/4 lifecycle counts. The historical focused real-runner diagnostic passes with at least one completed replay probe and parity, while the authentic release aggregate fails before JSON; retained source has no replay implementation or published counter. Carrier finalization timing is not replay evidence. |
| Revision-61 causal memory difference | **Open; no treatment attribution** | Historical triples are unpaired and endpoint-only. R1 observer control is same-binary A/B and proves observer perturbation, not candidate treatment. Manual reconciliation produced a detached real-consumer run (exit 0; 200 calls; exact closure; raw `v61_manual_reconciled_release_probe.log`, SHA-256 `dbced82c…9955c1`), but its documented changed-Rust manifest is `19a07121…23421` (an earlier undocumented canonical computation recorded `2fc5e8ca…e6eb7f`). Neither can be compared to historical `650f6713…57d41` without the historical Git-index/per-file manifest and candidate binary; no matched candidate/control treatment exists. |
| `rss_kib` semantics | Closed | Source anchor and raw log establish `/proc/self/status` `VmRSS` point sample during JSON construction before cleanup; prose now excludes peak/heap/HWM claims. |
| Observer harness defects | Closed by R1 correction | Readiness, fail-closed field gaps, timeout, identity, balanced ordering, and environment/affinity custody are persisted; earlier failed sets remain raw evidence. |
| Arithmetic / metric wording | Closed | Recomputed 4/12 positive endpoint deltas, all 12 positive wall deltas, median +577,643.5 us, and sampled-HWM lower-bound wording. |
| Candidate source custody | Closed as nonproduction boundary | All recovered/replayed candidates stayed in detached `/tmp` trees; no Rust source path was edited in the retained checkout. |
| Comparator service failure | Accepted process limitation | Fixed-role `comparator_suite_runner` failed at spawn due to service usage limit; bounded parent fallback and exact failure are recorded. |
| Successor recommendation | **No successor authorized** | V31 runtime cause is now characterized, but revision-61 has no authenticated equivalent candidate treatment or causal memory contrast. Required gates for any future increment remain listed in final disposition. |
