#!/usr/bin/env python3
"""Independent exact-binary64 reconstruction of the admitted night branch."""

import json

R_GAS = 8.31446261815324


def solve(case):
    an = -case["rd_umol_co2_m2_leaf_s"]
    gs_m_s = (
        case["g0_umol_h2o_m2_s"]
        * 1.0e-6
        * R_GAS
        * case["temperature_k"]
        / case["pressure_pa"]
    )
    rs_s_m = 1.0 / gs_m_s
    cs_pa = case["ca_pa"] - (
        1.4
        * case["rb_s_m"]
        * R_GAS
        * case["temperature_k"]
        * an
        * 1.0e-6
    )
    ci_pa = case["ca_pa"] - (
        (1.4 * case["rb_s_m"] + 1.6 * rs_s_m)
        * R_GAS
        * case["temperature_k"]
        * an
        * 1.0e-6
    )
    return {
        "an_umol_co2_m2_leaf_s": an,
        "gross_assimilation_umol_co2_m2_leaf_s": 0.0,
        "gs_m_s": gs_m_s,
        "rs_s_m": rs_s_m,
        "cs_pa": cs_pa,
        "ci_pa": ci_pa,
    }


CASES = [
    {
        "name": "ordinary_night",
        "absorbed_par_w_m2_leaf": 0.0,
        "temperature_k": 296.0,
        "pressure_pa": 101325.0,
        "ca_pa": 42.0,
        "rb_s_m": 50.0,
        "g0_umol_h2o_m2_s": 100.0,
        "rd_umol_co2_m2_leaf_s": 1.2,
    },
    {
        "name": "low_g0_high_elevation",
        "absorbed_par_w_m2_leaf": 0.0,
        "temperature_k": 285.0,
        "pressure_pa": 80000.0,
        "ca_pa": 40.0,
        "rb_s_m": 80.0,
        "g0_umol_h2o_m2_s": 10.0,
        "rd_umol_co2_m2_leaf_s": 0.5,
    },
]

print(
    json.dumps(
        {"authority": "SC-VEGETATION-001@13-existing-equations", "cases": [dict(case, result=solve(case)) for case in CASES]},
        sort_keys=True,
        separators=(",", ":"),
    )
)
