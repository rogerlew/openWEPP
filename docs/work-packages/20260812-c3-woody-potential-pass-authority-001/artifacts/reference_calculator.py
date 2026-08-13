#!/usr/bin/env python3
"""Independent, standard-library OPENWEPP_C3_WOODY_V3 vector oracle.

This is evidence-generating code, not production code.  It deliberately owns
its equations and canonical serialization and never imports or calls Rust.
"""

from __future__ import annotations

import hashlib
import json
import math
import struct
from pathlib import Path
from typing import Callable


MODEL = "OPENWEPP_C3_WOODY_V3"
R_GAS = 8.31446261815324


def canonical_bytes(value: object) -> bytes:
    return (json.dumps(value, allow_nan=False, separators=(",", ":"),
                       sort_keys=True) + "\n").encode("utf-8")


def require_finite(*values: float) -> None:
    if not all(math.isfinite(value) for value in values):
        raise ValueError("nonfinite operand")


def simpson(function: Callable[[float], float], left: float, right: float,
            tolerance: float = 1.0e-12, depth: int = 24) -> float:
    fl, fr = function(left), function(right)
    middle = 0.5 * (left + right)
    fm = function(middle)
    whole = (right-left) * (fl + 4.0*fm + fr) / 6.0

    def recurse(a: float, b: float, fa: float, fb: float, fc: float,
                estimate: float, tol: float, remaining: int) -> float:
        c = 0.5 * (a+b)
        d, e = 0.5 * (a+c), 0.5 * (c+b)
        fd, fe = function(d), function(e)
        one = (c-a) * (fa+4.0*fd+fc) / 6.0
        two = (b-c) * (fc+4.0*fe+fb) / 6.0
        delta = one + two - estimate
        if abs(delta) <= 15.0*tol:
            return one + two + delta/15.0
        if remaining == 0:
            raise ArithmeticError("adaptive quadrature iteration limit")
        return (recurse(a, c, fa, fc, fd, one, tol/2.0, remaining-1)
                + recurse(c, b, fc, fb, fe, two, tol/2.0, remaining-1))

    return recurse(left, right, fl, fr, fm, whole, tolerance, depth)


def mat_add(a: tuple[tuple[float, float], tuple[float, float]],
            b: tuple[tuple[float, float], tuple[float, float]]):
    return ((a[0][0]+b[0][0], a[0][1]+b[0][1]),
            (a[1][0]+b[1][0], a[1][1]+b[1][1]))


def mat_mul(a, b):
    return ((a[0][0]*b[0][0]+a[0][1]*b[1][0],
             a[0][0]*b[0][1]+a[0][1]*b[1][1]),
            (a[1][0]*b[0][0]+a[1][1]*b[1][0],
             a[1][0]*b[0][1]+a[1][1]*b[1][1]))


def mat_vec(a, x):
    return (a[0][0]*x[0]+a[0][1]*x[1],
            a[1][0]*x[0]+a[1][1]*x[1])


def mat_scale(a, scale: float):
    return ((scale*a[0][0], scale*a[0][1]),
            (scale*a[1][0], scale*a[1][1]))


def mat_inverse(a):
    determinant = a[0][0]*a[1][1]-a[0][1]*a[1][0]
    if abs(determinant) <= 1.0e-14:
        raise ArithmeticError("two-stream resonance requires exact integral branch")
    return ((a[1][1]/determinant, -a[0][1]/determinant),
            (-a[1][0]/determinant, a[0][0]/determinant))


IDENTITY = ((1.0, 0.0), (0.0, 1.0))


def expm_trace_zero(a, distance: float):
    # The admitted two-stream matrix has trace zero, so A^2=r^2 I.
    r2 = a[0][0]*a[0][0] + a[0][1]*a[1][0]
    if abs(r2) <= 1.0e-28:
        return mat_add(IDENTITY, mat_scale(a, distance))
    if r2 > 0.0:
        r = math.sqrt(r2)
        return mat_add(mat_scale(IDENTITY, math.cosh(r*distance)),
                       mat_scale(a, math.sinh(r*distance)/r))
    r = math.sqrt(-r2)
    return mat_add(mat_scale(IDENTITY, math.cos(r*distance)),
                   mat_scale(a, math.sin(r*distance)/r))


def optics(layer: dict, band: str, mu: float | None) -> dict:
    leaf, stem = layer["leaf_area"], layer["stem_area"]
    plant = leaf + stem
    if plant <= 0.0:
        raise ValueError("positive plant area required by optics layer")
    wl, ws = leaf/plant, stem/plant
    leaf_opt, stem_opt = layer["optics"][band]["leaf"], layer["optics"][band]["stem"]
    rho = wl*leaf_opt["rho"] + ws*stem_opt["rho"]
    tau = wl*leaf_opt["tau"] + ws*stem_opt["tau"]
    chi = layer["leaf_angle_chi"]
    if not -0.4 <= chi <= 0.6:
        raise ValueError("leaf-angle domain")
    phi1 = 0.5 - 0.633*chi - 0.33*chi*chi
    phi2 = 0.877*(1.0-2.0*phi1)
    mubar = simpson(lambda angle: angle/(phi1+phi2*angle), 0.0, 1.0,
                    tolerance=1.0e-14)
    omega = rho+tau
    omega_beta = 0.5*(rho+tau+(rho-tau)*((1.0+chi)/2.0)**2)
    beta = omega_beta/omega if omega else 0.0
    if mu is not None:
        if mu <= 0.0:
            raise ValueError("direct-beam zenith domain")
        gmu = phi1 + phi2*mu
        beam_k = gmu/mu
        k_eff = layer["clumping_index"] * beam_k
    else:
        gmu = beam_k = k_eff = None
    if omega and mu is not None:
        scatter = 0.5*omega*simpson(
            lambda angle: angle*gmu/(mu*(phi1+phi2*angle)+angle*gmu),
            0.0, 1.0, tolerance=1.0e-14)
        beta0 = (1.0+mubar*k_eff)*scatter/(mubar*k_eff*omega)
    else:
        beta0 = 0.0
    b = 1.0-(1.0-beta)*omega
    c = omega*beta
    d = omega*mubar*k_eff*beta0 if k_eff is not None else 0.0
    f = omega*mubar*k_eff*(1.0-beta0) if k_eff is not None else 0.0
    matrix = ((b/mubar, -c/mubar), (c/mubar, -b/mubar))
    source = (-d/mubar, f/mubar)
    a_leaf = wl*(1.0-leaf_opt["rho"]-leaf_opt["tau"])
    a_stem = ws*(1.0-stem_opt["rho"]-stem_opt["tau"])
    denominator = a_leaf+a_stem
    if denominator < 0.0:
        raise ValueError("nonpositive physical absorptivity")
    leaf_fraction = a_leaf/denominator if denominator > 0.0 else 0.0
    stem_fraction = a_stem/denominator if denominator > 0.0 else 0.0
    return {"beam_k_unclumped": beam_k, "k_eff": k_eff, "mubar": mubar,
            "rho_effective": rho, "tau_effective": tau,
            "leaf_absorption_fraction": leaf_fraction,
            "stem_absorption_fraction": stem_fraction,
            "matrix": matrix, "source": source, "plant_area": plant,
            "leaf_weight": wl, "stem_weight": ws}


def transfer(coeff: dict, distance: float, beam_top: float):
    matrix, source, k = coeff["matrix"], coeff["source"], coeff["k_eff"]
    homogeneous = expm_trace_zero(matrix, distance)
    if beam_top == 0.0:
        return homogeneous, (0.0, 0.0)
    if k is None:
        raise AssertionError("positive direct beam without directional extinction")
    shifted = mat_add(matrix, mat_scale(IDENTITY, k))
    try:
        particular = mat_vec(mat_inverse(shifted),
                             mat_vec(mat_add(homogeneous,
                                             mat_scale(IDENTITY, -math.exp(-k*distance))),
                                     source))
    except ArithmeticError:
        # Exact variation-of-constants definition for the removable resonance.
        # Adaptive quadrature evaluates the matrix-exponential integral without
        # perturbing K or clamping either exponent.
        def component(index: int) -> float:
            return simpson(
                lambda s: mat_vec(expm_trace_zero(matrix, distance-s), source)[index]
                * math.exp(-k*s), 0.0, distance, tolerance=1.0e-13)
        particular = (component(0), component(1))
    return homogeneous, (beam_top*particular[0], beam_top*particular[1])


def apply_transfer(matrix, offset, state):
    result = mat_vec(matrix, state)
    return result[0]+offset[0], result[1]+offset[1]


