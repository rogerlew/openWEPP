# CQR17 CRAP After

Status: closed.

Ran: after CRAP command:

```text
cargo crap --workspace \
  --lcov docs/work-packages/20260615-cqr17-hydrology-erod19-complexity-001/artifacts/lcov_after.info \
  --min 0 --format json \
  --output docs/work-packages/20260615-cqr17-hydrology-erod19-complexity-001/artifacts/crap_after.json
```

Result: exit code `0`; JSON saved to `crap_after.json`.

Target and extracted helper rows:

```text
Wb11HydrologyKernel::erod19_xcrit_classification    line 64   CC 2.0   Cov 100.0   CRAP 2.0
Wb11HydrologyKernel::erod19_xcrit_unclamped         line 92   CC 6.0   Cov 100.0   CRAP 6.0
Wb11HydrologyKernel::erod19_linear_xcrit_classification line 112 CC 3.0 Cov 95.23809523809523 CRAP 3.000971817298348
Wb11HydrologyKernel::erod19_increasing_linear_shear_class line 141 CC 3.0 Cov 85.71428571428571 CRAP 3.0262390670553936
Wb11HydrologyKernel::erod19_decreasing_linear_shear_class line 156 CC 3.0 Cov 85.71428571428571 CRAP 3.0262390670553936
Wb11HydrologyKernel::erod19_rising_xcrit_classification line 171 CC 8.0 Cov 94.28571428571428 CRAP 8.011941690962098
Wb11HydrologyKernel::erod19_curved_xcrit_classification line 209 CC 3.0 Cov 76.92307692307693 CRAP 3.1106053709604002
Wb11HydrologyKernel::erod19_curved_root_xcrit_classification line 238 CC 12.0 Cov 83.33333333333334 CRAP 12.666666666666664
Wb11HydrologyKernel::erod19_segment_root            line 288  CC 3.0   Cov 100.0   CRAP 3.0
Erod19XcritResult::clamped_tuple                    line 22   CC 1.0   Cov 100.0   CRAP 1.0
```

Closure: target and all extracted helpers are CRAP `<= 30`.

Warning: live after metrics still show out-of-scope pre-existing high CRAP rows
in the same file:

```text
Wb11HydrologyKernel::run_erod19_route_segment_migration CRAP 351.9234211799049
Wb11HydrologyKernel::erod19_depend CRAP 87.98408081839372
```

Disposition: those functions are outside the CQR17 target row and were not
modified except for line relocation caused by the target decomposition above.
