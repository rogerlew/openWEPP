# Correction Authority Envelope

Status: executed-hold
Evidence mode: Static + Ran

## Defects

- `SC-OFEROUTE-001#GAP-OFEROUTE-005`: Iwagaki Case-4 shock sampled-hydrograph
  and resolution sensitivity.
- `SC-OFEROUTE-001#INV-OFEROUTE-011` Case-4 residual only.
- H2637 runtime shadow reproduction of the same sampled-handoff/resolution
  sensitivity class.

## In-Scope Files

- `SC-OFEROUTE-001`
- D10 package artifacts
- `tools/dval/compare_dval.py`
- Read-only solver/cascade inspection under `ofe_routing`

## Protected Boundaries

- No production/default activation.
- No `OPENWEPP_LANED_SHADOW` activation work.
- No D11 friction sourcing/default promotion.
- No D12 melt-limb work.
- No D13 ADR-0036 erosion-shape implementation.
- No surrogate/provisional process physics.

## Seven DC Gates

| Gate | D10 result | Evidence |
|---|---|---|
| Reproduction | Pass | Case 4 and H2637 reproduced. |
| Named mechanism | Pass | Reduced-KWE TVD limiter/handoff/friction-mapping authority boundary. |
| Ownership | Pass | Lane D solver/cascade/harness/contract are in D10 scope. |
| Authority | Fail | Primaries do not bind the reduced implementation and Iwagaki `n` to D-val `k_o`. |
| Safety | Fail for production edit | A limiter-branch trial regressed Case 4 and focused tests. |
| Testability | Partial | D10 added Case-4 resolution controls; no source-backed acceptance rule exists. |
| Validation | Pass for HOLD | Required diagnostics and gates recorded; correction validation not authorized. |

Final envelope: `EXECUTED-HOLD-SOURCE-AUTHORITY`.
