# REFINTENT001 Operand Lineage

Evidence class: Static + Ran

## Authority

- `SC-SUBHYD-001#INV-SUBHYD-032` is the active authority for WB14 `ksatadj`.
- Source intent is the top-two-tillage WB14 algorithm from the pinned baseline:
  `/workdir/wepp-forest_260430_baseline/src/infpar.for`.
- This package does not change the contract. It implements the already-ratified
  `avsat/(avpor*avcpm)` saturation fraction and preserves the existing 9001,
  9002+, and 9003 effective-conductivity branches.

## Projected operands

| Source-intent term | openWEPP runtime source | Use |
|---|---|---|
| `st_i` | `wb18_perc_theta_####` | top-two layer storage, summed as `theta_storage` |
| `tillay(2)` / `dg_i` | `wb19_dg_####` preferred, `dg_####` legacy | top-two tillage depth and weights |
| `por_i` | `wb19_por_####` preferred, `por_####` legacy | `avpor = sum(por_i * dg_i) / tillage_depth` |
| `cpm_i` | `cpm_####` | `avcpm = sum(cpm_i * dg_i) / tillage_depth` |
| `thetfc_i` | `wb19_thetfc_####` preferred, `thetfc_####` legacy | `avthetafc` |
| `thetdr_i` | `wb19_thetdr_####` preferred, `thetdr_####` legacy | `avsm15` and `avthetadr` |
| branch policy | `ksatadj`, `ksatfac`, `ksatrec`, `lkeff`, `solwpv` | selects the existing 9001/9002+/9003 branch |

The `cpm_####` lineage was added to the WB19 state accessor surface at
`state_access.rs:1337-1417`; WB14 now loads all source-intent operands at
`02_ksat_adjustment.rs:38-55`.

## Guard posture

- Missing source-intent operands are typed failures, not defaults.
- `por`, `cpm`, `thetfc`, and `thetdr` are required positive unit-interval
  terms on the active path.
- `thetfc > thetdr`, `fc <= ul`, and `theta <= ul` are preserved as layer
  ordering guards.

Ran evidence:

- `wb14_ksatadj_missing_source_intent_operand_is_typed_failure` removes
  `cpm_0002` and receives `MissingRequiredStateSymbol { symbol: cpm_0002 }`.
- Focused WB14 unit and integration suites passed after the lineage change.
