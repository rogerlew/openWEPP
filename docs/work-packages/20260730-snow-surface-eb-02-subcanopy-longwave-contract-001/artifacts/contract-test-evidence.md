# Contract Test Evidence

Status: `PASS`.

Evidence class: Ran.

Command:

```text
.venv/bin/python docs/work-packages/20260730-snow-surface-eb-02-subcanopy-longwave-contract-001/tools/execute.py
```

Result: `PASS`; regenerated analytical vectors and two accessible SVG figures.

The CSV compares evaluator results to immutable expected numeric values with
the declared tolerances. It checks sky view, cloud endpoints and reachable
clamps, atmospheric components, complementary longwave, net-longwave sign,
and invalid/non-finite cover, temperature, vapor, radiation, cloud, and flux
guards. The polar-night unavailable branch is executed. The only `HOLD` row is
the deliberately absent EB-03 thermal provider.

The executor is package-local evidence code and is not imported by production
crates.