def radiation_component(layers: list[dict], band: str, mu: float | None,
                        direct_top: float, diffuse_top: float,
                        ground_albedo: float) -> dict:
    if direct_top > 0.0 and mu is None:
        raise ValueError("direct radiation requires zenith cosine")
    # The zero-direct branch intentionally never constructs beta0, K, or any
    # direct-source operand.
    coeffs = [optics(layer, band, mu if direct_top > 0.0 else None)
              for layer in layers]
    total_matrix, total_offset = IDENTITY, (0.0, 0.0)
    beam = direct_top
    transfers = []
    for coeff in coeffs:
        matrix, offset = transfer(coeff, coeff["plant_area"], beam)
        transfers.append((matrix, offset, beam))
        total_offset = apply_transfer(matrix, offset, total_offset)
        total_matrix = mat_mul(matrix, total_matrix)
        if beam > 0.0:
            beam *= math.exp(-coeff["k_eff"]*coeff["plant_area"])
    base0 = apply_transfer(total_matrix, total_offset, (0.0, diffuse_top))
    base1 = apply_transfer(total_matrix, total_offset, (1.0, diffuse_top))
    slope = (base1[0]-base0[0], base1[1]-base0[1])
    denominator = slope[0]-ground_albedo*slope[1]
    if abs(denominator) <= 1.0e-14:
        raise ArithmeticError("column boundary singular")
    up_top = (ground_albedo*(base0[1]+beam)-base0[0])/denominator
    state, beam = (up_top, diffuse_top), direct_top
    rows = []
    for layer, coeff, (matrix, offset, _) in zip(layers, coeffs, transfers):
        state_top, beam_top = state, beam

        def state_at(x: float):
            mx, hx = transfer(coeff, x, beam_top)
            return apply_transfer(mx, hx, state_top)

        def local_absorption(x: float):
            up, down = state_at(x)
            source_beam = (beam_top*math.exp(-coeff["k_eff"]*x)
                           if beam_top > 0.0 else 0.0)
            dup, ddown = mat_vec(coeff["matrix"], (up, down))
            dup += coeff["source"][0]*source_beam
            ddown += coeff["source"][1]*source_beam
            direct_absorption = (coeff["k_eff"]*source_beam
                                 if beam_top > 0.0 else 0.0)
            return direct_absorption-ddown+dup

        total_abs = simpson(local_absorption, 0.0, coeff["plant_area"])
        if direct_top > 0.0:
            plant_sun_abs = simpson(
                lambda x: local_absorption(x)*math.exp(-coeff["k_eff"]*x),
                0.0, coeff["plant_area"])
        else:
            plant_sun_abs = 0.0
        plant_shade_abs = total_abs-plant_sun_abs
        leaf_fraction = coeff["leaf_absorption_fraction"]
        leaf_sun_area = (coeff["leaf_weight"]
                         * -math.expm1(-coeff["k_eff"]*coeff["plant_area"])
                         / coeff["k_eff"] if direct_top > 0.0 else 0.0)
        leaf_shade_area = layer["leaf_area"]-leaf_sun_area
        rows.append({
            "occupancy_id": layer["occupancy_id"],
            "operands": {key: value for key, value in coeff.items()
                         if key not in {"matrix", "source"}},
            "results": {
                "absorbed_plant": total_abs,
                "absorbed_leaf_sun": leaf_fraction*plant_sun_abs,
                "absorbed_leaf_shade": leaf_fraction*plant_shade_abs,
                "absorbed_stem": coeff["stem_absorption_fraction"]*total_abs,
                "leaf_sun_area": leaf_sun_area,
                "leaf_shade_area": leaf_shade_area,
            },
        })
        state = apply_transfer(matrix, offset, state)
        if beam > 0.0:
            beam *= math.exp(-coeff["k_eff"]*coeff["plant_area"])
    terminal_diffuse = state[1]
    absorbed = math.fsum(row["results"]["absorbed_plant"] for row in rows)
    ground_absorbed = (1.0-ground_albedo)*(terminal_diffuse+beam)
    closure = direct_top+diffuse_top-up_top-ground_absorbed-absorbed
    return {"incident_direct": direct_top, "incident_diffuse": diffuse_top,
            "top_reflected": up_top, "terminal_direct": beam,
            "terminal_diffuse": terminal_diffuse,
            "ground_absorbed": ground_absorbed, "occupancies": rows,
            "closure_residual": closure}


def radiation_vectors() -> dict:
    optics_by_band = {
        "VIS": {"leaf": {"rho": 0.09, "tau": 0.06},
                "stem": {"rho": 0.18, "tau": 0.03}},
        "NIR": {"leaf": {"rho": 0.41, "tau": 0.31},
                "stem": {"rho": 0.29, "tau": 0.12}},
    }
    lower_optics = {
        "VIS": {"leaf": {"rho": 0.12, "tau": 0.04},
                "stem": {"rho": 0.22, "tau": 0.02}},
        "NIR": {"leaf": {"rho": 0.37, "tau": 0.27},
                "stem": {"rho": 0.25, "tau": 0.10}},
    }
    layers = [
        {"occupancy_id": "upper@tile-a", "leaf_area": 2.6,
         "stem_area": 0.7, "clumping_index": 0.74,
         "leaf_angle_chi": 0.12, "optics": optics_by_band},
        {"occupancy_id": "lower@tile-a", "leaf_area": 1.35,
         "stem_area": 0.45, "clumping_index": 0.86,
         "leaf_angle_chi": -0.08, "optics": lower_optics},
    ]
    incident = {"VIS": {"direct": 410.0, "diffuse": 83.0, "albedo": 0.14},
                "NIR": {"direct": 355.0, "diffuse": 101.0, "albedo": 0.31}}
    outputs = {}
    for band, values in incident.items():
        outputs[band] = {
            "direct": radiation_component(layers, band, 0.67,
                                          values["direct"], 0.0,
                                          values["albedo"]),
            "diffuse": radiation_component(layers, band, 0.67,
                                           0.0, values["diffuse"],
                                           values["albedo"]),
        }
    reductions = {}
    for name, leaf, stem in (("leaf_only", 2.1, 0.0),
                             ("stem_only", 0.0, 0.8),
                             ("identical_optics", 1.4, 0.6)):
        selected = json.loads(json.dumps(optics_by_band))
        if name == "identical_optics":
            for band in selected.values():
                band["stem"] = dict(band["leaf"])
        layer = {"occupancy_id": name, "leaf_area": leaf, "stem_area": stem,
                 "clumping_index": 0.8, "leaf_angle_chi": 0.1,
                 "optics": selected}
        reductions[name] = radiation_component([layer], "VIS", 0.67,
                                                300.0, 50.0, 0.18)
    zero_plant_result = radiation_component([], "VIS", 0.67, 100.0, 23.0, 0.18)
    zero_plant = {"operands": {"layers": [], "direct": 100.0,
                                "diffuse": 23.0, "ground_albedo": 0.18},
                  "result": zero_plant_result, "executed": True,
                  "expected_canopy_absorbed": 0.0}
    zero_absorption_optics = {band: {
        "leaf": {"rho": 0.6, "tau": 0.4},
        "stem": {"rho": 0.3, "tau": 0.7}} for band in ("VIS", "NIR")}
    zero_absorption_layer = {"occupancy_id": "zero-absorptivity",
        "leaf_area": 1.4, "stem_area": 0.6, "clumping_index": 0.8,
        "leaf_angle_chi": 0.1, "optics": zero_absorption_optics}
    zero_absorption = radiation_component([zero_absorption_layer], "VIS", 0.67,
                                           300.0, 50.0, 0.18)
    if abs(zero_absorption["occupancies"][0]["results"]["absorbed_plant"]) > 2.0e-8:
        raise AssertionError("zero absorptivity branch")
    resonance_coefficients = {
        "matrix": ((-0.5, 0.0), (0.0, 0.5)), "source": (1.0, 0.2),
        "k_eff": 0.5}
    resonance_matrix, resonance_offset = transfer(resonance_coefficients, 1.3, 2.0)
    resonance_expected_first = 2.0*1.3*math.exp(-0.5*1.3)
    if not math.isclose(resonance_offset[0], resonance_expected_first,
                        rel_tol=1.0e-12, abs_tol=1.0e-13):
        raise AssertionError("exact resonance integral branch")
    return {"units": {"area": "m2 plant m-2 tile-ground",
                       "radiation": "W m-2 tile-ground"},
            "operands": {"mu": 0.67, "layers": layers, "incident": incident},
            "two_rank": outputs, "reductions": reductions,
            "zero_plant_area_exact_branch": zero_plant,
            "zero_absorptivity_exact_owner_branch": zero_absorption,
            "zero_direct_exact_branch": {
                "directional_operands_evaluated": False,
                "beam_k_unclumped": outputs["VIS"]["diffuse"]["occupancies"][0]
                    ["operands"]["beam_k_unclumped"],
                "k_eff": outputs["VIS"]["diffuse"]["occupancies"][0]
                    ["operands"]["k_eff"],
                "terminal_direct": outputs["VIS"]["diffuse"]["terminal_direct"]},
            "resonance_exact_integral_branch": {
                "operands": resonance_coefficients,
                "transfer_matrix": resonance_matrix,
                "source_offset": resonance_offset,
                "analytic_first_component": resonance_expected_first}}


def aerodynamic_vector() -> dict:
    inputs = {"kappa": 0.4, "u_ref_m_s": 3.7, "z_ref_m": 24.0,
              "displacement_m": 8.1, "z0m_m": 1.25,
              "z0h_m": 0.12, "z0q_m": 0.08,
              "cv_m_s_half": 0.01, "leaf_dimension_m": 0.045,
              "wet_surface_dimension_m": 0.16, "stem_dimension_m": 0.34}
    if inputs["z_ref_m"] <= inputs["displacement_m"]+inputs["z0m_m"]:
        raise ValueError("invalid neutral wind geometry")
    ustar = (inputs["kappa"]*inputs["u_ref_m_s"]
             / math.log((inputs["z_ref_m"]-inputs["displacement_m"])
                        / inputs["z0m_m"]))
    winds = {name: ustar for name in ("u_leaf_m_s", "u_wet_m_s", "u_stem_m_s")}
    conductances = {
        "gb_leaf_m_s": inputs["cv_m_s_half"]*math.sqrt(ustar/inputs["leaf_dimension_m"]),
        "gb_wet_m_s": inputs["cv_m_s_half"]*math.sqrt(ustar/inputs["wet_surface_dimension_m"]),
        "gb_stem_m_s": inputs["cv_m_s_half"]*math.sqrt(ustar/inputs["stem_dimension_m"]),
    }
    rah = (math.log((inputs["z_ref_m"]-inputs["displacement_m"])/inputs["z0m_m"])
           * math.log((inputs["z_ref_m"]-inputs["displacement_m"])/inputs["z0h_m"])
           /(inputs["kappa"]**2*inputs["u_ref_m_s"]))
    raw = (math.log((inputs["z_ref_m"]-inputs["displacement_m"])/inputs["z0m_m"])
           * math.log((inputs["z_ref_m"]-inputs["displacement_m"])/inputs["z0q_m"])
           /(inputs["kappa"]**2*inputs["u_ref_m_s"]))
    return {"operands": inputs, "results": {"u_star_m_s": ustar,
                                               "semantic_winds": winds,
                                               "conductances": conductances,
                                               "rah_s_m": rah, "raw_s_m": raw}}


def vulnerability(potential: float, p50: float, exponent: float) -> float:
    return 2.0 ** (-(potential/p50)**exponent)


def smaller_quadratic_root(a: float, b: float, c: float) -> float:
    if a == 0.0:
        if b == 0.0:
            raise ValueError("indeterminate quadratic")
        return -c/b
    if c == 0.0:
        return min(0.0, -b/a)
    discriminant = b*b-4.0*a*c
    if discriminant < 0.0:
        scale = max(abs(b*b), abs(4.0*a*c))
        if discriminant >= -64.0*2.220446049250313e-16*scale:
            discriminant = 0.0
        else:
            raise ValueError("negative photosynthesis discriminant")
    root = math.sqrt(discriminant)
    q = -0.5*(b+math.copysign(root, b))
    return min(q/a, c/q)


def saturation_specific_humidity(temperature_k: float, pressure_pa: float) -> float:
    tc = temperature_k-273.15
    coefficients = [6.11213476, 4.44007856e-1, 1.43064234e-2,
                    2.64461437e-4, 3.05903558e-6, 1.96237241e-8,
                    8.92344772e-11, -3.73208410e-13, 2.09339997e-16]
    if not 0.0 <= tc <= 100.0:
        raise ValueError("liquid saturation polynomial domain")
    es = 100.0*math.fsum(value*tc**power
                         for power, value in enumerate(coefficients))
    return 0.622*es/(pressure_pa-0.378*es)


