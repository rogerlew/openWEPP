# EB-04W Gate Results

Evidence mode: **Ran + Static**.

| Gate | Result | Evidence |
|---|---|---|
| contract-first failing test | pass | authority passed and runtime failed before implementation |
| exact population freeze | pass | 4 unique lanes, 16 B/L/S/LS cells, 5 frozen operators |
| exact release execution and analysis | pass | binary `b50dd71cb00f24806193b98d73fc5444e836efac84ad5a4e0465d1e67c81fec9`; 16/16 return code zero; receipt `6f6bfe361c5b0aa155de1cfba61306e6d20fd570e68f67521eed12a3154dfbf7`; result `a44c3561cfea5bec64cc7514c4e3701d95111d8368ab96d8f0cb4784fcae6816`; `273.07 s`, exit `0` |
| phase closure | pass | maximum active fraction residual `1.1102230246251565e-16` |
| accumulation closure | pass | maximum residual `7.979727989493313e-17 m` |
| melt-component closure | pass | maximum residual `2.0274580625478933e-17 m` |
| pre-observed-peak mass closure | pass | maximum residual `2.9976021664879227e-15 m` |
| trace-to-WAT closure | pass | maximum SWE/depth residual `8.881784197001252e-16 m` |
| modeled redistribution boundary | pass | absolute modeled contribution sum `0 m`; physical contribution retained unknown |
| behavior neutrality | pass | self-bound result `b896b53ecb3787dd85fe46732a7154b22788c942af3f6c8093a96113859e7d0e`; 245,456 rows; 736,368 WAT values and 72,093,744 values across all 111 prior v2 trace fields; maximum numeric differences zero |
| figures and sidecars | pass | four SVGs parse, have same-stem Markdown sidecars, and passed montage visual inspection without material clipping or overlap |
| focused contract/runtime tests | pass | 15 integration tests plus focused semantic and snowbench replay tests |
| Rustfmt / affected Clippy | pass | warnings denied |
| assurance adoption/render/validate | pass | governed source lock adopted; rendered draft current |
| dependency/security | pass | `cargo deny check`; no new security boundary |
| unit registry | pass | 21/21 |
| SC unit compliance inventory | observational | retained repository inventory: 149 findings; no EB-04W unit bypass identified |
| Markdown | pass scoped / retained repository debt | recorded terminal package lint: 34 files, zero findings; closure re-lint after review/verification artifacts: 41 files, zero findings; three edited roadmap/catalog files also passed individually; full docs inventory reports 15 pre-existing broken-link errors outside the EB-04W write set |
| line count | pass with debt | no touched Rust file at or above 3,000 lines; three files remain in warning band |
| quick/frost/full exact-head suites | pass | quick `c2dcae3b…`: 2,143/2,143 passed in 2,208.697 s; frost `48827898…`: 341/341 passed in 529.018 s; full `803132f9…`: 2,192/2,192 passed in 2,216.531 s; all exit `0`; exact commands, HEAD, logs, and counts retained in `terminal-suite-summary.md` and `summary.json` |
| dual review/disposition | pass | Review A approves implementation and closes provenance; Review B accepts B-01 through B-05 remediation; terminal suites close the shared review hold |
| dual terminal verification | pass | independent `verification-agent-a.md` and `verification-agent-b.md`; both exact-terminal audits report no blockers |

## Invalidated Runs And Attempts

Nonterminal or superseded evidence is deliberately excluded:

- the first complete cohort preceded the trace-helper extraction and is retained
  under `invalidated-pre-terminal-helper-extraction/`;
- a partial four-cell restart used the prior release binary and retained no
  receipt or result;
- the exact pre-Clippy-helper cohort and the later pre-snowbench-phase-fix
  cohort are documented by their named invalidation artifacts;
- the combined b28e execution/analysis invocation completed its execution
  boundary but was interrupted during analysis and was later superseded; and
- two quick-suite starts were signal-terminated after new source findings and
  are marked invalid in their logs/sidecars.

The terminal receipt binds only the final corrected release binary shown above;
the fresh quick/frost/full bundle uses separate final-source evidence.
