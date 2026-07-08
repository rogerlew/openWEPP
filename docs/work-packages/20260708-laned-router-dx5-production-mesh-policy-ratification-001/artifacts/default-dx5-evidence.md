# Default DX5 Runtime Evidence

Status: PASS. Evidence mode: Ran.

Release binary:

- Build: `cargo build --release -p openwepp-runner --bins`
- Path: `target/release/openwepp-cli-hill`
- SHA256: `3f60d8bd064a11c514edd1558951051782f2e757f4ce71ce4b2e7be292c9524b`
- Git HEAD: `ed32fa4175196d6b8b8055e36e4f5a51ee3787de`
- Git status short:

```text
M crates/openwepp-hillslope-orchestrator/src/direct_runtime/laned_active.rs
 M crates/openwepp-runner/src/hillslope/laned_active.rs
 M docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md
?? docs/work-packages/20260708-laned-router-dx5-production-mesh-policy-ratification-001/
```

| Member | Mode | Status | Wall | User | Mesh mode | Target dx | Max dt | Steps | Trace rows | Clamp m3 | Max seam residual | HBP | Pass parquet |
|---|---|---:|---:|---:|---|---:|---:|---:|---:|---:|---:|---|---|
| mn_corn_h4 | active_default_dx5 | PASS | 0:00.74 | 0.72 | target_dx | 5.0 | 300.0 | 51324 | 2557 | 0.0 | 5.092454815097145e-15 | `2f200c2ee0ad` | `a364287f6fe3` |
| mn_corn_h4 | active_explicit_dx5 | PASS | 0:00.77 | 0.75 | target_dx | 5.0 | 300.0 | 51324 | 2557 | 0.0 | 5.092454815097145e-15 | `2f200c2ee0ad` | `a364287f6fe3` |
| mn_corn_h4 | off_default | PASS | 0:00.43 | 0.40 | n/a | n/a | n/a | n/a | n/a | n/a | n/a | `2f200c2ee0ad` | `a364287f6fe3` |
| mn_corn_h4 | off_mesh_env_control | PASS | 0:00.40 | 0.38 | n/a | n/a | n/a | n/a | n/a | n/a | n/a | `2f200c2ee0ad` | `a364287f6fe3` |
| n_idaho_forest_h1 | active_default_dx5 | PASS | 0:21.79 | 21.76 | target_dx | 5.0 | 300.0 | 403843 | 1461 | 0.0 | 6.427411888066303e-15 | `c5c970de6154` | `0ccbf4c24703` |
| n_idaho_forest_h1 | active_explicit_dx5 | PASS | 0:22.07 | 22.04 | target_dx | 5.0 | 300.0 | 403843 | 1461 | 0.0 | 6.427411888066303e-15 | `c5c970de6154` | `0ccbf4c24703` |
| n_idaho_forest_h1 | off_default | PASS | 0:00.40 | 0.38 | n/a | n/a | n/a | n/a | n/a | n/a | n/a | `891086a4359b` | `db67428fed2d` |
| n_idaho_forest_h1 | off_mesh_env_control | PASS | 0:00.37 | 0.35 | n/a | n/a | n/a | n/a | n/a | n/a | n/a | `891086a4359b` | `db67428fed2d` |
| wa_cascades_forest_h1 | active_default_dx5 | PASS | 1:05.37 | 65.30 | target_dx | 5.0 | 300.0 | 4891877 | 10960 | 7.305156020320419e-13 | 4.832475752036399e-14 | `800176e73bff` | `c9a516cfe6eb` |
| wa_cascades_forest_h1 | active_explicit_dx5 | PASS | 1:04.69 | 64.62 | target_dx | 5.0 | 300.0 | 4891877 | 10960 | 7.305156020320419e-13 | 4.832475752036399e-14 | `800176e73bff` | `c9a516cfe6eb` |
| wa_cascades_forest_h1 | off_default | PASS | 0:01.35 | 1.32 | n/a | n/a | n/a | n/a | n/a | n/a | n/a | `d0cdfb6d4a2e` | `860ca7f61169` |
| wa_cascades_forest_h1 | off_mesh_env_control | PASS | 0:01.39 | 1.35 | n/a | n/a | n/a | n/a | n/a | n/a | n/a | `d0cdfb6d4a2e` | `860ca7f61169` |

Identity comparisons:

| Member | Comparison | Status | Mismatches |
|---|---|---:|---:|
| mn_corn_h4 | active_default_vs_explicit_dx5 | PASS | 0 |
| mn_corn_h4 | off_default_vs_mesh_env_control | PASS | 0 |
| n_idaho_forest_h1 | active_default_vs_explicit_dx5 | PASS | 0 |
| n_idaho_forest_h1 | off_default_vs_mesh_env_control | PASS | 0 |
| wa_cascades_forest_h1 | active_default_vs_explicit_dx5 | PASS | 0 |
| wa_cascades_forest_h1 | off_default_vs_mesh_env_control | PASS | 0 |

Mesh policy assertions:

| Member | Mode | Status |
|---|---|---:|
| mn_corn_h4 | active_default_dx5 | PASS |
| mn_corn_h4 | active_explicit_dx5 | PASS |
| mn_corn_h4 | off_default | PASS |
| mn_corn_h4 | off_mesh_env_control | PASS |
| n_idaho_forest_h1 | active_default_dx5 | PASS |
| n_idaho_forest_h1 | active_explicit_dx5 | PASS |
| n_idaho_forest_h1 | off_default | PASS |
| n_idaho_forest_h1 | off_mesh_env_control | PASS |
| wa_cascades_forest_h1 | active_default_dx5 | PASS |
| wa_cascades_forest_h1 | active_explicit_dx5 | PASS |
| wa_cascades_forest_h1 | off_default | PASS |
| wa_cascades_forest_h1 | off_mesh_env_control | PASS |

Closure assertions:

| Member | Mode | Status |
|---|---|---:|
| mn_corn_h4 | active_default_dx5 | PASS |
| mn_corn_h4 | active_explicit_dx5 | PASS |
| n_idaho_forest_h1 | active_default_dx5 | PASS |
| n_idaho_forest_h1 | active_explicit_dx5 | PASS |
| wa_cascades_forest_h1 | active_default_dx5 | PASS |
| wa_cascades_forest_h1 | active_explicit_dx5 | PASS |

Detailed JSON:

- `/home/workdir/openWEPP/docs/work-packages/20260708-laned-router-dx5-production-mesh-policy-ratification-001/artifacts/default-dx5-evidence.json`
