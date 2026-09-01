# V37 derived-thickness closure pre-implementation red

Evidence state: `EXPECTED RED — RETAINED`

## Retained r93 finalization audit

Log: `/tmp/wghl_001d_v36_64m_r93_finalization_audit.log`

SHA-256: `c4ddfef9dc52bdd085ff43d97f1179406486795696425c9aa949097fd756b0a5`

Result: `FAIL` on exact `1800..1860 s` support. The v36 coupled root closed
LSE and boundary tolerances and retained bit-exact density, but authentic
finalization changed lane-1 `layer.thickness_m` from bits
`4569208177783694401` to `4569208162027237604`, delta
`6.833273876e-9 m`. The existing water-mass residual can pass at
`1e-6 kg m^-2`; at density near `100 kg m^-3`, its canonical `I/rho` depth
image can therefore exceed the unchanged `1e-9 m` depth bound. This is failure
evidence, not canonical qualification.

## Contract authority gate

Ran:

```text
nix develop -c cargo nextest run --test snow_terminal_enthalpy_event_numerics_contract v37_contract_binds_derived_thickness_root_closure --no-capture
```

Result: `PASS`, run `0538fbb8-8b91-423e-80ff-1920920e3b96`, one passed,
seventeen filtered. Assertions bind derived physical `R_z`, the unchanged
`depth_abs_m`, no new z coordinate or solved equation, same-map budget
charging, v35 receipt replay, authentic finalization, and shortcut refusals.

## Source-bound implementation gate

Ran:

```text
nix develop -c cargo nextest run --test snow_terminal_enthalpy_event_numerics_contract v37_derived_thickness_production_seams_are_required --no-capture
```

Result: `EXPECTED FAIL`, run `d9fb272d-23ea-42d3-a673-2c0db74a81f5`, zero
passed, one failed, seventeen filtered. Production lacked four required seams:

- `CoveredDerivedThicknessClosureV1`;
- `covered_derived_thickness_closure_evaluate_v1`;
- `r_z_m`;
- `derived_constraints_scaled_merit`.

It also lacked five required behavior vectors covering low-density water-error
amplification, same-charged-image derivation, combined residual/depth root
admission, v35 replay plus finalization, and independent-z/omission/repair/
bypass refusal.

Snapshot SHA-256 values:

- contract: `f1e174dd3ac47682b71c8c306ddaa206cdd2e686e079e404fec1d5c9963ea3e1`
- package: `99ecdb3ef0f033871833e0849890804de36bb2e82ce279747aee0592162e2af4`
- contract test: `f05b754829af99ba539b5a83a28e2fb4e56458aad4be40722bbdfe97ffe035f8`