class BrentFailure(ArithmeticError):
    def __init__(self, message: str, evaluations: int,
                 bracket: tuple[float, float], normalized: list[float]):
        super().__init__(message)
        self.evaluations = evaluations
        self.bracket = bracket
        self.normalized = normalized


def brent_dekker(function: Callable[[float], tuple[float, dict]], low: float,
                 high: float, max_evaluations: int = 64
                 ) -> tuple[float, dict, int, tuple[float, float]]:
    """Canonical bracketed Brent-Dekker with bisection safeguard."""
    a, b = low, high
    fa, _ = function(a)
    fb, state = function(b)
    if not all(math.isfinite(value) for value in (fa, fb)):
        raise BrentFailure("ci nonfinite domain", 2, (a, b), [])
    if fa == 0.0:
        return a, function(a)[1], 2, (a, b)
    if fb == 0.0:
        return b, state, 2, (a, b)
    if fa*fb > 0.0:
        raise BrentFailure("ci bracket failure", 2, (a, b),
                           [fa/1.0e-8, fb/1.0e-8])
    c, fc, d = a, fa, b-a
    mflag = True
    for evaluation in range(3, max_evaluations+1):
        if fa != fc and fb != fc:
            s = (a*fb*fc/((fa-fb)*(fa-fc))
                 + b*fa*fc/((fb-fa)*(fb-fc))
                 + c*fa*fb/((fc-fa)*(fc-fb)))
        else:
            s = b-fb*(b-a)/(fb-fa)
        left, right = min((3.0*a+b)/4.0, b), max((3.0*a+b)/4.0, b)
        conditions = (not left < s < right,
                      mflag and abs(s-b) >= abs(b-c)/2.0,
                      not mflag and abs(s-b) >= abs(c-d)/2.0,
                      mflag and abs(b-c) < 1.0e-6,
                      not mflag and abs(c-d) < 1.0e-6)
        if any(conditions):
            s, mflag = 0.5*(a+b), True
        else:
            mflag = False
        fs, state = function(s)
        d, c, fc = c, b, fb
        if fa*fs < 0.0:
            b, fb = s, fs
        else:
            a, fa = s, fs
        if abs(fa) < abs(fb):
            a, b, fa, fb = b, a, fb, fa
        scale = max(abs(a), abs(b), 1.0)
        if abs(fb) <= 1.0e-8 or abs(b-a) <= 1.0e-6+1.0e-10*scale:
            _, state = function(b)
            return b, state, evaluation, (min(a, b), max(a, b))
    raise BrentFailure("ci iteration limit", max_evaluations,
                       (min(a, b), max(a, b)), [fa/1.0e-8, fb/1.0e-8])


def executed_ci_failures() -> list[dict]:
    outputs = []
    scenarios = [
        ("domain", lambda value: (math.nan, {"x": value}), 0.0, 1.0, 64,
         "ci nonfinite domain"),
        ("bracket", lambda value: (value*value+1.0, {"x": value}), -1.0, 1.0, 64,
         "ci bracket failure"),
        ("iteration_limit", lambda value: (value*value-2.0, {"x": value}), 0.0, 2.0, 2,
         "ci iteration limit"),
    ]
    for solve_identity in ("sun_ci", "shade_ci"):
        for identity, function, low, high, limit, expected in scenarios:
            try:
                brent_dekker(function, low, high, max_evaluations=limit)
                raise AssertionError(f"{identity} failure was not executed")
            except BrentFailure as error:
                if str(error) != expected:
                    raise
                diagnostics = numerical_failure(solve_identity, error.evaluations,
                                                  error.normalized, None, 0)
                diagnostics["bracket"] = list(error.bracket)
                outputs.append({"failure_kind": identity, "typed_error": expected,
                    "diagnostics": diagnostics, "candidate": None,
                    "last_iterate": None, "evaluations": error.evaluations,
                    "executed_by": "brent_dekker"})
    return outputs


def executed_failure_precedence() -> dict:
    order = ["identity_schema", "domain", "bracket", "singular", "iteration"]
    def validate(active: set[str]) -> str:
        for identity in order:
            if identity in active:
                return identity
        return "none"
    rows = []
    for index in range(len(order)):
        active = set(order[index:])
        selected = validate(active)
        if selected != order[index]:
            raise AssertionError("failure precedence")
        rows.append({"present": sorted(active), "selected": selected,
                     "candidate": None, "last_iterate": None,
                     "executed_by": "ordered_failure_validator"})
    return {"order": order, "competing_conditions": rows}


