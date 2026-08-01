# Gate Results

Status: `PASS`

Evidence class: `Ran`

| Gate | Result | Evidence |
| --- | --- | --- |
| Python syntax/self-check | PASS | frozen tool and anti-alias vectors; independent verifier AST parses |
| Population execution | PASS | 48/48; 24/24 former failures; 612.944 s |
| Physical reconstruction | PASS | 761,212 daily rows; all declared bounds |
| Independent retained-output consumer | PASS | exact 48-cell key set; WAT/trace/layer identity; finite/coupled state; three mutation controls |
| Deterministic regeneration | PASS | two byte-identical reductions |
| Figure/sidecar inventory | PASS | 4/4 SVG/Markdown pairs |
| Focused contract/runtime | PASS | 27/27; run `226d805f-5794-4739-82cd-40b1a04a7618` |
| Authority anti-evasion | PASS | source guard plus 3/3 required-suite guards; run `7933cc76-5e1e-481b-a836-aac87be9f708` |
| Formatting | PASS | `cargo fmt --all -- --check` on the current tree |
| Strict Clippy | PASS / reused | exact executable HEAD and empty production/test diff; EB-04D exact-source receipt |
| Workspace quick | PASS / reused | 2,128/2,128; run `791467e8-8f07-42b8-9712-5c60c69fd709` |
| Workspace frost | PASS / reused | 329/329; run `61486d3b-760a-43d9-914c-f49252e282d6` |
| Critical full workspace | PASS | 2,177/2,177; 29 skipped; run `0fe1fcd2-de9f-431d-bf96-efaaa199b92e`; 2,194.517 s |
| Markdown/SVG | PASS | 37 Markdown files, zero findings; 4/4 SVGs parse with 4/4 sidecars |
| Diff/security | PASS | `git diff --check`; empty `crates/tests` diff; no dependency/network/secret/unsafe/public-schema change |

Quick, frost, and strict Clippy evidence are reused under the canonical testing
strategy's exact-evidence rule. EB-04E executes at committed source
`44c6c9cc2e4447064fbbbf70935cf581d60d49b0`, the same executable tree as the
receipts, and its terminal production/test diff is empty. Package narrative and
package-local analysis tools cannot affect those executable results. Critical
full is nevertheless being run afresh because the package explicitly assigns
that campaign gate to the comparator runner. It completed with zero failures or
errors; the retained JUnit is `target/nextest/full/junit.xml`.
