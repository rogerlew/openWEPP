# Numerics Convergence Evidence

Status: executed-hold
Evidence mode: Static + Ran

D10 did not establish source-authorized convergence criteria. The diagnostic
resolution axes were executed so the HOLD is evidence-backed, but the source
authority needed to convert these diagnostics into acceptance thresholds is
missing.

Diagnostic D10 harness results:

| `cells` | `sample_dt_s` | `max_dt_s` | `NS_trace` | Peak ratio | `t_peak_s` | Rise 10-90 s |
|---:|---:|---:|---:|---:|---:|---:|
| 120 | 1.0 | 0.5 | 0.262677 | 0.836598 | 37.0 | 29.409 |
| 240 | 0.25 | 0.25 | 0.193296 | 1.069855 | 39.0 | 31.007 |
| 480 | 0.125 | 0.125 | 0.101244 | 0.844099 | 36.125 | 28.745 |

Comparator subagent exploratory sweep also showed non-monotone sensitivity
across isolated axes; see `d10-s0-case4-resolution-sweeps.json`.

Verdict: no convergence acceptance. Refinement changes peak magnitude and
timing without moving the method toward the reference trace under a named
source-backed tolerance.