def coupled_canopy_energy(case: dict, betas: tuple[float, float],
                          leaf_potentials: tuple[float, float],
                          max_iterations: int = 50) -> dict:
    """Solve dry sun/shade, wet, dry-stem, and canopy-air nodes together."""
    forcing, classes = case["gas_energy"], case["classes"]
    pressure, ca = forcing["pressure_pa"], forcing["ca_pa"]
    gb_leaf, gb_wet, gb_stem = (forcing[key] for key in
        ("gb_leaf_m_s", "gb_wet_m_s", "gb_stem_m_s"))
    rb = 1.0/gb_leaf
    wet_fraction = forcing["wet_fraction"]
    dry_areas = {name: value["leaf_area"]*(1.0-wet_fraction)
                 for name, value in classes.items()}
    wet_leaf_area = wet_fraction*math.fsum(value["leaf_area"] for value in classes.values())
    wet_stem_area = wet_fraction*forcing["stem_area"]
    wet_area = wet_leaf_area+wet_stem_area
    dry_stem_area = (1.0-wet_fraction)*forcing["stem_area"]
    beta_by_class = {"sun": betas[0], "shade": betas[1]}

    def class_state(name: str, temperature: float, qcan: float) -> dict:
        class_input = classes[name]
        biochemical_parameters = case["biochemical_parameters"]
        par = class_input["absorbed_par_w_m2_leaf"]
        vcmax_factor = peaked_response(temperature,
            biochemical_parameters["ha_vcmax_j_mol"],
            biochemical_parameters["hd_vcmax_j_mol"],
            biochemical_parameters["entropy_vcmax_j_mol_k"])
        jmax_factor = peaked_response(temperature,
            biochemical_parameters["ha_jmax_j_mol"],
            biochemical_parameters["hd_jmax_j_mol"],
            biochemical_parameters["entropy_jmax_j_mol_k"])
        vcmax = class_input["vcmax25"]*vcmax_factor
        jmax = class_input["jmax25"]*jmax_factor
        kc = biochemical_parameters["kc25_pa"]*arrhenius_response(
            temperature, biochemical_parameters["ha_kc_j_mol"])
        ko = biochemical_parameters["ko25_pa"]*arrhenius_response(
            temperature, biochemical_parameters["ha_ko_j_mol"])
        gamma = biochemical_parameters["gamma25_pa"]*arrhenius_response(
            temperature, biochemical_parameters["ha_gamma_j_mol"])
        tp = biochemical_parameters["tp_vcmax_ratio"]*class_input["vcmax25"]*vcmax_factor
        rd = peaked_rd(class_input["rd25"], temperature)
        qsat = saturation_specific_humidity(temperature, pressure)
        es_leaf = qsat*pressure/(0.622+0.378*qsat)
        e_can = qcan*pressure/(0.622+0.378*qcan)
        vpd_kpa = (es_leaf-e_can)/1000.0
        if vpd_kpa <= 0.0:
            raise ValueError("nonpositive solved surface VPD")

        def ci_residual(ci: float) -> tuple[float, dict]:
            oxygen = biochemical_parameters["oxygen_partial_pressure_pa"]
            ipsii = (0.5*biochemical_parameters["electron_quantum_yield"]
                     * biochemical_parameters["par_photon_umol_per_j"]*par)
            electron = smaller_quadratic_root(
                biochemical_parameters["electron_curvature"], -(ipsii+jmax),
                ipsii*jmax) if ipsii > 0.0 else 0.0
            ac = vcmax*(ci-gamma)/(ci+kc*(1.0+oxygen/ko))
            aj = electron*(ci-gamma)/(4.0*ci+8.0*gamma)
            ap = 3.0*tp
            ai = smaller_quadratic_root(
                biochemical_parameters["ac_aj_curvature"], -(ac+aj), ac*aj)
            ag = smaller_quadratic_root(
                biochemical_parameters["ag_ap_curvature"], -(ai+ap), ai*ap)
            an = ag-rd
            cs = ca-1.4*rb*R_GAS*temperature*an*1.0e-6
            if cs <= 0.0:
                raise ValueError("nonpositive surface carbon dioxide")
            gs_pot_umol = (forcing["g0_umol_m2_s"] if an <= 0.0 else
                forcing["g0_umol_m2_s"]
                + 1.6*(1.0+forcing["medlyn_g1_kpa_sqrt"]/math.sqrt(vpd_kpa))
                * an/(cs/pressure))
            gs_umol = (forcing["g0_umol_m2_s"]
                       + beta_by_class[name]
                       *(gs_pot_umol-forcing["g0_umol_m2_s"]))
            gs_ms = gs_umol*1.0e-6*R_GAS*temperature/pressure
            if gs_ms <= 0.0:
                raise ValueError("nonpositive stomatal conductance")
            rs = 1.0/gs_ms
            predicted = ca-(1.4*rb+1.6*rs)*R_GAS*temperature*an*1.0e-6
            return ci-predicted, {"ag": ag, "an": an, "ac": ac, "aj": aj,
                "ap": ap, "ai": ai, "electron_transport": electron,
                "ipsii": ipsii, "vcmax": vcmax, "jmax": jmax, "tp": tp,
                "kc_pa": kc, "ko_pa": ko, "gamma_pa": gamma,
                "temperature_response_factors": {"vcmax": vcmax_factor,
                    "jmax": jmax_factor,
                    "kc": kc/biochemical_parameters["kc25_pa"],
                    "ko": ko/biochemical_parameters["ko25_pa"],
                    "gamma": gamma/biochemical_parameters["gamma25_pa"],
                    "tp": vcmax_factor},
                "rd": rd, "cs_pa": cs, "vpd_kpa": vpd_kpa,
                "gs_potential_umol_m2_s": gs_pot_umol,
                "gs_umol_m2_s": gs_umol, "gs_m_s": gs_ms, "rs_s_m": rs}

        ci, state, iterations, bracket = brent_dekker(ci_residual, gamma, ca)
        return {**state, "ci_pa": ci, "ci_iterations": iterations,
                "ci_initial_bracket_pa": [gamma, ca],
                "ci_bracket_pa": bracket, "qsat_kg_kg": qsat,
                "leaf_temperature_k": temperature}

    def residual_and_detail(x: list[float]) -> tuple[list[float], dict]:
        tsun, tshade, twet, tstem, tcan, qcan = x
        if not all(273.15 <= value <= 373.15 for value in x[:5]) or qcan < 0.0:
            raise ValueError("canopy energy domain")
        rho = pressure/(forcing["rdry_j_kg_k"]*tcan)
        sun = class_state("sun", tsun, qcan)
        shade = class_state("shade", tshade, qcan)
        states = {"sun": sun, "shade": shade}
        leaf_residuals, transpiration, component_scales = [], {}, []
        sigma, cp, latent = 5.670374419e-8, forcing["cp_air_j_kg_k"], forcing["latent_heat_j_kg"]
        for name, temperature in (("sun", tsun), ("shade", tshade)):
            area, state = dry_areas[name], states[name]
            flux = rho*(state["qsat_kg_kg"]-qcan)/(rb+state["rs_s_m"])*area
            transpiration[name] = flux
            sw = classes[name]["absorbed_shortwave_w_m2_tile"]*(1.0-wet_fraction)
            lw = forcing["leaf_emissivity"]*area*(forcing["longwave_down_w_m2"]
                + forcing["longwave_up_w_m2"]-2.0*sigma*temperature**4)
            sensible = rho*cp*gb_leaf*area*(temperature-tcan)
            leaf_residuals.append(sw+lw-sensible-latent*flux)
            component_scales.append(max(1.0, abs(sw)+abs(lw)+abs(sensible)
                                        + abs(latent*flux)))
        qsat_wet = saturation_specific_humidity(twet, pressure)
        wet_potential = rho*gb_wet*(qsat_wet-qcan)*wet_area
        store_cap = forcing["canopy_liquid_kg_m2_tile"]/forcing["dt_s"]
        wet_actual = min(wet_potential, store_cap) if wet_potential >= 0.0 else wet_potential
        wet_cap_active = wet_potential > store_cap
        wet_sw = wet_fraction*(math.fsum(value["absorbed_shortwave_w_m2_tile"]
                                        for value in classes.values())
                                + forcing["stem_absorbed_shortwave_w_m2_tile"])
        wet_lw = forcing["wet_emissivity"]*wet_area*(forcing["longwave_down_w_m2"]
            + forcing["longwave_up_w_m2"]-2.0*sigma*twet**4)
        wet_h = rho*cp*gb_wet*wet_area*(twet-tcan)
        wet_residual = wet_sw+wet_lw-wet_h-latent*wet_actual
        component_scales.append(max(1.0, abs(wet_sw)+abs(wet_lw)+abs(wet_h)
                                    + abs(latent*wet_actual)))
        stem_sw = (1.0-wet_fraction)*forcing["stem_absorbed_shortwave_w_m2_tile"]
        stem_lw = forcing["stem_emissivity"]*dry_stem_area*(forcing["longwave_down_w_m2"]
            + forcing["longwave_up_w_m2"]-2.0*sigma*tstem**4)
        stem_h = rho*cp*gb_stem*dry_stem_area*(tstem-tcan)
        stem_residual = stem_sw+stem_lw-stem_h
        component_scales.append(max(1.0, abs(stem_sw)+abs(stem_lw)+abs(stem_h)))
        rah, raw = forcing["rah_s_m"], forcing["raw_s_m"]
        heat_terms = ((tcan-forcing["air_temperature_k"])/rah
            - math.fsum(gb_leaf*dry_areas[name]*(states[name]["leaf_temperature_k"]-tcan)
                        for name in ("sun", "shade"))
            - gb_wet*wet_area*(twet-tcan)-gb_stem*dry_stem_area*(tstem-tcan))
        heat_balance = rho*cp*heat_terms
        component_scales.append(max(1.0, abs(rho*cp*(tcan-forcing["air_temperature_k"])/rah)
            + math.fsum(abs(rho*cp*gb_leaf*dry_areas[name]
                            *(states[name]["leaf_temperature_k"]-tcan))
                        for name in ("sun", "shade"))
            + abs(rho*cp*gb_wet*wet_area*(twet-tcan))
            + abs(rho*cp*gb_stem*dry_stem_area*(tstem-tcan))))
        vapor_atmosphere = rho*(qcan-forcing["air_specific_humidity_kg_kg"])/raw
        vapor_balance = vapor_atmosphere-math.fsum(transpiration.values())-wet_actual
        vapor_scale = max(1.0e-12, abs(vapor_atmosphere), abs(wet_actual),
                          *(abs(value) for value in transpiration.values()))
        return [*leaf_residuals, wet_residual, stem_residual, heat_balance, vapor_balance], {
            "sun": sun, "shade": shade, "transpiration": transpiration,
            "wet_potential_kg_m2_s": wet_potential,
            "wet_actual_kg_m2_s": wet_actual, "wet_store_cap_active": wet_cap_active,
            "wet_store_cap_kg_m2_s": store_cap, "wet_area": wet_area,
            "dry_stem_area": dry_stem_area, "rho_air_kg_m3": rho,
            "energy_component_scales_w_m2": component_scales,
            "vapor_scale_kg_m2_s": vapor_scale}

    x = [classes["sun"]["temperature_start_k"], classes["shade"]["temperature_start_k"],
         forcing["wet_temperature_start_k"], forcing["stem_temperature_start_k"],
         forcing["canopy_air_temperature_start_k"], forcing["qcan_start_kg_kg"]]
    epsilon = 2.220446049250313e-16
    backtracking, last_temperature_step = 0, None
    for iteration in range(max_iterations+1):
        residual, detail = residual_and_detail(x)
        normalized = [residual[index]/(1.0e-6+1.0e-10
                                       *detail["energy_component_scales_w_m2"][index])
                      for index in range(5)]
        normalized += [residual[5]/(1.0e-12+1.0e-9*detail["vapor_scale_kg_m2_s"])]
        norm = max(abs(value) for value in normalized)
        if norm <= 1.0 and (last_temperature_step is None
                            or last_temperature_step <= 1.0e-8):
            break
        if iteration == max_iterations:
            raise NumericalSolveError("canopy energy iteration limit", normalized,
                                      iteration, last_temperature_step,
                                      backtracking, locals().get("pivot"),
                                      locals().get("matrix_norm"))
        jacobian = [[0.0]*6 for _ in range(6)]
        unit_scales = [1.0]*5+[1.0e-3]
        for column in range(6):
            step = math.sqrt(epsilon)*max(abs(x[column]), unit_scales[column])
            plus, minus = x[:], x[:]
            plus[column] += step
            minus[column] -= step
            rplus, _ = residual_and_detail(plus)
            rminus, _ = residual_and_detail(minus)
            for row in range(6):
                jacobian[row][column] = (rplus[row]-rminus[row])/(2.0*step)
        delta, pivot, matrix_norm = solve_linear(jacobian, [-value for value in residual])
        if norm <= 1.0 and max(abs(value) for value in delta[:5]) <= 1.0e-8:
            last_temperature_step = max(abs(value) for value in delta[:5])
            break
        accepted = False
        for half in range(21):
            trial = [value+change/(2.0**half) for value, change in zip(x, delta)]
            try:
                trial_residual, _ = residual_and_detail(trial)
            except (ValueError, ArithmeticError):
                continue
            trial_detail = residual_and_detail(trial)[1]
            trial_normalized = [trial_residual[index]/(1.0e-6+1.0e-10
                *trial_detail["energy_component_scales_w_m2"][index])
                                for index in range(5)]
            trial_normalized += [trial_residual[5]/(1.0e-12+1.0e-9
                *trial_detail["vapor_scale_kg_m2_s"])]
            if max(abs(value) for value in trial_normalized) < norm:
                last_temperature_step = max(abs(change)/(2.0**half)
                                            for change in delta[:5])
                x, accepted, backtracking = trial, True, backtracking+half
                break
        if not accepted:
            raise NumericalSolveError("canopy energy backtracking limit", normalized,
                                      iteration, max(abs(value) for value in delta[:5]),
                                      backtracking, pivot, matrix_norm)
    residual, detail = residual_and_detail(x)
    return {"sun": {**detail["sun"],
                    "transpiration_kg_m2_tile_s": detail["transpiration"]["sun"]},
            "shade": {**detail["shade"],
                      "transpiration_kg_m2_tile_s": detail["transpiration"]["shade"]},
            "wet_surface_temperature_k": x[2], "dry_stem_temperature_k": x[3],
            "canopy_air_temperature_k": x[4],
            "canopy_air_specific_humidity_kg_kg": x[5],
            "wet_potential_kg_m2_s": detail["wet_potential_kg_m2_s"],
            "wet_actual_kg_m2_s": detail["wet_actual_kg_m2_s"],
            "wet_store_cap_active": detail["wet_store_cap_active"],
            "normalized_residuals": normalized, "iterations": iteration,
            "temperature_step_k": last_temperature_step,
            "backtracking_count": backtracking, "pivot_magnitude": pivot if iteration else None,
            "matrix_norm": matrix_norm if iteration else None,
            "leaf_potential_inputs_mm": list(leaf_potentials),
            "class_beta_hyd": beta_by_class}


class SingularMatrixError(ArithmeticError):
    def __init__(self, pivot: float, matrix_norm: float):
        super().__init__("singular Newton system")
        self.pivot = pivot
        self.matrix_norm = matrix_norm


class NumericalSolveError(ArithmeticError):
    def __init__(self, message: str, normalized: list[float], iterations: int,
                 step: float | None, backtracking: int,
                 pivot: float | None, matrix_norm: float | None):
        super().__init__(message)
        self.normalized = normalized
        self.iterations = iterations
        self.step = step
        self.backtracking = backtracking
        self.pivot = pivot
        self.matrix_norm = matrix_norm


def solve_linear(matrix: list[list[float]], right: list[float]) -> tuple[list[float], float, float]:
    a = [row[:] + [value] for row, value in zip(matrix, right)]
    matrix_norm = max(math.fsum(abs(x) for x in row) for row in matrix)
    pivot_min = math.inf
    for column in range(len(right)):
        pivot = max(range(column, len(right)), key=lambda row: abs(a[row][column]))
        magnitude = abs(a[pivot][column])
        pivot_min = min(pivot_min, magnitude)
        if magnitude < 64.0*2.220446049250313e-16*matrix_norm:
            raise SingularMatrixError(magnitude, matrix_norm)
        a[column], a[pivot] = a[pivot], a[column]
        for row in range(column+1, len(right)):
            ratio = a[row][column]/a[column][column]
            for j in range(column, len(right)+1):
                a[row][j] -= ratio*a[column][j]
    solution = [0.0]*len(right)
    for row in range(len(right)-1, -1, -1):
        solution[row] = (a[row][-1]-math.fsum(a[row][j]*solution[j]
                                             for j in range(row+1, len(right))))/a[row][row]
    return solution, pivot_min, matrix_norm


