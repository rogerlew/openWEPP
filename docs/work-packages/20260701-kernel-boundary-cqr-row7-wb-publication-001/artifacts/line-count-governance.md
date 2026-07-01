# Line-Count Governance

Evidence mode: Static + Ran.

Ran:

```text
wc -l crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs crates/openwepp-runner/src/hillslope/03_tests.rs crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs crates/openwepp-runner/src/hillslope/direct_seed_projections/01_wb12_wb16_wb19_projection.rs
```

Result:

```text
302 crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs
1235 crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs
1716 crates/openwepp-runner/src/hillslope/03_tests.rs
4137 crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs
726 crates/openwepp-runner/src/hillslope/direct_seed_projections/01_wb12_wb16_wb19_projection.rs
8116 total
```

## Disposition

Status: `PASS-WARN`.

`00_builders_and_authority.rs` remains above the 3000-line governance threshold.
The file was already a direct-publication monolith before row #7, and row #7's
primary objective was CRAP closure with H2637 byte identity. A broad module split
inside this CQR row would enlarge the behavior-preserving blast radius and
increase marker-contract risk beyond the row's write set.

Accepted exception:

- Owner: openWEPP maintainers / Codex package executor.
- Scope: row #7 only; no new physics, schema, or runtime-selection behavior is
  authorized by this exception.
- Sunset: before adding new production behavior to
  `00_builders_and_authority.rs`, or in the next direct-publication CQR/refactor
  package that touches this file for reasons beyond test-only coverage, split
  the file into smaller marker-safe modules.
- Guard: row #7 still passed full gates, refreshed CRAP-after, and H2637
  protected-output identity with `compatibility_edge_invocations=0`.
