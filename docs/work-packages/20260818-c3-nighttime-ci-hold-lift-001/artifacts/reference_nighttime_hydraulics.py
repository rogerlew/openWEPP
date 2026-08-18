#!/usr/bin/env python3
"""Independent 100-decimal V10 nighttime scalar hydraulic envelope.

This diagnostic deliberately does *not* claim to solve the coupled canopy/LSE
system. Several values are warm-start/illustrative operands, not the accepted
full-precision owner snapshot. Its output is not authority and must not be
used as a Rust expected result.
"""

from decimal import Decimal, getcontext
import json

getcontext().prec = 100
D = Decimal
LN2 = D(2).ln()
R_DRY = D("287.05")
R_GAS = D("8.31446261815324")
RHO_W = D(1000)


def vulnerability(psi, p50, shape):
    return (-LN2 * (psi / p50) ** shape).exp()


def bisect_signed(low, high, function):
    f_low = function(low)
    f_high = function(high)
    if not f_low or not f_high or (f_low > 0) == (f_high > 0):
        raise ArithmeticError("unbracketed")
    for _ in range(500):
        mid = (low + high) / 2
        value = function(mid)
        if not value:
            return mid
        if (value > 0) == (f_low > 0):
            low, f_low = mid, value
        else:
            high = mid
    return (low + high) / 2


CANONICAL = {
    "interval_s": D(1800),
    "pressure_pa": D("87633.52548751776"),
    "canopy_temperature_k": D("295.8"),
    "canopy_q": D("0.011"),
    "ca_pa": D(42),
    "leaf_temperature_k": D("295.4"),
    "surface_q": D("0.01928330221924848"),
    "leaf_area": D("1.0000000000000002"),
    "wet_fraction": D(0),
    "gb_m_s": D("0.0263049903"),
    "g0_umol_m2_s": D(25),
    "p50_leaf_mm": D(-120000),
    "p50_xylem_mm": D(-160000),
    "p50_root_mm": D(-140000),
    "shape": D(2),
    "k1_s1": D("3.5e-5"),
    "k2": D("2.8e-8"),
    "sai": D("0.35"),
    "height_m": D("12.5"),
    "authorization": D("9.7595293578063313e-10"),
    "rd": D("0.81"),
}


def fixed_authorization(case, fraction):
    authorization = case["authorization"] * fraction
    rho = case["pressure_pa"] / (R_DRY * case["canopy_temperature_k"])
    gs0 = case["g0_umol_m2_s"] * D("1e-6") * R_GAS * case["leaf_temperature_k"] / case["pressure_pa"]
    rb = 1 / case["gb_m_s"]
    dry_area = case["leaf_area"] * (1 - case["wet_fraction"])
    e0 = rho * dry_area * (case["surface_q"] - case["canopy_q"]) / (rb + 1 / gs0)
    if authorization == 0:
        return {"accepted": False, "typed_failure": "VEG-E-121", "reason": "positive residual demand with zero authorization", "e0": str(e0), "ci_pa": None}
    if authorization > e0:
        return {"accepted": False, "typed_failure": "VEG-E-121", "reason": "authorization exceeds unstressed demand", "e0": str(e0), "ci_pa": None}
    fhyd = authorization / e0
    psi_leaf = -abs(case["p50_leaf_mm"]) * (-fhyd.ln() / LN2) ** (1 / case["shape"])
    q1 = lambda psi_stem: case["k1_s1"] * case["leaf_area"] * vulnerability(psi_stem, case["p50_xylem_mm"], case["shape"]) * (psi_stem - psi_leaf)
    psi_stem = bisect_signed(psi_leaf, D(0), lambda value: q1(value) - authorization)
    gravity = RHO_W * case["height_m"]
    q2 = lambda psi_root: case["k2"] / case["height_m"] * vulnerability(psi_root, case["p50_xylem_mm"], case["shape"]) * case["sai"] * (psi_root - psi_stem - gravity)
    # This is the fixed-cap feasibility envelope. Full owner-layer q3 joins are
    # reconstructed separately in the frozen snapshot audit.
    root_low = psi_stem + gravity
    root_high = D(1000)
    if root_low >= root_high:
        return {"accepted": False, "typed_failure": "VEG-E-121", "reason": "root gravitational support", "e0": str(e0), "psi_leaf_mm": str(psi_leaf), "psi_stem_mm": str(psi_stem), "ci_pa": None}
    psi_root = bisect_signed(root_low, root_high, lambda value: q2(value) - authorization)
    rs = rho * dry_area * (case["surface_q"] - case["canopy_q"]) / authorization - rb
    gs = 1 / rs
    beta = gs / gs0
    ci = case["ca_pa"] + (D("1.4") * rb + D("1.6") / gs) * R_GAS * case["leaf_temperature_k"] * case["rd"] * D("1e-6")
    if ci <= 0 or ci >= case["pressure_pa"]:
        return {"accepted": False, "typed_failure": "VEG-E-121", "reason": "hydraulically attenuated conductance requires Ci outside (0,Patm)", "e0": str(e0), "authorization": str(authorization), "gs_m_s": str(gs), "ci_pa": str(ci), "psi_leaf_mm": str(psi_leaf), "psi_stem_mm": str(psi_stem), "psi_root_mm": str(psi_root)}
    return {
        "accepted": True,
        "e0": str(e0),
        "authorization": str(authorization),
        "fhyd": str(fhyd),
        "log_fhyd": str(fhyd.ln()),
        "gs0_m_s": str(gs0),
        "gs_m_s": str(gs),
        "beta_hyd": str(beta),
        "ci_pa": str(ci),
        "psi_leaf_mm": str(psi_leaf),
        "psi_stem_mm": str(psi_stem),
        "psi_root_mm": str(psi_root),
        "continuity": {
            "egas_minus_ehyd": str(authorization - e0 * fhyd),
            "ehyd_minus_q1": str(authorization - q1(psi_stem)),
            "q1_minus_q2": str(authorization - q2(psi_root)),
        },
    }


payload = {
    "evidence": "independent_decimal_v10_nighttime_scalar_envelope_v1",
    "accepted_authority": False,
    "decimal_precision": getcontext().prec,
    "mechanism": "stomatal_hydraulic_attenuation_without_storage_or_cuticular_alias",
    "cases": {
        "full": fixed_authorization(CANONICAL, D(1)),
        "partial": fixed_authorization(CANONICAL, D("0.25")),
        "very_small": fixed_authorization(CANONICAL, D("1e-6")),
        "zero": fixed_authorization(CANONICAL, D(0)),
    },
}
print(json.dumps(payload, sort_keys=True, separators=(",", ":")))