def hydraulic_fluxes(x: list[float], case: dict) -> dict:
    sun, shade, stem, root, beta_sun, beta_shade = x
    p = case["parameters"]
    leaf_factor = p["k1_max"]/p["stem_to_leaf_path_m"]
    q1sun = (leaf_factor*p["sun_leaf_area"]
             * vulnerability(stem, p["p50_xylem"], p["ck"])*(stem-sun))
    q1shade = (leaf_factor*p["shade_leaf_area"]
               * vulnerability(stem, p["p50_xylem"], p["ck"])*(stem-shade))
    height, gravity = p["height_m"], 1000.0*p["height_m"]
    q2 = (p["k2_max"]/height)*vulnerability(root, p["p50_xylem"], p["ck"])*p["sai"]*(root-stem-gravity)
    q3 = []
    for layer in case["layers"]:
        if not layer["accessible"] or layer["frozen"] or layer["root_fraction"] == 0.0:
            q3.append({"layer_id": layer["layer_id"], "flux": 0.0,
                       "kr_m_s": 0.0, "ks_m_s": 0.0,
                       "k3_series_m_s": 0.0, "rai_m2_m2": 0.0,
                       "soil_vulnerability": 0.0})
        else:
            soil_vulnerability = vulnerability(layer["soil_potential_mm"],
                                                p["p50_root"], p["ck"])
            kr = (p["k3_max_m_s"]/layer["z3_m"])*soil_vulnerability
            ks = layer["ksoil_m2_s"] / layer["dxroot_m"]
            k3 = kr*ks/(kr+ks)
            rai = ((p["lai"]+p["sai"])*layer["root_fraction"]
                   *p["root_to_leaf_area"])
            flux = k3*rai*(layer["soil_potential_mm"]-root
                           + layer["gravity_head_mm"])
            q3.append({"layer_id": layer["layer_id"], "flux": flux,
                       "kr_m_s": kr, "ks_m_s": ks,
                       "k3_series_m_s": k3, "rai_m2_m2": rai,
                       "soil_vulnerability": soil_vulnerability,
                       "z3_m": layer["z3_m"], "dxroot_m": layer["dxroot_m"],
                       "root_fraction": layer["root_fraction"]})
    vsun = vulnerability(sun, p["p50_leaf"], p["ck"])
    vshade = vulnerability(shade, p["p50_leaf"], p["ck"])
    emax_sun, emax_shade = case["emax"]["sun"], case["emax"]["shade"]
    energy = coupled_canopy_energy(case, (beta_sun, beta_shade), (sun, shade))
    sun_response, shade_response = energy["sun"], energy["shade"]
    residuals = [q1sun-sun_response["transpiration_kg_m2_tile_s"],
                 q1shade-shade_response["transpiration_kg_m2_tile_s"],
                 sun_response["transpiration_kg_m2_tile_s"]-emax_sun*vsun,
                 shade_response["transpiration_kg_m2_tile_s"]-emax_shade*vshade,
                 q2-q1sun-q1shade,
                 math.fsum(row["flux"] for row in q3)-q2]
    return {"q1_sun": q1sun, "q1_shade": q1shade, "q2": q2,
            "q3": q3, "v_sun": vsun, "v_shade": vshade,
            "residuals": residuals,
            "gas_energy_transpiration_sun": sun_response["transpiration_kg_m2_tile_s"],
            "gas_energy_transpiration_shade": shade_response["transpiration_kg_m2_tile_s"],
            "sun_gas_energy_state": sun_response,
            "shade_gas_energy_state": shade_response,
            "canopy_energy_state": {key: value for key, value in energy.items()
                                    if key not in {"sun", "shade"}},
            "stem_path_length_m": height, "stem_gravity_head_mm": gravity}


def coupled_solve(case: dict, start: list[float], max_iterations: int = 50) -> dict:
    x = start[:]
    backtracking, pivot_min, matrix_norm = 0, None, None
    last_potential_step = None
    residual_history = []
    for iteration in range(max_iterations+1):
        flux = hydraulic_fluxes(x, case)
        residual = flux["residuals"]
        scale = max(1.0e-12, case["emax"]["sun"], case["emax"]["shade"],
                    abs(flux["q1_sun"]), abs(flux["q1_shade"]), abs(flux["q2"]),
                    *(abs(row["flux"]) for row in flux["q3"]))
        tolerance = 1.0e-12+1.0e-9*scale
        normalized = [value/tolerance for value in residual]
        norm = max(abs(value) for value in normalized)
        residual_history.append(norm)
        if norm <= 1.0 and (last_potential_step is None
                            or last_potential_step <= 1.0e-7):
            if any(row["flux"] < -1.0e-14 for row in flux["q3"]):
                raise ValueError("hydraulic redistribution unsupported")
            dt, fraction = case["dt_s"], case["tile_fraction"]
            requests = [{"layer_id": row["layer_id"],
                         "amount_kg_h2o_m2_stand_ground": fraction*row["flux"]*dt}
                        for row in flux["q3"]]
            return {"iterations": iteration, "solution": {
                        "sun_leaf_potential_mm": x[0],
                        "shade_leaf_potential_mm": x[1],
                        "stem_potential_mm": x[2],
                        "root_node_potential_mm": x[3],
                        "beta_hyd_sun": x[4], "beta_hyd_shade": x[5],
                        "beta_hyd": ((case["emax"]["sun"]*x[4]
                                      + case["emax"]["shade"]*x[5])
                                     / (case["emax"]["sun"]+case["emax"]["shade"])
                                     if case["emax"]["sun"]+case["emax"]["shade"] > 0.0
                                     else 1.0)},
                    "fluxes": {key: value for key, value in flux.items()
                               if key != "residuals"},
                    "closures": {
                        "sun_gas_energy_minus_q1":
                            flux["gas_energy_transpiration_sun"]-flux["q1_sun"],
                        "shade_gas_energy_minus_q1":
                            flux["gas_energy_transpiration_shade"]-flux["q1_shade"],
                        "q1_sum_minus_q2": flux["q1_sun"]+flux["q1_shade"]-flux["q2"],
                        "q2_minus_q3_sum": flux["q2"]-math.fsum(
                            row["flux"] for row in flux["q3"]),
                    },
                    "water_requests": requests,
                    "normalized_residuals": [
                        {"identity": identity, "raw_kg_m2_tile_s": value,
                         "scale_kg_m2_tile_s": scale, "tolerance": tolerance,
                         "normalized": value/tolerance}
                        for identity, value in zip(("sun_gas_minus_q1",
                            "shade_gas_minus_q1", "sun_gas_minus_vulnerability_demand",
                            "shade_gas_minus_vulnerability_demand", "q1_sum_minus_q2",
                            "q3_sum_minus_q2"), residual)],
                    "residual_norm_history": residual_history,
                    "backtracking_count": backtracking,
                    "potential_step_mm": last_potential_step,
                    "pivot_magnitude": pivot_min, "matrix_norm": matrix_norm}
        if iteration == max_iterations:
            return {"failure": "iteration_limit", "iterations": iteration,
                    "residual_norm_history": residual_history,
                    "diagnostics": numerical_failure(
                                                      "outer_gas_energy_hydraulic_coupling",
                                                      iteration,
                                                      normalized, last_potential_step,
                                                      backtracking,
                                                      pivot_magnitude=pivot_min,
                                                      matrix_norm=matrix_norm),
                    "candidate": None, "last_iterate": None}
        unit_scales = [1000.0, 1000.0, 1000.0, 1000.0, 1.0, 1.0]
        steps = [math.sqrt(2.220446049250313e-16)*max(abs(value), unit)
                 for value, unit in zip(x, unit_scales)]
        jacobian = [[0.0]*6 for _ in range(6)]
        for column in range(6):
            plus, minus = x[:], x[:]
            plus[column] += steps[column]
            minus[column] -= steps[column]
            rp = hydraulic_fluxes(plus, case)["residuals"]
            rm = hydraulic_fluxes(minus, case)["residuals"]
            for row in range(6):
                jacobian[row][column] = (rp[row]-rm[row])/(2.0*steps[column])
        try:
            delta, pivot_min, matrix_norm = solve_linear(jacobian, [-r for r in residual])
        except SingularMatrixError as error:
            return {"failure": "singular_jacobian", "iterations": iteration,
                    "diagnostics": numerical_failure("hydraulic_system", iteration,
                                                      normalized, None, backtracking,
                                                      pivot_magnitude=error.pivot,
                                                      matrix_norm=error.matrix_norm),
                    "candidate": None, "last_iterate": None}
        if norm <= 1.0 and max(abs(value) for value in delta[:4]) <= 1.0e-7:
            last_potential_step = max(abs(value) for value in delta[:4])
            continue
        accepted = False
        for exponent in range(21):
            factor = 0.5**exponent
            trial = [value+factor*change for value, change in zip(x, delta)]
            if 0.0 <= trial[4] <= 1.0 and 0.0 <= trial[5] <= 1.0:
                trial_flux = hydraulic_fluxes(trial, case)
                trial_scale = max(
                    1.0e-12, case["emax"]["sun"], case["emax"]["shade"],
                    abs(trial_flux["q1_sun"]), abs(trial_flux["q1_shade"]),
                    abs(trial_flux["q2"]),
                    *(abs(row["flux"]) for row in trial_flux["q3"]))
                trial_tolerance = 1.0e-12+1.0e-9*trial_scale
                trial_norm = max(abs(value)/trial_tolerance
                                 for value in trial_flux["residuals"])
                if trial_norm < norm:
                    last_potential_step = max(abs(change)*factor
                                              for change in delta[:4])
                    x, accepted = trial, True
                    backtracking += exponent
                    break
        if not accepted:
            return {"failure": "backtracking_limit", "iterations": iteration,
                    "diagnostics": numerical_failure("hydraulic_system", iteration,
                                                      normalized, max(abs(d) for d in delta),
                                                      backtracking),
                    "candidate": None, "last_iterate": None}
    raise AssertionError("unreachable")


