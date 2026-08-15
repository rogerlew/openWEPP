# Hydrology and Ownership Review at `73299b981`

Evidence class: `Static + Ran`

Verdict: `HOLD`

The fresh exact-byte review found two high-severity defects:

1. Unified soil-source validation binds the layer ID but not the request OFE to
   the configuration's exact production lane, permitting cross-OFE borrowing
   when layer IDs are shared.
2. Standalone finalization accepts an equal-but-arbitrary LSE rollback digest
   because the expected LSE beginning digest is absent from its sealed owner
   set.

The reviewer confirmed reciprocal frost structure, thermal-owner attribution,
complete attempted-hash fields on the path that used them, computed-snapshot
reporting, selector exclusion and the remaining D/A/F, condensation,
persistence and receiver-ledger surfaces. Focused results were 52/52 unified
integration, 10/10 custody authority and 145/145 selected orchestrator tests.

No finding is rejected or deferred. Heavy gates remain blocked pending
remediation and fresh exact-byte review.
