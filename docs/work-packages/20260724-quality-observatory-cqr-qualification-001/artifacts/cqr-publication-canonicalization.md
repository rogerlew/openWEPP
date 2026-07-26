# CQR Publication Canonicalization

Evidence class: Ran + Static.

Selection-only CQR intake of successful QA evidence
`5cb7c5ea9471ab536ce7b9c9270992b68c0ab35b3c729e6d6c5095b57692baea`
returned `INVALID` with:

`JSON is not canonical: adjudicated-crap-report.json`

The report is produced by the external adjudicated-CRAP checker, while the
other JSON publication files use the observatory's canonical writer. The QA
verifier parsed and validated report semantics but did not compare its bytes
to canonical serialization.

The correction reads the external report and rewrites it through the canonical
JSON writer immediately after the checker succeeds, before any report digest,
payload, envelope, or publication identity is computed. Independent
verification now requires exact canonical report bytes. CQR remains
selection-only and launches no recollection.
