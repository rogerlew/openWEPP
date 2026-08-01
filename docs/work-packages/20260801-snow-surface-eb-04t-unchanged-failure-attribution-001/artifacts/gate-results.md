# Gate Results

Evidence mode: `Static + Reused Ran`.

| Gate | Result | Evidence |
|---|---|---|
| Frozen inputs | PASS | Six exact input hashes recorded in `failure-attribution.json`. |
| Failure inventory | PASS | Exactly 16; signature counts `9/2/2/2/1`. |
| B/L/S/LS labels | PASS | All 64 relevant labels remain `fail`. |
| Primary metric reconstruction | PASS | Every row retains value, target, error magnitude, and factorial effects. |
| Numerical-vs-label distinction | PASS | Zero full metric objects are exact B/LS matches; 15 selected primary errors away, one unchanged. |
| Open-control timing subset | PASS | Five sublimation-sensitive open controls: four away, one unchanged; zero identify canopy longwave. |
| Density/geometry subset | PASS | Eleven rows, including two mixed/ambiguous depth-SWE guards: all away. |
| Factorial interaction | PASS | Thirteen nonzero: 12 mitigate additive error and one amplifies it; Niwot cancellation is explicit. |
| Materiality boundary | PASS | Exact signs only; no practical-materiality threshold or claim. |
| Rejected alias | PASS | Mutated baseline failure inventory is rejected. |
| Deterministic regeneration | PASS | JSON, CSV, synthesis, SVGs, and sidecars hash-identical across rerun. |
| Model execution | PASS | Zero subprocess/model calls; retained evidence only. |
| Figures | PASS | Three SVGs parse; three Markdown sidecars present and timing stem uses open-control terminology. |
| Scoped Markdown | PASS | 24 package Markdown files plus roadmap/catalog files: zero findings. |
| Whitespace | PASS | `git diff --check`. |
| Protected diff | PASS | No production, test, contract, fixture, prior-package, or retained-output changes. |
| Independent reviews | PASS | Two reviews completed; all findings accepted and resolved. |
| Terminal verification | PASS | Two independent verifiers reproduced inventory, directions, interactions, hashes, claim boundaries, and deterministic outputs. |
| Exact write set | PASS | Package tree plus three declared roadmap/catalog files only. |

Disposition: `DIAGNOSTIC_COMPLETE`.
