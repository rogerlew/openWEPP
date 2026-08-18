#!/usr/bin/env python3
"""Independent V10 exact-zero and positive-low-light Ci authority."""

import json
import math

R_GAS = 8.31446261815324


def smaller_root(a, b, c):
    if c == 0.0:
        return min(0.0, -b / a)
    root = math.sqrt(b * b - 4.0 * a * c)
    q = -0.5 * (b + math.copysign(root, b))
    return min(q / a, c / q)


def fvcb(case, ci):
    ipsii = 0.5 * case["electron_quantum_yield"] * case["par_photon_umol_per_j"] * case["par_abs"]
    j = smaller_root(case["electron_curvature"], -(ipsii + case["jmax"]), ipsii * case["jmax"]) if ipsii > 0.0 else 0.0
    if ci < case["gamma"]:
        ac = aj = 0.0
    else:
        ac = case["vcmax"] * (ci-case["gamma"]) / (ci + case["kc"] * (1.0 + case["oi"] / case["ko"]))
        aj = j * (ci-case["gamma"]) / (4.0*ci + 8.0*case["gamma"])
    ap = 3.0 * case["tp"]
    ai = smaller_root(0.98, -(ac+aj), ac*aj)
    ag = smaller_root(0.95, -(ai+ap), ai*ap)
    return {"ac":ac,"aj":aj,"ap":ap,"ag":ag,"an":ag-case["rd"],"j":j}


def residual(case, ci):
    photo = fvcb(case, ci)
    cs = case["ca"] - 1.4*case["rb"]*R_GAS*case["temperature"]*photo["an"]*1.0e-6
    gs = case["g0"] if photo["an"] <= 0.0 else case["g0"] + 1.6*(1.0+case["g1"]/math.sqrt(case["vpd"]))*photo["an"]/(cs/case["pressure"])
    rs = 1.0 / (gs*1.0e-6*R_GAS*case["temperature"]/case["pressure"])
    predicted = case["ca"] - (1.4*case["rb"]+1.6*rs)*R_GAS*case["temperature"]*photo["an"]*1.0e-6
    return ci-predicted, {**photo,"cs":cs,"gs":gs,"rs":rs}


def brent(case, low, high):
    a, b = low, high
    fa, _ = residual(case, a)
    fb, state = residual(case, b)
    if fa == 0.0:
        return a, residual(case, a)[1], 2, [a,b]
    if fb == 0.0:
        return b, state, 2, [a,b]
    if fa*fb > 0.0:
        raise ArithmeticError("unbracketed")
    c, fc, d, mflag = a, fa, b-a, True
    for evaluation in range(3, 65):
        if fa != fc and fb != fc:
            s = (a*fb*fc/((fa-fb)*(fa-fc)) + b*fa*fc/((fb-fa)*(fb-fc)) + c*fa*fb/((fc-fa)*(fc-fb)))
        else:
            s = b-fb*(b-a)/(fb-fa)
        left, right = min((3.0*a+b)/4.0,b), max((3.0*a+b)/4.0,b)
        conditions = (not left < s < right, mflag and abs(s-b) >= abs(b-c)/2.0, not mflag and abs(s-b) >= abs(c-d)/2.0, mflag and abs(b-c) < 1.0e-6, not mflag and abs(c-d) < 1.0e-6)
        if any(conditions):
            s, mflag = 0.5*(a+b), True
        else:
            mflag = False
        fs, state = residual(case, s)
        d, c, fc = c, b, fb
        if fa*fs < 0.0:
            b, fb = s, fs
        else:
            a, fa = s, fs
        if abs(fa) < abs(fb):
            a, b, fa, fb = b, a, fb, fa
        scale = max(abs(a),abs(b),1.0)
        if abs(fb) <= 1.0e-8 or abs(b-a) <= 1.0e-6+1.0e-10*scale:
            return b, residual(case,b)[1], evaluation, [min(a,b),max(a,b)]
    raise ArithmeticError("iteration_limit")


BASE = {"temperature":296.0,"pressure":101325.0,"ca":42.0,"rb":50.0,"g0":100.0,"g1":4.0,"vpd":1.2,"gamma":5.0,"oi":20265.0,"kc":40.0,"ko":30000.0,"vcmax":60.0,"jmax":100.0,"tp":6.0,"rd":1.2,"electron_quantum_yield":0.85,"par_photon_umol_per_j":4.6,"electron_curvature":0.7}


def solve(par_abs):
    case = dict(BASE, par_abs=par_abs)
    rs = 1.0/(case["g0"]*1.0e-6*R_GAS*case["temperature"]/case["pressure"])
    ci_dark = case["ca"] + (1.4*case["rb"]+1.6*rs)*R_GAS*case["temperature"]*case["rd"]*1.0e-6
    if par_abs == 0.0:
        _, state = residual(case,ci_dark)
        return {"branch":"exact_zero_analytic","ci":ci_dark,"iterations":0,"bracket":[ci_dark,ci_dark],"state":state}
    fg, _ = residual(case,case["gamma"])
    fc, _ = residual(case,case["ca"])
    low, high, branch = (case["gamma"],case["ca"],"historical_daylight") if fg*fc <= 0.0 else (case["ca"],ci_dark,"positive_low_light")
    ci, state, iterations, bracket = brent(case,low,high)
    return {"branch":branch,"ci":ci,"iterations":iterations,"bracket":bracket,"state":state}


payload = {"authority":"SC-VEGETATION-001@14","cases":[{"par_abs":x,"result":solve(x)} for x in (0.0,-0.0,0.1,1.0,50.0)]}
print(json.dumps(payload,sort_keys=True,separators=(",",":")))
