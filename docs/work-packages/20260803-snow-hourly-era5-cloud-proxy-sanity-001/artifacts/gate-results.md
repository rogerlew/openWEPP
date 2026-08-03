# Gate Results

| Gate | Result | Evidence |
|---|---|---|
| Four-site cloud comparison | `PASS` | Receipt reports four complete site lanes with exact source/comparator/climate hashes. |
| Proxy inversion/domain/constancy | `PASS` | Reconstructed cloud remains in `[0,1]`; maximum within-day range is below `6.6e-14`. |
| Complete chronology and wet-event populations | `PASS` | Complete days are 14,244 or 16,436; wet events are 3,120/4,517/2,464/2,247. |
| Protected precipitation | `PASS` | Receipt records `precipitation_modified=false`; comparator precipitation only selects days. |
| Python/JSON/Markdown | `PASS` | Python AST compilation, both JSON parses, and scoped `markdown-doc` lint pass. |
| Diff/protected paths/bytecode | `PASS` | `git diff --check`, empty production/test diff, and package no-`__pycache__` checks pass. |
| Independent review | `PASS` | Both fresh exact-current reviews pass after accepted remediation. |
| Terminal verification | `PASS` | Both exact-current verifiers regenerated the result and passed science, binding, lifecycle, and direct gates. |
