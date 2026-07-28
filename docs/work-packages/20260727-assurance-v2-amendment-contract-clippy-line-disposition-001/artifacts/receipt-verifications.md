# Receipt Verifications

Status: `PASS`

Exact evidence head: `6fead755fd6107d470df5dfd81b95a2da477b398`

Canonical subject: `ffe1dd71eec578a621f66fc2939304971653e92b`

## Verifier A

Evidence class: `Static + Ran`

Verdict: `PASS`

Verifier A recomputed 50/50 attempt-index hashes, all summary hashes, every
ledger entry/link, exact identities, 10/10 audit, 12/12 receipt, 2,387-item
inventory equality, full 2,361/2,361 evidence, source immutability, attempt-one
no-retry posture, and ADR-0041 disposition.

## Verifier B

Evidence class: `Static + Ran`

Verdict: `PASS`

Verifier B independently confirmed the same cryptographic and transactional
evidence. Both verifiers initially held on an overbroad “no Harvard access”
sentence because required regression tests read committed Harvard fixtures.
That accepted finding was corrected at the exact evidence head above. Both
then passed the precise claim: no CAL population, Harvard calibration workflow,
or protected/sealed-state mutation occurred; read-only fixture coverage is
disclosed.

No finding remains.
