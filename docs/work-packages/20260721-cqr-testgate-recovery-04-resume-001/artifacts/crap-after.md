# CRAP After

Ran: same-LCOV CRAP passes at exact `9c0db17d` with 29 production functions,
zero rows above 30, and maximum 25.3961 at `verify_archive_provenance`.

| Original target | Before | After |
| --- | ---: | ---: |
| `load_candidate_internal` | 96.1443 | 10.0031 |
| `verify_native_attestation` | 41.8250 | 3.0000 |
| `verify_checkpoint` | 36.3893 | 5.0000 |

Static: all 14 extracted candidate, attestation, and checkpoint helpers are at
most 9.8889. The corrected measurement's CRAP JSON is bound to the same LCOV
used for the authoritative isolation-corrected coverage closure.
