# Inherited Baseline Defect

Evidence class: Ran.

The first selected combined contract run produced 27 passes and one failure:

`GATE-POLICY-DIGEST-DRIFT: impact map does not bind the current testing strategy`

Order 4 closeout changed
`docs/standards/testing-and-gate-strategy.md`, while
`gate-policy/v1/impact-map.json` retained the prior `policy_sha256`. This is an
inherited mechanical authority-binding defect, not an Order-5 behavior failure.

Authorized correction: replace only `policy_sha256` with the SHA-256 of the
current canonical testing strategy, then rerun the failed contract. Matcher,
risk, gate, and workflow semantics are out of scope.
