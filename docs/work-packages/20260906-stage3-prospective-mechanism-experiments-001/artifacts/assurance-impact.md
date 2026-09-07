# Assurance impact

Static + Ran: read-only impact analysis by independent feed_forward_design.
No source adoption, normalization apply, publication or assurance promotion.

A full correctness run reports the production-DRAFT normalization test failed
because the global identity lock has stale SC-SNOWENERGY-001 dependency content.
This condition predates kickoff: lock SHA prefix 1d23afda, e89 kickoff7a12fda0,
A2b56 source94eb7a73. The additive experimental authority changes its identity
again; do not describe the entirety of this mismatch as package-introduced.

Read-only commands executed against the available release assurance tool:

```
target/release/openwepp-assurance plan --all --format json
target/release/openwepp-assurance amend adopt-report-source --report snow-and-frozen-soil-process-evaluation --path docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md --check
```

Plan selects the snow report's two aliases, not groundwater/native-canopy science.
Preview classifies adoption `scientific-full`, invalidated_authority empty, snow
science/content-review roots changed, communication/governance and other science
roots unchanged; requires assurance-implementation-package-v1. The actual snow
report is DRAFT. README's independent-review summary is stale.

Disposition: preserve the failed gate and inherited stale-assurance status; do
not expand this controlled experiment into adoption of all intervening preexisting
science changes. No claim of current assurance, release readiness, or production
qualification follows from experimental admission. A future source-adoption
package must own the canonical transaction and scientific-full review. No old
assertion is suppressed and the failing test is not relabeled PASS.
