# Terminal Review Disposition

Evidence mode: `Static + Ran`.

| Finding ID | Source | Severity | Decision | Action taken | Evidence |
|---|---|---:|---|---|---|
| `TA-H1` | terminal A | high | accepted | Added pre-score bindings for current tool/protocol, source trees, fixtures, observations, roles/filters, selectors, and decision dependencies to the executed attempt; regenerated analysis. | `adjudicate_retained.py`; `retained-adjudication.json` |
| `TA-M1` | terminal A | medium | accepted | Added EB-04R's stricter `1e-12 kg m^-2` vapor-aggregation gate before population acceptance. | Maximum `7.993605777301127e-15`; PASS. |
| `TA-L1` | terminal A | low | accepted | Removed the erroneous fifth Phase-A whitelist row from the required-reading map. | `required-reading-map.md` |
| `TB-H1` | terminal B | high | accepted | Same complete frozen-input repair as `TA-H1`; all identities match. | regenerated retained adjudication |
| `TB-M1` | terminal B | moderate | accepted | Same stricter aggregation gate as `TA-M1`. | conservation and gate evidence |
| `TB-M2` | terminal B | moderate | accepted | Added terminal `--verify-seal`; truthfully distinguishes prospective Phase-A self-check from post-amendment verification. | authority tool output and gate table |
| `TB-L1` | terminal B | low | accepted | Same whitelist map correction as `TA-L1`. | required-reading map |

All findings are accepted and repaired. The regenerated outcome remains
`CLOSE_NONPROMOTION_EMPIRICAL_RULE`.
