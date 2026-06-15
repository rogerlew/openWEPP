# CQR12 Disposition

Status: complete-with-warnings.

Static: CQR12 package objective was met for the scoped target.

Closed:

- `seed_hillslope_runtime_surface_from_irrigation_depletion` CRAP reduced from
  `1122.0` to `2.0`.
- Every newly extracted depletion helper is CRAP `<= 9.015780389578367`.
- Focused CQR12 irrigation depletion characterization passed before and after
  production refactor.
- Public API, typed error variants, stable field names, `allowed` strings,
  depletion symbols, units, parser compatibility, period iteration, and
  kernel-facing projection meanings were preserved.
- Required Rust closure gates passed.

Warnings accepted:

- target-file coverage remains below the ADR-0021 science-tier threshold;
- pre-existing out-of-scope frost `#[allow(clippy::too_many_lines)]` remains at
  line `865`.

No review finding remains undispositioned.

Package status: complete-with-warnings.
