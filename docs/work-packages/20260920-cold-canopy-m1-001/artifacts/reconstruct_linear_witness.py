"""Independently inspect a recorded linear system; never solve canopy physics."""
import hashlib
import json
import math
from pathlib import Path
import sys

trace = Path(sys.argv[1])
output = Path(sys.argv[2])
prefix = "m1-linear-witness "
witnesses = [
    json.loads(line.split(prefix, 1)[1])
    for line in trace.read_text().splitlines()
    if prefix in line
]
assert witnesses, "No actual linear witness was recorded"
assert all(value == witnesses[0] for value in witnesses), "Conflicting witnesses"
witness = witnesses[0]
matrix, residuals, delta = (
    witness[key] for key in ("normalized_jacobian", "normalized_residuals", "delta")
)
assert len(matrix) == len(residuals) == len(delta) == 21
assert all(len(row) == 21 for row in matrix)
assert all(math.isfinite(x) for row in matrix for x in row)
assert all(math.isfinite(x) for x in residuals + delta)
remainders = [math.fsum([r, *(a * x for a, x in zip(row, delta))])
              for row, r in zip(matrix, residuals)]
row_denominators = [math.fsum([abs(r), *(abs(a * x) for a, x in zip(row, delta))])
                    for row, r in zip(matrix, residuals)]
assert all(d > 0.0 or e == 0.0 for d, e in zip(row_denominators, remainders))
matrix_norm = max(math.fsum(abs(x) for x in row) for row in matrix)
residual_norm = max(map(abs, residuals))
delta_norm = max(map(abs, delta))
remainder_norm = max(map(abs, remainders))
denominator = matrix_norm * delta_norm + residual_norm
assert denominator > 0.0
result = {
    "trace": str(trace.resolve()),
    "trace_sha256": hashlib.sha256(trace.read_bytes()).hexdigest(),
    "script_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
    "identical_witness_occurrences": len(witnesses),
    "method": "Python math.fsum of actual binary64 products J_ij*delta_j and r_i",
    "linear_remainder_by_row": remainders,
    "linear_remainder_inf_norm": remainder_norm,
    "matrix_inf_norm": matrix_norm,
    "residual_inf_norm": residual_norm,
    "delta_inf_norm": delta_norm,
    "normwise_backward_error": remainder_norm / denominator,
    "componentwise_backward_error": max(
        abs(e) / d if d > 0.0 else 0.0
        for e, d in zip(remainders, row_denominators)
    ),
    "disposition": "Diagnostic only; does not establish Jacobian authority, nonlinear convergence, or physical acceptance",
}
output.write_text(json.dumps(result, indent=2, allow_nan=False) + "\n")
print(json.dumps(result, indent=2, allow_nan=False))
