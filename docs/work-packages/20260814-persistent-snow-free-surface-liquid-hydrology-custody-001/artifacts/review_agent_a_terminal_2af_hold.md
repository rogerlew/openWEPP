# Rust Correctness Review — `2afffa9dc`

Evidence class: `Static + Ran`

Exact reviewed commit:
`2afffa9dcbcee2681572f912d63d90e31c035118`.

Verdict: `HOLD`.

## Material Findings

1. `A-TERMINAL-2AF-HIGH-001`: public `WaterProtocol::validate()` interleaves
   row identity, domain, cardinality and bounds. An early NaN can mask a later
   transaction E002. The orchestrator duplicates four staged passes to
   compensate, while other public LSE paths call the divergent validator.
   Required closure is canonical staged validation in LSE reused by every path,
   with one mixed-defect vector through direct, owner-envelope, standalone and
   unified boundaries.
2. `A-TERMINAL-2AF-MEDIUM-002`: current implementation evidence incorrectly
   claims a final floating authorization remainder, contradicting the admitted
   symmetric common downward scale and actual implementation.

Ran evidence: LSE 28/28; integration 67/67; custody authority 10/10; affected
strict Clippy; formatting and diff hygiene.
