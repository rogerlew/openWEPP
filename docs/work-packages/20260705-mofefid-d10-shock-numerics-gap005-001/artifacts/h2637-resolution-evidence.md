# H2637 Resolution Evidence

Status: executed
Evidence mode: Ran + Static

Runner: comparator subagent Russell.

Command:

```bash
cargo nextest run -p openwepp --test laned_shadow_h2637 h2637_executed_vector_shadow_on_off
```

Result: PASS.

Metrics from `artifacts/d10-s0-h2637-shadow-evidence.json`:

| Metric | Value |
|---|---:|
| `days_seen` | 731 |
| `days_routed` | 610 |
| `days_uniform_shape` | 10 |
| `max_supply_reconstruction_rel` | 5.636938e-16 |
| `aggregate_router_conservation_rel` | 0.1047607953 |
| `max_router_conservation_rel` | 0.6110480464 |
| `total_source_m3` | 1754945.2321 |
| `total_routed_outlet_m3` | 1658805.6614 |

Verdict: diagnostic/hold-supporting only. H2637 proves the production-shaped
shadow path still reproduces the sampled-handoff/resolution class, but the
shadow manifest block is not an acceptance surface and no parameterized H2637
resolution-sweep CLI exists in D10.
