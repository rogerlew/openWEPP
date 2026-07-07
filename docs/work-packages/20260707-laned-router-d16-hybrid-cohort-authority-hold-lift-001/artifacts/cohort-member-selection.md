# Cohort Member Selection

Status: EXECUTED-HOLD-ROUTE-COEFFICIENT-AUTHORITY. Evidence mode: Static + Ran.

## Selected Minimum Cohort

| Member | Source | Role | Active-Runnable Status |
|---|---|---|---|
| H2637 | `tests/fixtures/laned_shadow_h2637` plus prior D16 patched scratch evidence | Known high-runoff/multi-event Lane-D benchmark | Runnable only when package/test scratch inserts native cropland `routing_coefficients`; H2637 alone is insufficient for promotion tolerance. |
| Minnesota corn H1-H43 | `tools/owcmp/suites/minnesota-corn-ksflag1.json` -> `/wc1/runs/al/algebraic-radium` | Cropland ksflag=1 cohort | Inventory-only; no native route coefficients. |
| N Idaho single-OFE | `tools/owcmp/suites/n-idaho-single-ofe-ksflag0.json` -> `/wc1/runs/un/unpalatable-rind` | Single-OFE/different climate cohort | Inventory-only; no native route coefficients. |
| WA Cascades MOFE | `tools/owcmp/suites/wa-cascades-mofe-ksflag0.json` -> `/wc1/runs/ar/arboreal-dendrite` | MOFE/watershed-style cohort | Inventory-only; no native route coefficients. |

## Selection Rationale

The set matches the prior worker handoff: reuse the current owcmp inventories
plus H2637. It is the smallest useful promotion cohort candidate because it
would span the known D16 H2637 case, cropland ksflag=1, single-OFE ksflag=0,
and MOFE ksflag=0 roots.

## Selection Result

The selection is valid as a target cohort but not yet valid as an executable
active plain-vs-hybrid promotion cohort. The external members do not carry
source-authorized Lane-D route coefficients, and the manifests are not runnable
comparison pairs.
