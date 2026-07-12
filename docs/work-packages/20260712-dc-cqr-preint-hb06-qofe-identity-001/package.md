# DC CQR HB-06 — Canonical QOFE Identity

Status: `ACTIVE`

## Objective

Close `DC-CQR-HB06-001`: WB13 row construction admits `QOFE != Q` when the
per-OFE policy marker is set, contrary to the current canonical public
`QOFE == Q` convention.

## Correction Authority Envelope

- Canonical authority: `SC-WATBAL-001#INV-WATBAL-098`,
  `SC-SYSTEM-001#INV-SYSTEM-031`, and
  `SC-RUNOFFPART-001#INV-RUNOFFPART-032`.
- Production write set: `crates/openwepp-summary-accumulator/src/lib.rs`,
  limited to WB13 relationship validation and active HB-06 decomposition.
- Test write set: focused summary-accumulator and WB13 output-surface tests.
- Allowed correction: require `QOFE == Q` within the existing 1e-9 tolerance
  regardless of the provenance marker, preserving QOFE-first relationship
  priority and exact typed error.
- Excluded: anti-clone genuineness heuristics, raw local-runoff evidence,
  formulas, symbol mapping/order, schemas, format, tolerances, and runner
  consumer adoption (separately reviewed scope).
- Acceptance: aggregate and per-OFE mismatches reject; equal values accept;
  all other mappings/relationships retain order; coverage/floors/CRAP pass.

Conversion rule: the current authority is explicit and the local mechanism is
safe/testable, so the correction is mandatory.

## Progress

- [x] Resolve reviewer disagreement from canonical contract text.
- [ ] Record a red per-OFE mismatch regression.
- [ ] Land the canonical identity correction.
- [ ] Close all eligible module function floors and regenerate metrics.
- [ ] Complete dual review/verification and terminal disposition.

## Review And Delegation

Subagent authorization: this package explicitly authorizes spawning/delegating
to one bounded implementer and two read-only review/verification agents for the
declared source, tests, and evidence.

## Outcomes

Pending execution.
