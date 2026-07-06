# Conservation and Output Lineage

Status: executed
Evidence mode: Static + Ran

No production solver/cascade edit was landed. This lineage records the D10
acceptance boundary before any future correction may modify conservation-
sensitive routing code.

| Operand / output | Unit | Lineage | D10 status |
|---|---:|---|---|
| Lateral source rate `v` / Case-4 supply | m s^-1 | Iwagaki primary has three lateral-supply sections; D-val uses the Papanicolaou-derived Case-4 setup. | Authoritative as a source shape, not sufficient alone for acceptance. |
| Upstream boundary hydrograph | m^2 s^-1 | `ofe_routing::cascade` samples the upstream OFE outlet hydrograph into the next OFE boundary. | Diagnostic boundary implicated; no production change. |
| Outlet hydrograph samples | m^2 s^-1 | `compare_dval.py` reads `dval_case` samples and compares to Figure 4 Enhanced_WEPP. | Diagnostic/validation surface; Case 4 does not pass. |
| Sample interval | s | D10 harness controls `--sample-dt` for Case 4 only. | Diagnostic resolution axis. |
| Max sub-step | s | D10 harness controls `--max-dt` for Case 4 only. | Diagnostic resolution axis. |
| Cell count | count | D10 harness controls `--cells` for Case 4 only. | Diagnostic resolution axis. |
| Iwagaki friction | Manning `n=0.009` in primary | Iwagaki primary names Manning `n`; D-val uses `k_o`. | Source-authority gap; not tuned in D10. |
| H2637 shadow ledger | m^3 and relative residuals | Runtime shadow manifest records supply reconstruction, routed outlet, and router conservation diagnostics. | Production-shaped diagnostic; not acceptance. |

Rejected aliases/formulas:

- D10 rejected treating the D-val `k_o` scan as an acceptance/tuning route; D11
  owns friction operand sourcing and defaults.
- D10 rejected treating H2637 run-level conservation diagnostics as proof of
  production acceptance; the shadow block is diagnostics-only.
- D10 rejected a source-shaped limiter-branch flip after focused tests showed
  worse Case-4 behavior and conservation regressions.
