# Coordinated successor science/ownership review

Evidence mode: `Static`

Independence: reviewer verified all six frozen manifest hashes, read no other
review output, ran no tests and made no edits.

Recommendation: `HOLD`.

| ID | Severity | Finding | Required correction |
|---|---:|---|---|
| `TBTV20-OWN-001` | Critical | Batch request tags/primitive encodings are named, but carrier, hydrology and joint preimages lack exact ordered fields; CoupledTime result/topology/proposal surfaces likewise lack complete closed schemas. | Specify schema version, closed domain tag and exact ordered field preimage for every Batch V2/result/topology/core/receipt hash. |
| `TBTV20-OWN-002` | Critical | Zero-prefix absence/replay proof is stated per proposal core, but the witness preimage contains no proposal-core, event-result, forcing or topology digest. Identical witness bytes can be substituted across cores. | Bind the exact prospective core and its event/forcing/topology evidence without creating a digest cycle. |
| `TBTV20-OWN-003` | Major | Candidate v5 ambiguously combines the positive cursor-to-CN-prefix complete-owner join and the zero-duration prefix-to-post-event `{snow}` mutation. | Define two separate beginning/ending owner-set joins and their one-to-one receipt chain. |
| `TBTV20-OWN-004` | Major | V138 does not explicitly exclude newly terminal liquid from the carrier-to-hydrology join, risking premature consumption before ProducedUnconsumed parcel creation. | Require terminal liquid to remain exclusively in the accepted high snow endpoint/pending parcel and absent from hydrology/surface-liquid ingress until the later authorized receiver. |

No production wiring is authorized.
