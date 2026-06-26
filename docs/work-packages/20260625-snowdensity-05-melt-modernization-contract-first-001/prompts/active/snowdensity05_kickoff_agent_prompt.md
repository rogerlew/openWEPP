# SNOWDENSITY-05A Closed Prompt

Status: complete.

This package is closed as SNOWDENSITY-05A, the contract/sign reconciliation
slice. Do not resume this prompt for runtime implementation work.

If continuing melt modernization, scaffold the next package from
`artifacts/worker-handoff.md`: `SNOWDENSITY-05B Shortwave Source Binding`.

Carried constraints:

- Keep `legacy_coe` default and rollback.
- Do not implement `coe_shortwave_albedo_v1` production melt until 05B and 05C
  are complete and 05D is authorized.
- Preserve the signed `melt_bmelt_in` convention.
- Do not tune or rescale shared radiation forcing.
- Do not promote `dense_slow_melt_v1` or any degree-day snowbench variant.