def numerical_failure(solve: str, iterations: int, residuals: list[float],
                      step: float | None, backtracking: int,
                      pivot_magnitude: float | None = None,
                      matrix_norm: float | None = None) -> dict:
    identities = {
        "hydraulic_system": ["sun_gas_minus_q1", "shade_gas_minus_q1",
            "sun_gas_minus_vulnerability_demand",
            "shade_gas_minus_vulnerability_demand", "q1_sum_minus_q2",
            "q3_sum_minus_q2"],
        "sun_ci": (["sun_ci"] if len(residuals) <= 1
                   else ["sun_ci_bracket_low", "sun_ci_bracket_high"]),
        "shade_ci": (["shade_ci"] if len(residuals) <= 1
                     else ["shade_ci_bracket_low", "shade_ci_bracket_high"]),
        "canopy_energy": ["sun_leaf_energy", "shade_leaf_energy",
            "wet_surface_energy", "dry_stem_energy", "canopy_air_heat",
            "canopy_air_vapor"],
        "outer_gas_energy_hydraulic_coupling": [f"outer_{index}"
                                                  for index in range(len(residuals))],
    }.get(solve, [f"residual_{index}" for index in range(len(residuals))])
    labeled = [{"identity": identity, "normalized": value}
               for identity, value in zip(identities, residuals)]
    return {"model_definition_sha256": "BOUND_BY_V3_DEFINITION_NOT_ORACLE",
            "transaction_id": "tx-v3-vector-17", "occupancy_id": "upper@tile-a",
            "pass": "potential", "solve": solve, "iterations": iterations,
            "residual_norms": labeled, "step_norm": step,
            "backtracking_count": backtracking, "active_bounds": [],
            "active_water_caps": [], "bracket": None,
            "pivot_magnitude": pivot_magnitude, "matrix_norm": matrix_norm}


def hydraulic_vectors() -> dict:
    kappa, u_ref, zref, displacement, z0m = 0.4, 3.7, 24.0, 8.1, 1.25
    ustar = kappa*u_ref/math.log((zref-displacement)/z0m)
    gb_leaf = 0.01*math.sqrt(ustar/0.045)
    gb_wet = 0.01*math.sqrt(ustar/0.16)
    gb_stem = 0.01*math.sqrt(ustar/0.34)
    rah = (math.log((zref-displacement)/z0m)*math.log((zref-displacement)/0.12)
           /(kappa*kappa*u_ref))
    raw = (math.log((zref-displacement)/z0m)*math.log((zref-displacement)/0.08)
           /(kappa*kappa*u_ref))
    case = {
        "tile_fraction": 0.38, "dt_s": 1800.0,
        "gas_energy": {"pressure_pa": 101325.0, "ca_pa": 42.0,
                       "derived_u_star_m_s": ustar,
                       "gb_leaf_m_s": gb_leaf, "gb_wet_m_s": gb_wet,
                       "gb_stem_m_s": gb_stem,
                       "g0_umol_m2_s": 25.0, "medlyn_g1_kpa_sqrt": 3.5,
                       "cp_air_j_kg_k": 1004.64,
                       "latent_heat_j_kg": 2501000.0,
                       "rdry_j_kg_k": 287.05, "air_temperature_k": 296.0,
                       "air_specific_humidity_kg_kg": 0.0102,
                       "reference_wind_operands": {"kappa": kappa,
                           "u_ref_m_s": u_ref, "z_ref_m": zref,
                           "displacement_m": displacement, "z0m_m": z0m,
                           "z0h_m": 0.12, "z0q_m": 0.08},
                       "rah_s_m": rah, "raw_s_m": raw,
                       "leaf_emissivity": 0.96,
                       "wet_emissivity": 0.97, "stem_emissivity": 0.94,
                       "longwave_down_w_m2": 395.0, "longwave_up_w_m2": 430.0},

        "classes": {
            "sun": {"leaf_area": 1.5, "absorbed_par_w_m2_leaf": 175.0,
                    "absorbed_shortwave_w_m2_tile": 315.0,
                    "vcmax25": 62.0, "jmax25": 108.0, "rd25": 1.15,
                    "temperature_start_k": 296.2},
            "shade": {"leaf_area": 1.2083333333333333,
                      "absorbed_par_w_m2_leaf": 48.0,
                      "absorbed_shortwave_w_m2_tile": 94.0,
                      "vcmax25": 41.0, "jmax25": 74.0, "rd25": 0.81,
                      "temperature_start_k": 295.4}},
        "biochemical_parameters": {"kc25_pa": 40.49, "ko25_pa": 27840.0,
            "gamma25_pa": 4.275, "ha_vcmax_j_mol": 65330.0,
            "hd_vcmax_j_mol": 200000.0, "entropy_vcmax_j_mol_k": 650.0,
            "ha_jmax_j_mol": 43540.0, "hd_jmax_j_mol": 200000.0,
            "entropy_jmax_j_mol_k": 650.0, "ha_kc_j_mol": 79430.0,
            "ha_ko_j_mol": 36380.0, "ha_gamma_j_mol": 37830.0,
            "tp_vcmax_ratio": 0.167, "oxygen_partial_pressure_pa": 20265.0,
            "electron_quantum_yield": 0.85, "par_photon_umol_per_j": 4.6,
            "electron_curvature": 0.7, "ac_aj_curvature": 0.98,
            "ag_ap_curvature": 0.95},
        "parameters": {"k1_max": 1.2e-6, "stem_to_leaf_path_m": 1.0,
                       "sun_leaf_area": 1.5,
                       "shade_leaf_area": 1.2083333333333333,
                       "k2_max": 4.2e-6, "height_m": 12.5, "sai": 0.72,
                       "lai": 2.708333333333333, "k3_max_m_s": 5.0e-5,
                       "root_to_leaf_area": 1.8, "p50_root": -14000.0,
                       "p50_xylem": -7200.0, "p50_leaf": -9800.0, "ck": 2.0},
        "layers": [
            {"layer_id": "soil-1", "soil_potential_mm": 100.0,
             "gravity_head_mm": 120.0,
             "root_fraction": 0.62, "z3_m": 0.32,
             "ksoil_m2_s": 6.0e-11, "dxroot_m": 0.18,
             "accessible": True, "frozen": False},
            {"layer_id": "soil-2", "soil_potential_mm": 100.0,
             "gravity_head_mm": 360.0,
             "root_fraction": 0.38, "z3_m": 0.55,
             "ksoil_m2_s": 4.5e-11, "dxroot_m": 0.24,
             "accessible": True, "frozen": False},
            {"layer_id": "soil-dry", "soil_potential_mm": -9000.0,
             "gravity_head_mm": 600.0,
             "root_fraction": 0.0, "z3_m": 0.8,
             "ksoil_m2_s": 2.0e-7, "dxroot_m": 0.31,
             "accessible": False, "frozen": False},
            {"layer_id": "soil-frozen", "soil_potential_mm": -1100.0,
             "gravity_head_mm": 740.0,
             "root_fraction": 0.0, "z3_m": 1.1,
             "ksoil_m2_s": 1.0e-7, "dxroot_m": 0.4,
             "accessible": True, "frozen": True},
        ],
    }
    case["gas_energy"].update({"stem_area": 0.72,
        "stem_absorbed_shortwave_w_m2_tile": 77.0, "wet_fraction": 0.37,
        "canopy_liquid_kg_m2_tile": 0.018, "dt_s": case["dt_s"],
        "wet_temperature_start_k": 295.6, "stem_temperature_start_k": 295.2,
        "canopy_air_temperature_start_k": 295.8, "qcan_start_kg_kg": 0.011})
    internal_energy = coupled_canopy_energy(case, (1.0, 1.0), (-5900.0, -5450.0))
    internal_sun, internal_shade = internal_energy["sun"], internal_energy["shade"]
    case["emax"] = {"sun": internal_sun["transpiration_kg_m2_tile_s"],
                    "shade": internal_shade["transpiration_kg_m2_tile_s"]}
    start_a = [-5900.0, -5450.0, -4300.0, -2850.0, 0.68, 0.66]
    start_b = [-8100.0, -7600.0, -5700.0, -3300.0, 0.35, 0.31]
    solved_a, solved_b = coupled_solve(case, start_a), coupled_solve(case, start_b)
    if "failure" in solved_a or "failure" in solved_b:
        raise AssertionError(f"coupled fixture did not converge: {solved_a}, {solved_b}")
    for field, value in solved_a["solution"].items():
        if not math.isclose(value, solved_b["solution"][field], rel_tol=2e-9, abs_tol=2e-7):
            raise AssertionError("alternate warm starts differ")
    singular = json.loads(json.dumps(case))
    singular["parameters"]["k1_max"] = 0.0
    singular["parameters"]["k2_max"] = 0.0
    for layer in singular["layers"]:
        layer["accessible"] = False
    singular_result = coupled_solve(singular, start_a)
    limit_result = coupled_solve(case, start_b, max_iterations=1)
    energy_failures = []
    energy_failure_cases = []
    domain_case = json.loads(json.dumps(case))
    domain_case["gas_energy"]["canopy_air_temperature_start_k"] = 250.0
    energy_failure_cases.append(("domain", domain_case, 50, "canopy energy domain"))
    energy_failure_cases.append(("iteration_limit", case, 0,
                                 "canopy energy iteration limit"))
    for failure_kind, failure_case, limit, expected_error in energy_failure_cases:
        try:
            coupled_canopy_energy(failure_case, (0.6, 0.6), (-5900.0, -5450.0),
                                  max_iterations=limit)
            raise AssertionError("canopy energy failure not exercised")
        except (ValueError, ArithmeticError) as error:
            if str(error) != expected_error:
                raise
            normalized = error.normalized if isinstance(error, NumericalSolveError) else []
            iterations = error.iterations if isinstance(error, NumericalSolveError) else 0
            step = error.step if isinstance(error, NumericalSolveError) else None
            backtracks = error.backtracking if isinstance(error, NumericalSolveError) else 0
            pivot = error.pivot if isinstance(error, NumericalSolveError) else None
            matrix_norm_failure = (error.matrix_norm
                                   if isinstance(error, NumericalSolveError) else None)
            energy_failures.append({"failure_kind": failure_kind,
                "typed_error": expected_error,
                "diagnostics": numerical_failure("canopy_energy", iterations,
                    normalized, step, backtracks, pivot, matrix_norm_failure),
                "candidate": None, "last_iterate": None,
                "executed_by": "coupled_canopy_energy"})
    redistribution = json.loads(json.dumps(case))
    redistribution["layers"][1]["soil_potential_mm"] = -20000.0
    redistribution_fluxes = hydraulic_fluxes(start_a, redistribution)["q3"]
    if not any(row["flux"] < 0.0 for row in redistribution_fluxes):
        raise AssertionError("redistribution poison did not create negative layer flux")
    redistribution_outcome = "VEG-E-063 hydraulic_redistribution_unsupported"
    return {"units": {"potential": "mm H2O", "flux": "kg H2O m-2 tile-ground s-1",
                       "request": "kg H2O m-2 stand-ground interval-1"},
            "operands": case, "internal_maximum_evaluation": {
                "beta_hyd": 1.0, "emax": case["emax"],
                "sun_gas_energy_state": internal_sun,
                "shade_gas_energy_state": internal_shade,
                "canopy_energy_state": {key: value for key, value in internal_energy.items()
                                        if key not in {"sun", "shade"}},
                "accepted_state_or_request": False},
            "zero_maximum_demand_exact_branch": {
                "emax_sun": 0.0, "emax_shade": 0.0,
                "beta_hyd_sun": 1.0, "beta_hyd_shade": 1.0,
                "persisted_beta_hyd": 1.0,
                "accepted_class_demand": [0.0, 0.0],
                "hydraulic_fluxes": [0.0, 0.0, 0.0],
                "division_evaluated": False},
            "accepted_uncapped_stage_a": solved_a,
            "alternate_warm_start": {"start": start_b, "result": solved_b},
            "dry_and_frozen_exact_zero_layers": ["soil-dry", "soil-frozen"],
            "singular_jacobian": singular_result,
            "iteration_limit": limit_result,
            "executed_canopy_energy_failures": energy_failures,
            "redistribution_poison": {"operands": redistribution,
                                      "candidate_layer_fluxes": redistribution_fluxes,
                                      "expected": redistribution_outcome}}


