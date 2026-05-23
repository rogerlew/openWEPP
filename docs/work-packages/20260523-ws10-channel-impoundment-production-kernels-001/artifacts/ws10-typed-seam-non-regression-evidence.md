# WS10 Typed-Seam Non-Regression Evidence

Status: `completed`
Evidence mode: `Ran`

## Regression Gate
Command:
```bash
cargo test --test parser_runtime_seam_integration
```
Result: pass (`45 passed`).

Interpretation:
- Existing parser->runtime seam closure across hillslope and watershed adapters
  remains non-regressed after WS10 production-kernel and runtime-projection
  additions.

## WS10 Runtime Projection Unit Evidence
Command:
```bash
cargo test -p openwepp-watershed-orchestrator
```
Result: pass (`26 passed`), including new WS10 seed tests:
- `watershed_channel_runtime_seed_projects_ws10_symbols`
- `watershed_channel_runtime_seed_rejects_out_of_domain_symbol`
- `watershed_impoundment_runtime_seed_projects_ws10_symbols`
- `watershed_impoundment_runtime_seed_rejects_h_above_hfull`
