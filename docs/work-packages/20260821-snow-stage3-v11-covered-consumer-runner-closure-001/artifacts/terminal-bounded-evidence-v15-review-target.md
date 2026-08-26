# Terminal bounded evidence V15 review target

| Frozen member | SHA-256 |
|---|---|
| `package.md` | `7c98ae899ac706a64ad4188f87e98ffdb7fcfe05cf12606d31fe78a9a728b718` |
| `artifacts/gate-results.md` | `24914a3e283d6a15ab8a4d81fe9e4300c2fef4e0a4183211799ef44d29ff7e6d` |
| `artifacts/review-finding-disposition.md` | `6104ae41c1f53a0cfcc1c8b0121e7fac916d3f6aae36cfb400134e01c57cc9ac` |
| `artifacts/terminal-bounded-evidence-v15-raw-validated-authority.md` | `8d12ca699f4239399250868466045264e6224235172790d2b3733159c18d260a` |

Formatting and diff hygiene passed. Historical V20/V21 guards passed 5/5 in
nextest `4a94e96b-9047-4a1b-aaa8-5392f555dfa5`. The exact stable eleven-failure
comparator remains the frozen V14 census.

Two independent reviews must verify these hashes and return `GO-to-evidence`
or `HOLD`. No source edit is authorized before two GO results.