def migration_vectors() -> dict:
    def migrate(entries: list[float]) -> dict:
        if not entries:
            return {"status": "unresolved", "field": "root_node_potential_mm",
                    "reason": "ambiguous_v2_layer_root_warm_starts"}
        first = struct.pack(">d", entries[0])
        if all(struct.pack(">d", value) == first for value in entries[1:]):
            return {"status": "complete", "root_node_potential_mm": entries[0]}
        return {"status": "unresolved", "field": "root_node_potential_mm",
                "reason": "ambiguous_v2_layer_root_warm_starts"}
    return {"bitwise_identical": {"input": [-4200.0, -4200.0, -4200.0],
                                    "expected": migrate([-4200.0]*3)},
            "numerically_equal_bitwise_distinct": {
                "input_hex": [struct.pack(">d", 0.0).hex(), struct.pack(">d", -0.0).hex()],
                "expected": migrate([0.0, -0.0])},
            "ambiguous": {"input": [-4200.0, -4200.000000000001],
                          "expected": migrate([-4200.0, -4200.000000000001])},
            "missing": {"input": [], "expected": migrate([])}}


def peaked_rd(rd25: float, temperature_k: float) -> float:
    return rd25*peaked_response(temperature_k, 46390.0, 150650.0, 490.0)


def arrhenius_response(temperature_k: float, activation_j_mol: float) -> float:
    require_finite(temperature_k, activation_j_mol)
    if temperature_k <= 0.0 or activation_j_mol <= 0.0:
        raise ValueError("Arrhenius domain")
    return math.exp(activation_j_mol/R_GAS*(1.0/298.15-1.0/temperature_k))


def peaked_response(temperature_k: float, activation_j_mol: float,
                    deactivation_j_mol: float, entropy_j_mol_k: float) -> float:
    require_finite(temperature_k, activation_j_mol, deactivation_j_mol,
                   entropy_j_mol_k)
    if min(temperature_k, activation_j_mol, deactivation_j_mol,
           entropy_j_mol_k) <= 0.0:
        raise ValueError("peaked response domain")
    reference = 298.15
    def log_one_plus_exp(value: float) -> float:
        return (value+math.log1p(math.exp(-value)) if value > 0.0
                else math.log1p(math.exp(value)))
    log_factor = (activation_j_mol*(temperature_k-reference)
                  /(R_GAS*temperature_k*reference)
                  + log_one_plus_exp((reference*entropy_j_mol_k
                                      - deactivation_j_mol)/(R_GAS*reference))
                  - log_one_plus_exp((temperature_k*entropy_j_mol_k
                                      - deactivation_j_mol)/(R_GAS*temperature_k)))
    response = math.exp(log_factor)
    if not math.isfinite(response):
        raise ValueError("nonfinite peaked response")
    return response


def respiration_vectors() -> dict:
    operands = {"atkin_intercept_umol_co2_m2_leaf_s": 0.82,
                "leaf_n_kg_n_m2_leaf": 0.002,
                "t10_k": 293.15, "sun_leaf_temperature_k": 301.2,
                "shade_leaf_temperature_k": 294.4,
                "sun_leaf_area_m2_m2_tile": 1.31,
                "shade_leaf_area_m2_m2_tile": 1.09,
                "ag_sun_umol_co2_m2_leaf_s": 14.2,
                "ag_shade_umol_co2_m2_leaf_s": 5.7,
                "dt_s": 1800.0, "tile_fraction": 0.38,
                "molar_mass_c_kg_mol": 0.012011}
    n_g = 1000.0*operands["leaf_n_kg_n_m2_leaf"]
    t10_c = operands["t10_k"]-273.15
    rd25 = (operands["atkin_intercept_umol_co2_m2_leaf_s"]
            +0.2061*n_g-0.0402*t10_c)
    if rd25 <= 0.0:
        raise ValueError("nonpositive Atkin Rd25")
    rd_sun = peaked_rd(rd25, operands["sun_leaf_temperature_k"])
    rd_shade = peaked_rd(rd25, operands["shade_leaf_temperature_k"])
    an_sun = operands["ag_sun_umol_co2_m2_leaf_s"]-rd_sun
    an_shade = operands["ag_shade_umol_co2_m2_leaf_s"]-rd_shade
    debit = (operands["tile_fraction"]*operands["dt_s"]*1.0e-6
             * operands["molar_mass_c_kg_mol"]
             * (rd_sun*operands["sun_leaf_area_m2_m2_tile"]
                + rd_shade*operands["shade_leaf_area_m2_m2_tile"]))
    nonpositive = {"atkin_intercept_umol_co2_m2_leaf_s": 0.1,
                   "leaf_n_kg_n_m2_leaf": 0.0001, "t10_k": 303.15,
                   "expected": "VEG-E-085 nonpositive_atkin_rd25"}
    return {"units": {"rd": "umol CO2 m-2 leaf s-1",
                       "carbon_debit": "kg C m-2 stand-ground interval-1"},
            "operands": operands, "source_unit_conversion": {
                "leaf_n_g_n_m2_leaf": n_g, "t10_degc": t10_c,
                "atkin_result_units": "umol CO2 m-2 leaf s-1"},
            "results": {"rd25_umol_co2_m2_leaf_s": rd25, "rd_sun": rd_sun,
                        "rd_shade": rd_shade, "an_sun": an_sun,
                        "an_shade": an_shade,
                        "leaf_maintenance_carbon_debit_exact_once": debit,
                        "double_debit_poison": 2.0*debit},
            "zero_leaf_area_exact_branch": {
                "leaf_area_m2_m2_tile": 0.0, "rd25": 0.0,
                "rd_sun": 0.0, "rd_shade": 0.0,
                "carbon_debit": 0.0, "leaf_n_division_evaluated": False},
            "nonpositive_atkin_poison": nonpositive}


