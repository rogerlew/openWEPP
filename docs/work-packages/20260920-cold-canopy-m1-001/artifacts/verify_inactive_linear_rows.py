"""Frozen-original60 control for the retained direct inactive-leaf rows."""
import hashlib
import json
from pathlib import Path
import sys

trace = Path(sys.argv[1])
prefix = "m1-linear-witness "
items = [json.loads(line.split(prefix, 1)[1])
         for line in trace.read_text().splitlines() if prefix in line]
assert items and all(item == items[0] for item in items)
item = items[0]
matrix = item["normalized_jacobian"]
linear_residuals = item["normalized_residuals"]
convergence_residuals = item["convergence_residuals"]
coordinates = item["coordinates"]
rows = []
for row in (0, 6):
    raw = coordinates[row] - max(coordinates[12], 273.15)
    assert matrix[row] == [1.0 if column == row else 0.0 for column in range(21)]
    assert linear_residuals[row] == raw
    assert convergence_residuals[row] == raw / (1.0e-6 + 1.0e-10)
    rows.append({"row": row, "raw_anchor": raw,
                 "linear_rhs": -linear_residuals[row],
                 "convergence_residual": convergence_residuals[row]})
result = {"trace": str(trace.resolve()),
          "trace_sha256": hashlib.sha256(trace.read_bytes()).hexdigest(),
          "script_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
          "checked_rows": rows,
          "result": "PASS: exact unit-scaled direct rows; convergence normalization retained",
          "scope": "Frozen original60 inactive sun rows only; not nonlinear or physical acceptance"}
Path(sys.argv[2]).write_text(json.dumps(result, indent=2) + "\n")
print(json.dumps(result, indent=2))
