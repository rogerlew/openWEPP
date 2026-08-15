# Rust Correctness Review at `73299b981`

Evidence class: `Static + Ran`

Verdict: `HOLD`

The fresh exact-byte review found three material defects:

1. A soil request can retain one OFE identity while its source mapping selects
   another configured OFE lane that happens to share the same layer ID.
2. Standalone sealed finalization requires the LSE rollback row to be
   unchanged, but does not bind it to the actual beginning LSE digest.
3. Source-map, winter-domain and exact-one public failures can retain only the
   request-batch hash rather than the complete ingress, WB14 and soil-mapping
   attempted-input hash.

The reviewer also requested a poison for frost indices beyond the production
lane layer cardinality. The complete integration suite passed 52/52 and diff
hygiene passed, but passing tests do not override the findings.

No finding is rejected or deferred. Heavy gates remain blocked pending
remediation and fresh exact-byte review.
