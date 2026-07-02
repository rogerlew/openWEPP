# External-Authority Guard Log

Evidence class: **Ran** (main checkout, 2026-07-02, disposition of
C01-CX-003; independently reproduces Codex's review run).

| Guard | Command | Result |
|---|---|---|
| Authority-suite anti-evasion | `bash tools/release/check_authority_suite_antievasion.sh` | `PASS: authority suite anti-evasion checks passed.` |
| Required-suite obligation contract | `cargo nextest run --test auth11_required_suite_obligation_guards_contract` | `2 tests run: 2 passed, 0 skipped` |

Both guards protect the observed-authority posture introduced by
`INV-SUBHYD-033` / `REF-SUBHYD-OBSERVED-LATERAL-ENVELOPE`: they enforce that
an authority suite cannot be silently weakened or bypassed. Green here
confirms the C01 amendment did not open an evasion path.
