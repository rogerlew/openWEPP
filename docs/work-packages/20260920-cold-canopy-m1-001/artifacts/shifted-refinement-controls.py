#!/usr/bin/env python3
"""Non-result-bearing analytic and primitive controls for shifted refinement."""
import importlib.util
import json
import math
import sys
import time
from fractions import Fraction
from pathlib import Path

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("shifted_author", HERE / "shifted-refinement-author.py")
assert spec and spec.loader
m = importlib.util.module_from_spec(spec); spec.loader.exec_module(m)

# Nonsymmetric V and nonidentity traversal distinguish orientation and column joins.
v = [[0.6, -0.8], [0.8, 0.6]]; sigma = [2.0, 3.0]; h = [10.0, -20.0]; lam = 5.0
delta = m.shifted_inverse(v, sigma, [1, 0], h, lam)
assert all(abs(a-b) < 1e-15 for a,b in zip(delta, [-10.0 / 21.0, 110.0 / 63.0])), delta
assert abs(delta[0] - 0.6) > 0.1

# A 21-dimensional diagonal fixture uses the same author functions and proves that
# an active join is unchanged while BVLS admission still rejects a free defect.
A = [[0.0 for _ in range(21)] for _ in range(21)]
for i in range(21): A[i][i] = 1.0
f = [0.0] * 21; p = [0.0] * 21; p[11] = -2.0
lo = [-10.0] * 21; hi = [10.0] * 21; lo[11] = hi[11] = -2.0
assessment = m.bvls02(A, f, p, 1.0, lo, hi, [False]*21, [False]*21, 10.0)
assert assessment["classes"][11] == "Fixed"
assert assessment["classes"][0] == "FreePass"
joined = m.update_free(p, [0], [0.5])
assert m.bits(joined[11]) == m.bits(p[11])
# Both ordinary and accepted Dot2 defects must retain lambda*q, not merely its
# denominator contribution to the inverse.
ordinary_full = m.ordinary_defect(A, f, p, 1.0)
dot2 = m.load_dot2()
_, dot2_full, _, traces = m.dot2_defect(A, f, p, 1.0, list(range(21)), dot2)
for trace in traces: dot2.verify_identities(trace)
assert ordinary_full[11] == -4.0 and dot2_full[11] == -4.0
p_bad = p[:]; p_bad[0] = 1.0
rejected = m.bvls02(A, f, p_bad, 1.0, lo, hi, [False]*21, [False]*21, 10.0)
assert rejected["classes"][0] == "FreeRefuse"
p_box = p[:]; p_box[0] = 11.0
box_rejected = m.bvls02(A, f, p_box, 1.0, lo, hi, [False]*21, [False]*21, 100.0)
assert not box_rejected["box_feasible"]
p_radius = p[:]; p_radius[0] = 11.0
wide_lo = [-20.0]*21; wide_hi = [20.0]*21; wide_lo[11] = wide_hi[11] = -2.0
radius_rejected = m.bvls02(A, f, p_radius, 1.0, wide_lo, wide_hi, [False]*21, [False]*21, 10.0)
assert not radius_rejected["radius_feasible_exact"]
assert 21*(2*21+1) + 20*(2*21+2) + 2*(4*20*20+4*20) + 2*20 + (21+20)*(25*(21+1)-7) == 27446

# Exercise actual guards, exact zero skips and cancellation, not canned results.
def refuses(fn, error):
    try:
        fn()
    except error:
        return
    raise AssertionError("expected arithmetic-domain refusal")

for value in (math.inf, math.nan, sys.float_info.min / 2):
    refuses(lambda value=value: dot2.dot2([value], [1.0]), dot2.PrototypeLimitation)
refuses(lambda: dot2.dot2([1e308], [1.0]), dot2.PrototypeLimitation)
refuses(lambda: dot2.dot2([2.0**-600], [2.0**-600]), dot2.PrototypeLimitation)
refuses(lambda: m.upmul(2.0**-600, 2.0**-600, "control"), m.PrototypeLimitation)
assert m.bits(m.mul(-0.0, 1.0, "signed_zero")) == "8000000000000000"
assert m.upmul(0.0, 1.0, "zero") == 0.0
assert m.upadd(0.0, 0.0, "zero") == 0.0
cancel = dot2.dot2([1e16, 1.0, -1e16], [1.0, 1.0, 1.0])
dot2.verify_identities(cancel)
assert cancel.value == 1.0

