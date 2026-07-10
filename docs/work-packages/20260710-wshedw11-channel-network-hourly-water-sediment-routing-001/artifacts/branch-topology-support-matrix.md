# Branch and Topology Support Matrix

Status: `EXECUTED`

Evidence mode: `Static`.

| Branch/topology | Water series authority | Sediment series authority | W11 disposition |
|---|---|---|---|
| `ipeak=1` Rational | No routed outlet series | None | Preserve minor-0/M-T3 leaf behavior; fail closed for hourly dependency claim. |
| `ipeak=2` CREAMS | No routed outlet series | None | Preserve minor-0/M-T3 leaf behavior; fail closed for hourly dependency claim. |
| `ipeak=3` kinematic wave | Baseline `wshchr` | Missing interval sequencing | Water implementation-ready; W11 held on sediment authority. |
| `ipeak=4` Muskingum-Cunge | Baseline `wshchr` | Missing interval sequencing | Water implementation-ready; W11 held on sediment authority. |
| `ipeak=5` variable Muskingum-Cunge | Baseline `wshchr` | Missing interval sequencing | Water implementation-ready; W11 held on sediment authority. |
| `ipeak>5` | Falls through in source but lacks named semantic authority | None | Fail closed for W11 hourly claim. |
| channel -> channel | Direct upstream `q1` superposition | Event-only class handoff | Water authorized; sediment held. |
| local hourly hillslope + upstream hourly channel | Additive same-grid water | Missing coupled interval class/state rule | Water authorized; sediment held. |
| impoundment boundary | Separate Chapter-14 state | Separate trapping/storage authority | Explicit W11 exclusion and typed failure. |
