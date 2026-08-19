# Reference And Equation Lineage

Evidence class: `Ran + Static`

Retrieved 2026-08-19 from primary publishers/projects:

| Source | Locator | Local retrieval digest | Bound claim |
|---|---|---|---|
| CTSM CLM5.0 Plant Hydraulics technical note | section 2.11.2.1.3, equations 2.11.14--18 | `4228822c94293f6673adf12b0fbb7d4e3a78f72e5c268eecb9cefef75ba36cee` | Soil layers connect in parallel; soil-interface and root-tissue resistances are in series; `z3` and `dxroot` are distinct; gravity participates in the gradient. |
| CTSM CLM5.0 Hydrology technical note | section 2.7.3.1, equations 2.7.49--55 | `fff5080f4b9285bfa19bca4f7913b17e93c341138249f70d515c6706b5cced09` | Node-depth matric potential uses porosity, saturated potential and B with relative saturation in `[0.01,1]` and a `-1e8 mm` floor. |
| Clapp and Hornberger (1978) | DOI `10.1029/WR014i004p00601` | citation locator only; copyrighted bytes not vendored | Retention and conductivity power relations are distinct from derived Green-Ampt wetting-front suction. |

These sources justify the proposed equation family and the non-alias rule. They
do not provide site-specific openWEPP root-tissue path values, parameter
calibration, or an input-data mapping.