def poison_manifest(families: dict) -> dict:
    """Execute every release poison; no name-only declarations are emitted."""
    radiation, aero = families["radiation"], families["aerodynamics"]
    hydraulic, respiration = (families["hydraulic_potential_pass"],
                              families["leaf_respiration"])
    upper = radiation["operands"]["layers"][0]
    direct = radiation["two_rank"]["VIS"]["direct"]
    accepted = direct["occupancies"][0]
    leaf, stem = upper["optics"]["VIS"]["leaf"], upper["optics"]["VIS"]["stem"]
    hyd = hydraulic["accepted_uncapped_stage_a"]
    p, solution = hydraulic["operands"]["parameters"], hyd["solution"]
    root, stem_psi = solution["root_node_potential_mm"], solution["stem_potential_mm"]

    def numeric(accepted_value: float, rejected_value: float) -> dict:
        if math.isclose(accepted_value, rejected_value, rel_tol=1.0e-12, abs_tol=1.0e-15):
            raise AssertionError("poison failed to discriminate")
        return {"executed": True, "accepted": accepted_value,
                "rejected": rejected_value, "discriminates": True}

    def typed(error: str, operands: dict) -> dict:
        def owning_validator() -> None:
            if error == "VEG-E-081 nonpositive_u_star":
                value = operands.get("u_star_m_s", operands.get("u_ref_m_s"))
                if value is None or not math.isfinite(value) or value <= 0.0:
                    raise ValueError(error)
            elif error == "VEG-E-081 invalid_reference_height_geometry":
                if operands["z_ref_m"] <= operands["displacement_m"]+operands["z0m_m"]:
                    raise ValueError(error)
            elif error == "VEG-E-082 nonpositive_height":
                if operands["height_m"] <= 0.0:
                    raise ValueError(error)
            elif error == "ambiguous_v2_layer_root_warm_starts":
                entries = operands["entries"]
                if (not entries or any(struct.pack(">d", value)
                                       != struct.pack(">d", entries[0])
                                       for value in entries[1:])):
                    raise ValueError(error)
            elif error == "VEG-E-084 class_continuity_failure":
                if any(value != 0.0 for value in operands["class_residuals"]):
                    raise ValueError(error)
            elif error == "VEG-E-084 authorization_in_potential":
                if "authorization_kg_m2" in operands:
                    raise ValueError(error)
            elif error == "VEG-E-085 nonpositive_atkin_rd25":
                if operands["raw_rd25"] <= 0.0:
                    raise ValueError(error)
            else:
                raise AssertionError(f"no owning validator for {error}")
            raise AssertionError(f"owning validator accepted poison {error}")
        try:
            owning_validator()
        except ValueError as raised:
            if str(raised) != error:
                raise
        else:
            raise AssertionError(f"typed poison did not fail: {error}")
        return {"executed": True, "operands": operands, "typed_error": error,
                "candidate": None, "last_iterate": None,
                "executed_by": "owning_validator"}

    accepted_q2 = hyd["fluxes"]["q2"]
    def q2_variant(path: float, gravity: float) -> float:
        return (p["k2_max"]/path)*vulnerability(root, p["p50_xylem"], p["ck"])*p["sai"]*(root-stem_psi-gravity)

    accepted_total = hyd["fluxes"]["gas_energy_transpiration_sun"]+hyd["fluxes"]["gas_energy_transpiration_shade"]
    emax = hydraulic["internal_maximum_evaluation"]["emax"]
    aggregate_beta = solution["beta_hyd"]
    zero_lower_reflection = math.fsum(
        radiation_component([layer], "VIS", 0.67, 410.0 if index == 0 else 0.0,
                            0.0, 0.0)["top_reflected"]
        for index, layer in enumerate(radiation["operands"]["layers"]))
    direct_summed_reflection = math.fsum(
        radiation_component([layer], "VIS", 0.67, 410.0, 0.0, 0.14)["top_reflected"]
        for layer in radiation["operands"]["layers"])
    rd = respiration["results"]
    poisons = {
        "leaf_optics_for_all_plant_area": numeric(accepted["operands"]["rho_effective"], leaf["rho"]),
        "stem_optics_for_all_plant_area": numeric(accepted["operands"]["rho_effective"], stem["rho"]),
        "arithmetic_mean_optics": numeric(accepted["operands"]["rho_effective"], 0.5*(leaf["rho"]+stem["rho"])),
        "area_only_absorption_partition": numeric(accepted["operands"]["leaf_absorption_fraction"], accepted["operands"]["leaf_weight"]),
        "clumping_applied_twice": numeric(accepted["operands"]["k_eff"], accepted["operands"]["k_eff"]*upper["clumping_index"]),
        "clumping_omitted": numeric(accepted["operands"]["k_eff"], accepted["operands"]["beam_k_unclumped"]),
        "sunlit_plant_area_as_sunlit_leaf_area": numeric(accepted["results"]["leaf_sun_area"], accepted["results"]["leaf_sun_area"]/accepted["operands"]["leaf_weight"]),
        "stem_absorption_in_fvcb_par": numeric(accepted["results"]["absorbed_leaf_sun"], accepted["results"]["absorbed_leaf_sun"]+accepted["results"]["absorbed_stem"]),
        "vis_nir_swap": numeric(direct["occupancies"][0]["results"]["absorbed_plant"], radiation["two_rank"]["NIR"]["direct"]["occupancies"][0]["results"]["absorbed_plant"]),
        "direct_diffuse_swap": numeric(direct["top_reflected"], radiation["two_rank"]["VIS"]["diffuse"]["top_reflected"]),
        "stem_only_photosynthesis": numeric(0.0, radiation["reductions"]["stem_only"]["occupancies"][0]["results"]["absorbed_stem"]),
        "reference_wind_as_leaf_wind": numeric(aero["results"]["u_star_m_s"], aero["operands"]["u_ref_m_s"]),
        "hidden_minimum_wind": typed("VEG-E-081 nonpositive_u_star", {"u_ref_m_s": 0.0, "hidden_floor": 0.1}),
        "undocumented_wet_surface_wind": numeric(aero["results"]["semantic_winds"]["u_wet_m_s"], aero["results"]["u_star_m_s"]+0.2),
        "heat_roughness_in_momentum_log": numeric(aero["results"]["u_star_m_s"], aero["operands"]["kappa"]*aero["operands"]["u_ref_m_s"]/math.log((aero["operands"]["z_ref_m"]-aero["operands"]["displacement_m"])/0.12)),
        "nonpositive_friction_velocity": typed("VEG-E-081 nonpositive_u_star", {"u_star_m_s": 0.0}),
        "invalid_reference_height_geometry": typed("VEG-E-081 invalid_reference_height_geometry", {"z_ref_m": 9.35, "displacement_m": 8.1, "z0m_m": 1.25}),
        "crown_base_as_stem_path": numeric(accepted_q2, q2_variant(4.2, 4200.0)),
        "half_height_stem_path": numeric(accepted_q2, q2_variant(p["height_m"]/2.0, 500.0*p["height_m"])),
        "missing_gravity": numeric(accepted_q2, q2_variant(p["height_m"], 0.0)),
        "wrong_gravity_sign": numeric(accepted_q2, q2_variant(p["height_m"], -1000.0*p["height_m"])),
        "metres_as_mm_gravity": numeric(accepted_q2, q2_variant(p["height_m"], p["height_m"])),
        "stem_leaf_gravity": numeric(hyd["fluxes"]["q1_sun"], hyd["fluxes"]["q1_sun"]-p["k1_max"]*1000.0),
        "nonpositive_height": typed("VEG-E-082 nonpositive_height", {"height_m": 0.0}),
        "average_v2_root_warm_starts": typed("ambiguous_v2_layer_root_warm_starts", {"entries": [-4000.0, -5000.0], "rejected_average": -4500.0}),
        "first_v2_root_warm_start": typed("ambiguous_v2_layer_root_warm_starts", {"entries": [-4000.0, -5000.0], "rejected_first": -4000.0}),
        "publish_beta_one_emax_as_request": numeric(accepted_total, math.fsum(emax.values())),
        "hydraulics_without_energy_resolve": numeric(accepted_total, math.fsum(emax.values())),
        "posthoc_scalar_stress": numeric(accepted_total, aggregate_beta*math.fsum(emax.values())),
        "aggregate_only_transpiration_equality": typed("VEG-E-084 class_continuity_failure", {"class_residuals": [1.0e-6, -1.0e-6], "aggregate_residual": 0.0}),
        "authorization_in_potential_pass": typed("VEG-E-084 authorization_in_potential", {"authorization_kg_m2": 0.01}),
        "external_hydraulic_clamp": numeric(accepted_total, min(accepted_total, 0.5*accepted_total)),
        "rd_debited_twice": numeric(respiration["results"]["leaf_maintenance_carbon_debit_exact_once"], respiration["results"]["double_debit_poison"]),
        "nonpositive_rd_clamp": typed("VEG-E-085 nonpositive_atkin_rd25", {"raw_rd25": -0.1, "rejected_clamp": 0.0}),
        "whole_column_zero_lower_boundary": numeric(direct["top_reflected"], zero_lower_reflection),
        "direct_summed_lower_reflection": numeric(direct["top_reflected"], direct_summed_reflection),
        "root_weighted_v2_migration": typed("ambiguous_v2_layer_root_warm_starts", {"entries": [-4000.0, -5000.0], "weights": [0.62, 0.38], "rejected_weighted": -4380.0}),
        "legacy_rd_leaf_n_rate": numeric(rd["rd25_umol_co2_m2_leaf_s"], 0.002*800.0),
        "wrong_rd_temperature_response": numeric(rd["rd_sun"], rd["rd25_umol_co2_m2_leaf_s"]*arrhenius_response(respiration["operands"]["sun_leaf_temperature_k"], 46390.0)),
        "sun_shade_respiration_swap": numeric(rd["leaf_maintenance_carbon_debit_exact_once"], respiration["operands"]["tile_fraction"]*respiration["operands"]["dt_s"]*1.0e-6*respiration["operands"]["molar_mass_c_kg_mol"]*(rd["rd_shade"]*respiration["operands"]["sun_leaf_area_m2_m2_tile"]+rd["rd_sun"]*respiration["operands"]["shade_leaf_area_m2_m2_tile"])),
    }
    return poisons


def vectors() -> dict:
    families = {"radiation": radiation_vectors(),
                "aerodynamics": aerodynamic_vector(),
                "hydraulic_potential_pass": hydraulic_vectors(),
                "v2_to_v3_root_state_migration": migration_vectors(),
                "leaf_respiration": respiration_vectors(),
                "executed_ci_failures": executed_ci_failures()}
    families["failure_precedence"] = executed_failure_precedence()
    checks = {
        "all_radiation_closures": all(
            abs(component["closure_residual"]) <= 2.0e-8
            for band in families["radiation"]["two_rank"].values()
            for component in band.values()),
        "alternate_starts_converge": all(
            math.isclose(value,
                         families["hydraulic_potential_pass"]["alternate_warm_start"]
                         ["result"]["solution"][key], rel_tol=2e-9, abs_tol=2e-7)
            for key, value in families["hydraulic_potential_pass"]
            ["accepted_uncapped_stage_a"]["solution"].items()),
        "dry_and_frozen_zero": all(
            row["flux"] == 0.0 for row in families["hydraulic_potential_pass"]
            ["accepted_uncapped_stage_a"]["fluxes"]["q3"]
            if row["layer_id"] in {"soil-dry", "soil-frozen"}),
        "zero_direct_skips_directional_operands":
            families["radiation"]["zero_direct_exact_branch"] == {
                "directional_operands_evaluated": False,
                "beam_k_unclumped": None, "k_eff": None,
                "terminal_direct": 0.0},
        "accepted_gas_energy_is_resolved_not_posthoc_scalar": all(
            not math.isclose(
                families["hydraulic_potential_pass"]["accepted_uncapped_stage_a"]
                    ["fluxes"][f"gas_energy_transpiration_{class_name}"],
                families["hydraulic_potential_pass"]["accepted_uncapped_stage_a"]
                    ["solution"]["beta_hyd"]
                * families["hydraulic_potential_pass"]["internal_maximum_evaluation"]
                    ["emax"][class_name]
                * families["hydraulic_potential_pass"]["accepted_uncapped_stage_a"]
                    ["fluxes"][f"v_{class_name}"],
                rel_tol=1.0e-8, abs_tol=1.0e-12)
            for class_name in ("sun", "shade")),
        "respiration_exact_once_distinguishes_double":
            families["leaf_respiration"]["results"]
            ["leaf_maintenance_carbon_debit_exact_once"] !=
            families["leaf_respiration"]["results"]["double_debit_poison"],
        "all_release_poisons_executed": all(
            poison.get("executed") is True
            for poison in poison_manifest(families).values()),
        "all_failures_publish_no_candidate": all(
            failure.get("candidate") is None and failure.get("last_iterate") is None
            for failure in (families["executed_ci_failures"]
                + families["hydraulic_potential_pass"]["executed_canopy_energy_failures"]
                + [families["hydraulic_potential_pass"]["singular_jacobian"],
                   families["hydraulic_potential_pass"]["iteration_limit"]])),
    }
    if not all(checks.values()):
        raise AssertionError(checks)
    return {"model_version": MODEL,
            "oracle_independence": {
                "implementation_language": "Python standard library only",
                "calls_rust": False,
                "expected_values_generated_by_rust": False,
                "canonical_serialization": "recursive key sort, compact separators, UTF-8, LF"},
            "canonical_definition_sections": ["vegetation_variables",
                "vegetation_algorithm_and_equations", "vegetation_invariants",
                "vegetation_schema", "vegetation_numerics"],
            "families": families, "poisons": poison_manifest(families), "checks": checks}


def main() -> None:
    result = vectors()
    output = Path(__file__).with_name("openwepp_c3_woody_v3_vectors.json")
    payload = canonical_bytes(result)
    output.write_bytes(payload)
    print(json.dumps({"fixture": str(output), "sha256": hashlib.sha256(payload).hexdigest(),
                      "bytes": len(payload), "checks": result["checks"]}, sort_keys=True))


if __name__ == "__main__":
    main()
