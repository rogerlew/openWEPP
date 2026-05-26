# SIMIMPL28 Kernel Profile Compliance Checklist

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
- Kernel-affecting package: yes.
- Contract-first sequencing: satisfied (contracts -> tests -> gate -> code).
- Canonical authority maintained in `SC-*` contracts; package artifacts are
  evidence only.
- Legacy migration provenance posture retained against
  `/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- Typed guard posture preserved; missing/out-of-range winter context symbols
  fail with typed runtime errors.
- No silent domain clamping/defaulting introduced for required active winter
  synthesis inputs.

## Ran
- `rg -n "MissingRuntimeContextSymbol|RuntimeContextSymbolOutOfRange|InvalidCalendarDate" crates/openwepp-climate-runtime-adapter/src/lib.rs crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`
- `rg -n "SIMIMPL28 Hourly Winter Forcing Synthesis Addendum|SIMIMPL28 Forcing-Emission Scope Clarification" docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
