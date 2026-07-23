# RTR-047 And RTR-048 Intent Binding Evidence

Evidence class: Ran unless labeled Static.

## Retained Automatic Run

- Run: `29981856347`
- Exact head: `be7853fecfeaf791e458ade1a02cc6853fbecff2`
- Runner CLI preflight: PASS.
- Durable-history restore: PASS.
- Superseded-head rejection: PASS.
- Planning: not started.
- TESTGATE nodes and expensive gates: zero.
- Retained error: `testgate.py` required `--intent-package`.
- Authenticated recovery index: empty `attempts.jsonl`.

## Canonical Corrections

RTR-047 binds push execution to exactly one case-sensitive
`TESTGATE-Intent-Package` head trailer and trusted manual execution to one
explicit input. Missing, duplicate, malformed, event-inconsistent, lexical
dot/dot-dot, and CR/LF output-injection declarations fail before planning.

RTR-048 recognizes only exact
`ACTIVE / READY-REPOSITORY-ATTESTATION` as the missing active package state;
terminal, blocked, typo, and case-drift states remain inactive.

Ran: dual implementation review and dual terminal verification pass. At exact
verification head `9c27ea37319a206547556451dc6683a999ae3268`, package-chain
reconstruction is READY at
`cbf003db91f3ecfdb2090b53f38c37c9cdf70ae80dfa4795ba31b5a88afafab6`,
with 26 changed paths and zero unauthorized paths.

Ran: the canonical ledger verifies at 161 records. RTR-047 is CLOSED at
`21fd9458a9c765a9888ec31f291797dad44b36c2158b5ded376ce5f3e33accac`;
RTR-048 is CLOSED at tip
`5cb57d17691454ccf2392712c2612c552a1eae7446a8e81318e3c556d166b34b`.
The effective open-defect count is zero.
