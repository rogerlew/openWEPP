# ASSURE-06 Review-Entry Roots

Evidence class: Ran

## Source Identities

| Object | SHA-256 |
| --- | --- |
| Principal registry | `c2b2f2499b58651152f2e9415a61cacb51f4fa1397006cd9ce43b2f4436b96f0` |
| Manuscript | `8ad2eeea3a21640a9f551dfee7d0b9421b343b3592ddc88abcbf6b90a27a8def` |
| Supplement | `40d1104e097a333a5b4093412c44adc81d7a780b79c3751beae1ab578e9ffbc8` |
| Agent-assistance packet | `7facef9067ae59b908903fa684d33b5299f4e5010776cbacdb8428d3f6587286` |
| Report manifest | `82d5b386055d03458b9357564dba36cb6062b96237db9f14dd4b21e34ec1e537` |
| V2 catalog | `063bfb0d3b63b1aa5abccdf877e97ffd3fa447b08458aced723963e8461c3a64` |

## Layered Review Roots

The retained standard-library Rust helper calls the public
`V2Repository::review_roots` API. It was compiled against the current
`openwepp-assurance` library and run independently against both checked staging
roots:

- `/tmp/assure06-review-entry-a.tmlbLd`
- `/tmp/assure06-review-entry-b.wmj6qi`

Both runs returned:

```text
report_id=snow-and-frozen-soil-process-evaluation
subject_root=11a473da9b26a31d017d1581e194136082e3bc8f79edefb95051546406e5aa4e
finding_ledger_root=595f8ead6ada47b1cf7bbcb25bfb1f057b937451bc3ce38b79b8baf4d8b61674
approval_lock_root=null
release_transfer_root=null
```

The complete staging trees were byte-identical. Rebuilding the first staging
root after binding the declared roots and catalog manifest identity did not
change either calculated review root. Review remediation corrected stale
`DRAFT`-era reader disclosures, and both roots were recalculated and rebound
before the final two-root check. The report binds the exact subject and
finding-ledger roots and correctly has no approval or release-transfer root.
