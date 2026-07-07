# Design Evidence

Status: **EXECUTED**. Evidence mode: **Static + Ran**.

## Baseline

Current held evidence from the parent package:

- Case-4 hybrid ladder peak errors: `22.8% / 15.5% / 10.2%`.
- Required parent tolerance: `<=5%` peak error at every rung.
- Baseline active hybrid H2637 timing/profile: `36.61 s` user,
  `274681460` implicit equilibrium map evaluations.

## Candidate Matrix

| Candidate | Authority class | Result |
|---|---|---|
| Current source-free predicate | Current `SC-OFEROUTE-002` rev 2 | Fails Case-4 ladder. |
| Explicit cool-down after source-off | Recorded I0 fallback; amended as source-memory cooldown in `SC-OFEROUTE-002` rev 3 | SELECTED |
| State-aware spatial wave-quiet predicate | Non-binding assessment candidate in rev 2; requires contract amendment and evidence | Not needed for this package after cooldown passed Case-4. |

## Cooldown Scan

Ran by `comparator_suite_runner` against the exploratory D-val harness:

```
OPENWEPP_IWAGAKI_HYBRID_COOLDOWN_S=<value> cargo nextest run -p openwepp-hillslope-orchestrator 'ofe_routing::d10b_reconciliation_tests::case4_hybrid_manning_ladder_meets_iwagaki_oracle' --profile quick --run-ignored ignored-only
```

Results:

| Cooldown after `10 s` source | Result | Peak-error ladder |
|---|---|---|
| `0 s` | FAIL | `22.8% / 15.5% / 10.2%` |
| `5 s` | FAIL | `18.8% / 12.4% / 8.0%` |
| `10 s` | FAIL | `13.1% / 8.1% / 5.03%` |
| `20 s` | PASS | Accepted by the retained ladder. |

Interpretation:

- A cooldown equal to the source duration is still insufficient.
- A cooldown equal to `2 * source_duration` passes the current Case-4 ladder.
- The selected production predicate is source-memory based: after any
  contiguous source-active burst, route the next `2 * burst_duration`
  source-free bins explicitly before allowing implicit recession.

Artifact: `verification-case4-cooldown-scan.md`.

## Post-Implementation Focused Evidence

Ran locally:

```
cargo check -p openwepp-hillslope-orchestrator
cargo nextest run -p openwepp-hillslope-orchestrator hybrid_source_memory --profile quick
cargo nextest run -p openwepp-hillslope-orchestrator case4_hybrid_manning_ladder_meets_iwagaki_oracle --profile quick
cargo nextest run -p openwepp-hillslope-orchestrator ofe_routing --profile quick
```

Results:

- `cargo check`: PASS.
- Source-memory focused tests before review-response amendments: PASS, `2`
  tests.
- Source-memory focused tests after review-response amendments: PASS, `4`
  tests.
- Retained Case-4 hybrid ladder: PASS, `1` test in `144.342 s`.
- Focused `ofe_routing` slice before review-response amendments: PASS,
  `88` tests in `153.465 s`.

## H2637 Timing/Profile

Ran by `comparator_suite_runner` after the retained Case-4 ladder passed:

- `user`: `37.96 s`
- `system`: `0.02 s`
- `wall`: `0:37.99`
- `solver_steps`: `7381407`
- `solver_steps_implicit`: `980804`
- `implicit_equilibrium_map_evaluations`: `151435969`
- `implicit_branch_evaluations`: `20110816`
- `alpha_evaluations`: `119746485`

Baseline deltas:

- vs rev-31 active hybrid `36.61 s`: `+1.35 s` (`+3.69%`)
- vs plain-active `37.9 s`: `+0.06 s` (`+0.16%`)

Artifact: `verification-h2637-timing.md`.
