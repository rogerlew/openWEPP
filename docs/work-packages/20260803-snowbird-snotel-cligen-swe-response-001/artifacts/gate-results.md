# Gate Results

| Gate | Result | Evidence |
|---|---|---|
| Frozen inputs and binaries | `PASS` | Source climate, SNOTEL record, Alta station, CLIGEN, openWEPP binary, period, variants, guards, and roles were frozen before execution. |
| Climate generation | `PASS` | Terminal-v2 generated four 12,784-row `.prn`/`.cli` pairs; receipt binds hashes, eligible assignments, effective quantized changes, and realized mediated column changes. |
| Real consumer execution | `PASS` | All five cells completed through the snowbench direct-production executor. |
| Snow operators and closure | `PASS` | Peak SWE/date, melt-out, input/storage/loss, and closure reported; maximum closure is `1.221e-15 m`. |
| Claim discipline | `PASS` | Calibration-only evidence; no correction, validation, transferability, provider, or promotion claim. |
| Direct syntax/JSON/Markdown/diff/hygiene | `PASS` | Python AST, three JSON files, 16-file Markdown lint, diff, protected paths, overwrite, and bytecode checks pass. |
| Independent review | `PASS` | Both terminal-v2 exact-current re-reviews pass after accepted remediation. |
| Terminal verification | `PASS` | Both exact-current terminal-v2 verifiers independently reconstructed results, identities, mediated changes, and direct gates. |
