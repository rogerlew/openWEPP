# Required Reading Map

Status: `UPDATED`

## Core

| Path | Purpose | Bytes | Status |
| --- | --- | ---: | --- |
| `AGENTS.md` | Root repository governance. | 9759 | `STATIC-SCAFFOLD` |
| `docs/work-packages/AGENTS.md` | Work-package governance and subagent authorization wording. | 12268 | `STATIC-SCAFFOLD` |
| `docs/standards/prompt-wording-guidance.md` | Execution prompt wording and required reading budget rules. | 7450 | `STATIC-SCAFFOLD` |
| `docs/work-packages/20260701-wshedperf01-watershed-baseline-performance-characterization-001/package.md` | Package objective, scope, and gates. | 10073 | `STATIC-SCAFFOLD` |
| `docs/work-packages/20260613-wshed01-watershed-routed-outputs-totalwatsed3-closure-001/package.md` | Prior arboreal-dendrite watershed CLI/output closure context. | 14539 | `STATIC-SCAFFOLD` |
| `docs/work-packages/20260701-wshedperf01-watershed-baseline-performance-characterization-001/prompts/active/wshedperf01_kickoff_agent_prompt.md` | Active prompt and exact execution constraints. | 4187 | `STATIC-SCAFFOLD` |

## Conditional

Read when command-surface or architecture context is needed.

| Path | Trigger | Bytes | Status |
| --- | --- | ---: | --- |
| `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs` | Exact openWEPP CLI args, output behavior, and command-surface inspection. | 73007 | `CONDITIONAL` |
| `docs/work-packages/20260616-perf-high-ofe-hillslope-characterization-001/package.md` | Hillslope baseline/perf lesson extraction. | 6312 | `CONDITIONAL` |
| `docs/work-packages/20260616-perfarch01-indexed-runtime-surface-design-001/package.md` | Architecture lessons from hillslope profiling. | 8663 | `CONDITIONAL` |
| `docs/work-packages/20260616-perfopt01-runtime-surface-map-churn-001/package.md` | Optimized-hillslope measurement and bit-identity gate patterns. | 7790 | `CONDITIONAL` |
| `docs/work-packages/20260630-direct-publication-streaming-sink-001/package.md` | Current direct publication/RSS performance endpoint context. | 3136 | `CONDITIONAL` |
| `docs/ROADMAP.md` | Current performance-track state and forward queue. | 60670 | `CONDITIONAL` |

## Budget

Static scaffold core byte total excluding this mutable map: `54195`.

Static scaffold core byte total including this map: `56724`.

Conditional context total if all listed conditional files are read: `159578`.

Maximum listed scaffold total: `216302`.

Threshold disposition: `OK` (`<=400000` bytes).

The worker should refresh this table if package edits materially change the
package-local required-reading files before disposition.
