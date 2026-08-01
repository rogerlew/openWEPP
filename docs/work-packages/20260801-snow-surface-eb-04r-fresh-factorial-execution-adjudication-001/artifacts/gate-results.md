# Gate Results

Status: `HOLD`

Evidence class: `Ran`

| Gate | Result | Evidence |
| --- | --- | --- |
| Python syntax and prospective self-check | PASS | Sanitizer, immutable population, signed controls, and retained-consumer mutations passed before freeze. |
| Result-bearing execution | PASS | One attempt; 48/48 subprocesses; 761,212 retained daily rows. |
| Environment/file provenance | PASS | Exact seven-key mappings and all result-bearing identities reconcile. |
| Implemented physical consumer | PASS / not dispositive | WAT/trace/layer, energy, sign, finite-number, and selector checks pass under implemented thresholds. |
| Frozen vapor-to-sublimation gate | **FAIL** | 12 cells exceed frozen `1e-9 kg m^-2`; maximum `8.109983287707401e-8`. Consumer used contradictory `1e-6`. |
| Observation-access gate | **FAIL** | Observations loaded after consumer PASS but before reconciliation to the stricter frozen protocol. Scores/effects are inadmissible. |
| Deterministic regeneration | FAIL / non-closure | All generated files except `factorial-results.json` were byte-identical. Concurrent Critical build changed the mutable analysis-binary hash in that report; frozen execution identity remains retained. This cannot cure the science HOLD. |
| Authority anti-evasion | PASS | Source guard and 3/3 required-suite guards, run `552deba5-2fbf-4cca-9c20-bdccef3e01fa`. |
| Formatting and diff hygiene | PASS | `cargo fmt --all -- --check`; `git diff --check`. |
| Markdown/SVG | PASS | 37 Markdown files, zero findings; 7/7 SVG/sidecar pairs parse and carry HOLD warnings. |
| Production/test diff | PASS | Empty binary diff, SHA-256 `e3b0c442…`. |
| Security/write set | PASS | Docs/package-only; no dependency, network, secret, unsafe, external write, or public-schema change. |
| Critical full workspace | PASS | 2,177/2,177 passed, 29 skipped, 47 slow; run `cbfb36c6-8490-40a4-8e89-a000eb36e645`; 2,943.021 s. |

Quick, frost, focused snow-energy, and strict Clippy evidence remain reusable
from the exact unchanged production/test tree and executed binary source. A
passing software regression suite cannot override the failed frozen science
gate.
