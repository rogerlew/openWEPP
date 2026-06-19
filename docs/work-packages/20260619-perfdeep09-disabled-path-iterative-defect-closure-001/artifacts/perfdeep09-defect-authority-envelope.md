# PERFDEEP09 Defect Authority Envelope

Status: closed - `READY-FOR-R2`.
Evidence class: Static + Ran.

Defect: `PERFDEEP09-DISABLED-PATH-R2-BLOCKER`.

Observed failure:

- Same-machine no-edit control reproduced the default-disabled gate miss:
  `perfdeep09_baseline_rep1 682.65 228924`.
- Gate: final H2637 default-disabled three-run median `<= 676.67 s`.

Attributed mechanism:

- PERFDEEP04 default profile already ranked
  `ensure_no_overflow_indexed_symbols_for_decomposition` as a top default-path
  cost (`9.18%` children / `7.72%` self). Static inspection confirmed the
  function scanned the full state-surface key map once per indexed root for
  each perennial decomposition control dispatch.

Ownership:

- Mechanism is in
  `crates/openwepp-hillslope-orchestrator/src/hydrology/07_decomposition_equations.rs`,
  inside the declared write set.

Authority / safety:

- Retained remediation changes only validation traversal shape: seven per-root
  full-map scans become one slot/crop pass that records first overflow per root
  and reports in the old root order.
- Typed guard `HS-DECOMP-E-008` remains fail-closed and is covered by a new
  regression.
- No process-physics math, output schema, publication meaning, default opt-in,
  direct executor, or R2+ runtime implementation changed.

Seven-gate bar:

| Gate | Status |
|---|---|
| Reproduction | PASS: `682.65 s` no-edit control |
| Mechanism | PASS: prior profile + static one-pass attribution |
| Ownership | PASS: hydrology write set |
| Authority | PASS: guard traversal only |
| Safety | PASS: typed guard preserved; no physics/output meaning change |
| Testability | PASS: focused `pl12_contract_conformance_rejects_unexpected_indexed_perennial_symbol` |
| Validation | PASS: final median `635.65 s`, protected identity passed |
