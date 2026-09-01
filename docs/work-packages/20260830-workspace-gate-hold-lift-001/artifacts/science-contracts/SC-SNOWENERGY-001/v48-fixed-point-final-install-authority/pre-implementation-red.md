# V48 fixed-point final-install authority pre-implementation red

Status: `EXPECTED RED CONFIRMED`

Evidence mode: `Static + Ran`

## Trigger and exact seam

Retained r122 at `/tmp/wghl_001d_v47_64m_r122.log`, SHA-256
`20f5b118b43f69a35ce3e0ed03576bd916b3b4a9cb579692727f0438fb5de2bc`,
clears the direct `60 s` receipt path and then fails at composed
`1800..1980 s` with `V2 soil atomic split transaction authority required`.

The real ordinary fixed-point completion in `open_snow.rs` has an authenticated
soil preview after exact receipt replay but calls `finalize_v11_imported_segment`
without a precomputed ending, candidate, or continuation. That wrapper supplies
no continuation authority. `owner_finalization.rs` reconstructs the lawful
prepared target 43 with exact predecessor/source 42 and then calls the strict
generic `install_soil_thermal_accepted_v2`, which correctly refuses the split
because its explicit authority is `None`. V47's continuation tests never drove
this real `None` branch.

## Expected-red execution

Ran:

```text
nix develop -c cargo nextest run \
  --test snow_terminal_enthalpy_event_numerics_contract -E 'test(/v48_/)'
```

Result: Nextest run `b54139fe-ee30-4042-bcfa-e0936f8f0004`, `1 passed; 1
failed`. The contract-first authority assertion is green; the production/source
obligation reports the absent authenticated-prepared-beginning authority and
install methods, absent real-finalizer calls, and five absent behavior/source
vectors.

No V48 production source had been edited when this evidence was captured.
