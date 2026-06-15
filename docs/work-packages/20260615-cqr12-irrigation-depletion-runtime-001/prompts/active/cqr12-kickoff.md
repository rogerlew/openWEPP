# CQR12 Kickoff

Execute CQR12 for
`crates/openwepp-hillslope-orchestrator/src/runtime_inputs/04_snow_frost_irrigation.rs`.

Follow `package.md` exactly. Preserve public API, `irrigation.depletion.*`
symbols, typed guard variants, field names, allowed strings, units,
parser-compatibility, period ordering, and kernel-facing projection semantics.

Use fresh LCOV and CRAP before selecting the live target. Add focused
characterization before production refactor if needed. Close only when the live
target and all extracted helpers are CRAP `<= 30` and all required gates are
run and recorded.
