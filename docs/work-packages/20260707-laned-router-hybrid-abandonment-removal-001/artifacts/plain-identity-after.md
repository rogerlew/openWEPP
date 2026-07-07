# Plain Identity After

Status: PASS. Evidence mode: Ran.

Release binary:

- Path: `target/release/openwepp-cli-hill`
- SHA256: `11cb3d49f74c1b00966d9fd41b2dba6077313f6dc9919f56ded526155182c43a`
- Git HEAD: `b1d5fd4410b700012d857ef4056000163e6aa6a0`
- Git status short:

```text
 M crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs
 M crates/openwepp-hillslope-orchestrator/src/direct_runtime/laned_active.rs
 M crates/openwepp-hillslope-orchestrator/src/ofe_routing.rs
 M crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs
 M crates/openwepp-hillslope-orchestrator/src/ofe_routing/d10b_reconciliation_tests.rs
 M crates/openwepp-hillslope-orchestrator/src/ofe_routing/dval.rs
 D crates/openwepp-hillslope-orchestrator/src/ofe_routing/implicit_recession.rs
 M crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs
 M crates/openwepp-hillslope-orchestrator/src/ofe_routing/profile.rs
 M crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs
 M crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs
 M crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs
 M crates/openwepp-runner/src/hillslope/laned_active.rs
 M docs/decisions/README.md
 M docs/numerics/README.md
 M docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md
 D docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md
 M docs/specifications/science-contracts/index.md
 M tests/integration/laned_shadow_h2637.rs
?? docs/decisions/0037-abandon-hybrid-implicit-stepping.md
?? docs/numerics/kinematic-wave-equilibrium-rating-z-structure.md
?? docs/work-packages/20260707-laned-router-hybrid-abandonment-removal-001/
```

| Member | Status | Wall | User | Sys | HBP SHA256 | Pass parquet SHA256 |
|---|---:|---:|---:|---:|---|---|
| h2637 | PASS | 0:39.91 | 39.86 | 0.03 | `efd8c4255fbe976ecafb2bc89defb7bebd4e2054c9e65c89cd5353c4c31c3790` | `21c54bf2b045c3fb2f79f39ca174e36a4d188b39f7064f2a75f1170be6bb1656` |
| mn_corn_h4 | PASS | 0:00.55 | 0.53 | 0.01 | `2f200c2ee0ad4f1b581d6d95aafe7bc2ff2ba5368afa96846263ea86b5243e18` | `a364287f6fe348f609d25f341823781fdb6885607644eb531050ba1abbf5084f` |
| n_idaho_forest_h1 | PASS | 0:00.98 | 0.96 | 0.02 | `5ccf8c4edb1bacb862b92161171b35fb0790df263424a47647ca3df47e52a394` | `be510725f5bd7bc92c2cb86742d352c7931e02831ef85853093d83e4e2726c77` |
| wa_cascades_forest_h1 | PASS | 0:15.60 | 15.57 | 0.02 | `3640fdf3b3c1d3bf61189a9430fe268143ce9db0e1996cb89e614cfd4d5c4f23` | `bb3b2e03f3fbd5834eb65a06c59476aba8a383bb021e9f669be7825a342f9e63` |

Pre/post comparison:

| Member | HBP identical | Pass parquet identical |
|---|---:|---:|
| h2637 | True | True |
| mn_corn_h4 | True | True |
| n_idaho_forest_h1 | True | True |
| wa_cascades_forest_h1 | True | True |

Detailed JSON: `/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-hybrid-abandonment-removal-001/artifacts/plain-identity-after.json`
