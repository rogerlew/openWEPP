# Rust Correctness Review At `2e32a8a0e`

Evidence class: `Static + Ran`

Verdict: `HOLD`

The reviewer inspected exact commit
`2e32a8a0e3f5473720704b4d419d7340f63a5ca9` against baseline
`af9a989063aa8751dfadb14c442e1b360653658c` and accepted two material
implementation findings.

1. `A-TERMINAL-2E3-HIGH-001`: a full-infiltration mass can round-trip through
   depth one ULP upward. For
   `x=0x1.f9e1df20c7aa4p-6`, `(x/1000)*1000` is the next larger binary64 value,
   producing a negative source remainder. Producer and independent replay use
   the same conversion, and raw source mass is absent from the independent
   join. This violates nonnegative partition and exact `I_b + E_b = X_b`.
2. `A-TERMINAL-2E3-HIGH-002`: public unified request/protocol/native-domain and
   ingress validators do not consistently implement canonical
   `SURFACELIQUID-E-001..011` precedence. Mixed failures can report E003/E006
   before E002/E005, E005 before E004, or E008 for a nonfinite interval.

The reviewer also recorded `A-TERMINAL-2E3-EVIDENCE-003`: exact-commit heavy
workspace evidence remains pending after the material source changes. This is
an acceptance gate, not a separate implementation defect.

Commands run by the reviewer:

- surface-liquid filtered orchestrator suite: 70/70 passed;
- unified LSE/real-hydrology integration: 32/32 passed;
- custody authority contract: 10/10 passed;
- formatting and base-relative diff hygiene: passed.

The focused successes do not override the two accepted defects. This review is
immutable historical evidence and does not become PASS after later edits.
