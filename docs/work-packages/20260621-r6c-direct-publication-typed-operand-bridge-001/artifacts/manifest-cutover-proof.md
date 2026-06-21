# Manifest Cutover Proof

Evidence mode: Static.

R6C must prove cutover-mode manifest production reads typed direct publication
projection for:

- schema ID;
- output policy;
- direct runtime counters;
- checksums;
- provenance;
- warnings and gate metadata.

Pre-change state: production manifest writing remains compatibility-owned.

## Execution Disposition

Manifest cutover remains blocked. R6C did not change manifest production or
schema behavior because the direct runtime does not yet retain production
publication counters, checksums, warning metadata, and output policy as a
direct manifest projection.
