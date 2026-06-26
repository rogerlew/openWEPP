# SNOWDENSITY-05C Kickoff Prompt

Read the root, work-package, science-contract, tests, and crates `AGENTS.md`
files. Read `docs/planning/snow-frost-fidelity-strategy.md` sections 2, 4, 5,
7, and 10; `SC-SNOWFREEZE-001`; the SNOWDENSITY-05A/05B handoffs; and the
Brock et al. (2000) albedo authority.

Scaffold and execute SNOWDENSITY-05C as a contract/albedo-state-core package.
Ratify the opt-in `brock2000_temperature_age_v1` albedo state before code.
Implement a typed, standalone Rust state update core with bounds, fresh-snow
reset, accumulated positive-temperature age, model id, and fail-closed missing
state behavior for future `coe_shortwave_albedo_v1`. Preserve `legacy_coe`
default behavior and do not wire routed melt, parser surfaces, output schemas,
or activation defaults.
