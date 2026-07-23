# Aggregate Verification B

Static/Ran: HOLD on trust class after retained technical evidence PASS.

Ran: independently verified receipt `c22fe3f...f06ca`, all 15 checkpoints and
artifact hashes, exact plan/audit/head/binary bindings, the 151-record durable
ledger, 2,293/2,293 ordinary and instrumented Nextest results, unchanged source
manifests, and fresh global CRAP PASS with zero actionable rows. No effective
tooling defect remains open.

Static: the receipt claims and verifies only `LOCAL_UNTRUSTED`, while
`observation.json` records `LOCAL_RECEIPT_PENDING_GITHUB_ATTESTATION`. The
canonical trust contract says local-untrusted evidence cannot close this
`INCREMENT` boundary. Obtain and verify the native repository-reviewed
attestation envelope for the exact receipt and HEAD without rerunning gates.
