# Gate Results

| Gate | Result | Evidence |
|---|---|---|
| Source identity | `PASS` | Climate, normalized observation, and provenance hashes match the frozen manifest. |
| Cumulative precipitation operator | `PASS` | Same-water-year consecutive-date differences only; values below `-1e-9 mm` and gaps fail closed. |
| Boundary-interval operator | `PASS` | Eight matched October-1-to-September-30 boundary differences cover October 2 through September 30 and are not labeled complete years. |
| Temperature comparison | `PASS` | Finite exact-date Tmax/Tmin populations are reported separately with signed bias, MAE, and correlation. |
| Claim discipline | `PASS` | Observation remains diagnostic-only; no correction, causation, provider, or snow-improvement claim. |
| Direct syntax/JSON/Markdown/diff/hygiene | `PASS` | Python AST, both JSON files, 16-file Markdown lint, diff, protected-path, overwrite, and no-bytecode checks pass. |
| Independent review | `PASS` | Both fresh exact-current re-reviews pass after accepted remediation. |
| Terminal verification | `PASS` | Both exact-current verifiers independently regenerated and reconstructed the result and passed all direct gates. |
