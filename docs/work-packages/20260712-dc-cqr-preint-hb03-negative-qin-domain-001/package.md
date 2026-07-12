# DC CQR HB-03 — Negative Erosion Qin Domain

Status: `TERMINAL-PASS`

## Objective

Close `DC-CQR-HB03-001`: Wave-1 operand assembly accepts negative upstream
`qin_m2_s`, allowing an impossible erosion-lineage discharge to affect routing
activation and interrill selection.

## Correction Authority Envelope

- Canonical authority: `SC-SED-001#INV-SED-008/013/016`, especially positive-
  upstream full-reinfiltration continuity and typed erosion-only handoff.
- Production write set:
  `crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion_seed.rs`,
  limited to qin admission and mechanical HB-03 decomposition.
- Test write set:
  `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_erosion_seed.rs`.
- Allowed correction: after existing finite checks, reject negative standalone
  and handoff qin with `NegativeDirectValue` retaining their existing exact
  field identities; preserve conflict priority before handoff qin domain use.
- Excluded: water-transfer authority, positive-inflow/full-reinfiltration
  semantics, runoff/passby gates, formulas, class arrays, float grouping,
  schemas, and other process families.
- Acceptance: conflict and non-finite priority remain exact; both negative qin
  sources fail typed; zero/positive inputs retain behavior; real Wave-1
  continuity and conservation consumers pass; HB-03 coverage/CRAP closes.
- Security impact: none; the correction strengthens fail-closed admission.

Conversion rule: the mechanism is reproduced, in-envelope, canonically
authorized, safe, and directly testable, so correction is mandatory.

## Progress

- [x] Reproduce standalone negative-qin acceptance.
- [x] Confirm authority, mechanism, and bounded write set.
- [x] Correct standalone and handoff admission with exact priority.
- [x] Resume HB-03 decomposition and focused measurement.
- [x] Complete dual review/verification and terminal disposition.

## Review And Delegation

Subagent authorization: this package explicitly authorizes spawning/delegating
to one bounded implementer and two read-only review/verification agents for the
declared source, tests, and evidence.

## Outcomes

The qin boundary now rejects negative standalone/handoff erosion discharge while
retaining exact authority/conflict order and positive full-reinfiltration
continuity. The target falls from CRAP 34.808 to 26.278; slice coverage is
94.118% lines / 93.617% regions and every function clears 75%. Focused tests,
Clippy, format, and diff pass. Dual review/verification pass with every finding
closed. Disposition: `TERMINAL-PASS`; `DC-CQR-HB03-001` is closed.
