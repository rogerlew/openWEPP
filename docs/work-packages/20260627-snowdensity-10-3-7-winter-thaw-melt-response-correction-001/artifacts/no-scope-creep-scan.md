# No-Scope-Creep Scan

Evidence mode: Static.

- Default activation changed: false.
- Parser/runfile/user CLI selector added: false.
- Package-bound diagnostic environment selector added for coupled WAT evidence:
  true (`OPENWEPP_SNOWDENSITY1037_MELT_MODEL`); absent/empty preserves
  `legacy_coe`, unknown values fail closed, and it is not exposed through parser,
  runfile, or user CLI activation.
- Public WAT/HBP/PASS output schema changed: false.
- Fixture input edits: none.
- Melt coefficients changed: none.
- Radiation source/scalar changed: none.
- Canopy handling changed: none.
- Phase-partition behavior changed: none.
- Density constants changed: none.
- Frost physics changed: none.
- Rain heat changed: none.
- Sub-canopy longwave changed: none.
- Qwet/frzftp changed: none.
- Compatibility-runtime deletion/change: none.

The only production-runtime branch delta is guarded by explicit typed opt-in
`SnowMeltModel::CoeWinterThawStateLossV1`.
