# PERFIDX02 Disposition

Status: complete 2026-06-16
Evidence mode: **Ran** + **Static**

PERFIDX02 is complete. ADR-0022 Stage 2 passed its hard go/no-go.

## Result

The indexed shadow runtime surface was implemented as a dormant, env-gated
shadow beside the authoritative BTreeMap runtime surface. Storage authority was
not flipped.

H2637-scale clone economics passed:

- H2637 without UI: sparse clone `69.882x` faster than BTreeMap clone.
- H2637 with UI: sparse clone `54.096x` faster than BTreeMap clone.

The tightened registry passed completeness:

- H2637 without UI: `registry_symbol_count = 44746`,
  `unknown_symbol_count = 0`.
- H2637 with UI: `registry_symbol_count = 44746`,
  `unknown_symbol_count = 0`.
- OFE1-OFE5: every case `unknown_symbol_count = 0`.

Shadow equality passed:

- H2637 both UI variants: `mismatch_count = 0`.
- OFE1-OFE5 ladder: every case `mismatch_count = 0`.

Output safety passed:

- `ANCHOR_MISMATCHES=0`.
- `POST_SHADOW_UI_MISMATCHES=0`.
- `DETERMINISM_MISMATCHES=0`.

Required gates passed:

- `cargo fmt --all -- --check`.
- `cargo clippy --workspace --all-targets -- -D warnings`.
- `cargo test --workspace`.
- `cargo deny check`.
- `git diff --check`.

## Closure

PERFIDX02 closes as PASS. The proper follow-on is
`PERFIDX03-indexed-surface-authority-001`, scoped to the storage-authority flip
and its own output/conservation gates.
