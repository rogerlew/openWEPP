# Independent Science Review A

Status: `PASS_WITH_FINDINGS`

Evidence mode: Static + Ran

Disposition agreement: agrees with
`BASELINE_FIDELITY_WITH_AUTHORITY_GAP`.

## Findings

1. Moderate: “caller chronology exactly” was broader than the audited term
   generator and midpoint/interval-start ordering. Rust's typed inactive
   threshold, inactive drift, and downstream redistribution/routing are not a
   whole-caller identity claim.
2. Moderate: handbook wording must say surface *soil* temperature `0 C` and
   explicitly name its daily `Tmax < -3 C` no-melt assumption, distinct from
   independent snow-surface/cold-content state.
3. Low: the Stage-3 closure assertion needed a predecessor evidence citation.
4. Low: focused tests needed both caller-bypass branches that caused the
   rejected first attempt.

## Independent Checks

Ran: syntax and the then-current three unit tests passed. Frozen PDF, Rust,
and pinned blob hashes reproduced. An independent AWK reduction reproduced all
four site hour/day and exposure counts across `394705` rows, and independently
reconstructed all `206047` nonzero-formula rows with maximum term residual no
larger than `5.54e-18 m`.

Required remediation: accept and correct all four findings before terminal
verification.
