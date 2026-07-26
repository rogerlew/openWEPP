# Gate Evidence

Evidence class: `Ran`

| Gate | Result | Evidence |
| --- | --- | --- |
| CAL-01 bounded admission | PASS | Exact dependency identities remain current. |
| Authorized soil reconstruction | PASS | Mukeys 665220 and 131976 serialized through WEPPpy as `2006.2`; hashes retained. |
| Harness focused tests | PASS | Ten unittest cases passed, including perennial transfer reconstruction. |
| Exact executable identity | PASS | Pre/post SHA-256 is `6104a344...47fc3f`. |
| Five-arm matrix | PASS | Five exit-zero runs; 36,525 daily rows and years 1--100 per arm; empty stderr. |
| Daily/process parsing | PASS | Pool aliases remain separate; fixed-width overflow is null plus an explicit flag. |
| Linux 260725 / source-native 9002 matrix | PASS | Five exit-zero 100-year arms, empty stderr, 36,525 crop rows each, unchanged release hash, hourly UI activation, and retained Observe manifests. |
| Annual litter/transfer reconstruction | PASS / BOUNDED | Pinned `grow.for` `delvd` authority plus crop-output `vdmt` reconstruct gross transfer; published biomass is rounded to `0.001 kg/m2`, and independent daily decline differs by at most `0.00088 kg/m2/year`. The operator explicitly accepted this precision for campaign use. |
| Practical equilibrium | MIXED / supports verdict | Hubbard 0.95 passes; Hubbard 0.92 and Santee mixed exceed the 2% range/mean rule and are not represented as accepted equilibrium. |
| Daily-runoff return periods | PASS | Empirical descending ranks 50, 20, 10, 5, 4, and 2 reconstructed from 100 simulation years. |
| Peak-rate return periods | PASS | `.element.dat` `PeakRO` parsed and scored at the same empirical ranks. |
| Report comparison | PASS | Rebuild script scores 64 Windows process/hydrology/daily/peak targets with frozen tolerances and the bounded soil label; Santee's 40-year chart uses years 31--40. |
| Deterministic figure | PASS | Rebuild script emits `figures/equilibrium-stocks.svg` from normalized equilibrium results. |
| Security/path confinement | PASS | Host/root allowlists, regular-file checks, and pre/post executable hashes enforced. |
| Diff hygiene | PASS | Focused tests, deterministic rebuild, JSON parsing, and `git diff --check` pass. |
| Dual review | PASS | Both reopened-lane reviewers independently passed the reconciled terminal evidence. |
| Dual verification | PASS | Both reopened-lane verifiers independently passed the terminal evidence and deterministic rebuild. |

Scientific target failures are evidence for the top-level
`NOT_REPRODUCIBLE` verdict, not failed execution gates. No current execution
or evidence gate remains blocked before final review.