# Real category counts on synthetic m=20 values, no captured input loaded.
free20 = list(range(20))
v20 = [[float(i == j) for j in range(20)] for i in range(20)]
start = m.snapshot()
m.shifted_inverse(v20, [1.0]*20, list(reversed(range(20))), [1.0]*20, 1.0)
assert m.count_delta(start)["scalar"] == 1680
start = m.snapshot()
m.ordinary_defect(A, f, p, 1.0, free20)
assert m.count_delta(start)["scalar"] == 1783
_, _, cost, traces = m.dot2_defect(A, f, p, 1.0, free20, dot2)
assert cost["primitive_arithmetic"] == 22263
for trace in traces:
    dot2.verify_identities(trace)

# A small excess is rejected even when its absolute gap is below tolerance.
p_excess = [0.0]*21
p_excess[0] = math.nextafter(1.0, math.inf)
wlo, whi = [-100.0]*21, [100.0]*21
assessment = m.bvls02(A, f, p_excess, 1.0, wlo, whi, [False]*21, [False]*21, 1.0)
disposition = m.norm_and_disposition(assessment,p_excess,p_excess,list(range(21)),[],1.0,1.0,2.0**-40,1.0)
assert not disposition["full_feasible_ordered"] and not disposition["reduced_gap_within_tolerance"]
assert assessment["work"]["kkt"]["scalar"] == 1827
assert assessment["work"]["enclosure"]["scalar"] > 0

# Different full and remaining radii exercise the active contribution.
p_join = [0.0]*21
p_join[0], p_join[11] = 0.5, 2.0
assess_join = m.bvls02(A,f,p_join,1.0,wlo,whi,[False]*21,[False]*21,3.0)
d_join = m.norm_and_disposition(assess_join,p_join,p_join,[i for i in range(21) if i != 11],[11],3.0,0.75,0.3,1.0)
assert d_join["full_feasible_ordered"] and d_join["reduced_feasible_ordered"]
assert d_join["reduced_radius_gap_ordered"] == 0.25
assert d_join["full_norm_squared_exact"] != d_join["reduced_norm_squared_exact"]

# Independent analytic exact reference: q=-3/(1+lambda), radius=1,
# root lambda=2, recorded multiplier=8 and narrow endpoint=7.
reference = m.reference([[1.0]], [3.0], [0.0], [0], [], 8.0, 1.0, time.monotonic()+10, runtime_low=7.0)
assert reference["lower_lambda_fraction"] == reference["upper_lambda_fraction"] == "2"
assert reference["lambda_error_interval_against_recorded"] == ["6", "6"]
assert not reference["recorded_runtime_bracket_contains_reference"]
assert reference["all_linear_residuals_exact_zero"]
assert reference["exact_reference_operations"]["face_matrix_builds"] == 1
assert reference["exact_reference_operations"]["solves"] == 5
exact_upper = m.reference([[1.0]], [3.0], [0.0], [0], [], 2.0, 1.0, time.monotonic()+10, runtime_low=1.0)
assert exact_upper["bisections_completed"] == 0 and exact_upper["complete_80_or_exact_zero"]
no_positive = m.reference([[1.0]], [0.5], [0.0], [0], [], 8.0, 1.0, time.monotonic()+10, runtime_low=7.0)
assert not no_positive["bracket_established"]

out = {"evidence_class": "Ran: synthetic analytic/primitive controls only; capture is not loaded and target candidate/reference not evaluated",
       "controls": {name: "pass" for name in [
         "nonzero_lambda_and_V_orientation", "nonzero_lambda_full_ordinary_and_dot2_defect",
         "inactive_active_join_preservation", "unchanged_bvls02_admission_and_rejection",
         "scaled_box_rejection", "original_radius_rejection", "small_outside_radius_rejection",
         "distinct_full_reduced_radius", "dot2_guards_cancellation_signed_zero",
         "actual_category_counts", "exact_reference_root_sign_error_and_cache", "exact_upper_root", "no_positive_root"]},
       "candidate_result_bearing_execution": "not run"}
(HERE / "shifted-refinement-controls.json").write_text(json.dumps(out, indent=2, sort_keys=True) + "\n")
