# Rejected-trial diagnostic custody/reachability review

Evidence class: `Static`

Reviewed SHA-256:
`af381f66ad624cdb90770bcb454fea3f41f65131c0ad7751146a708eb096b114`.
The hash matched. The reviewer made no edits and did not read the other review.

Recommendation: **HOLD**.

## Findings

1. **Critical — complete evidence is unreachable inside the declared write
   boundary.** The rejection layer has only aggregate trial state/ledger data;
   complete carrier receipts live upstream and the full joint is discarded.
   Correlation requires a production temporal-control observer or duplicated
   acceptance logic, both explicitly prohibited by the mini-gate.
2. **Critical — the evidence digest is not closed.** Nested receipt encodings,
   collection keys/order, enum wire values, inclusion rules and exact beginning
   and ending owner identities are absent, so an independent consumer cannot
   reconstruct the proposed evidence digest.
3. **Major — panic injection conflicts with the normal rejection trajectory.**
   A callback panic would unwind before the solver reaches its established
   `BelowCarrierDomain` outcome; no admissible catch boundary is defined.
4. **Major — generated-amount and terminal-liquid custody are ambiguous.** The
   schema must distinguish precipitation/carrier amounts from the later
   ProducedUnconsumed terminal parcel and prove that no terminal parcel or WB14
   receiver credit is observed, inferred or fabricated.
