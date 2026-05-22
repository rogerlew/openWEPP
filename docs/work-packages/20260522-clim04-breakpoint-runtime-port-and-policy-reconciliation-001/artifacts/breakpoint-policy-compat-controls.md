# Breakpoint Policy Compatibility Controls (CLIM04)

Evidence mode: `Static`
Status: `implemented`

## Strict Defaults
1. Breakpoint cardinality: `nbrkpt <= 1500`.
2. Breakpoint time policy: strict `dtime > 0` across all intervals (duplicate/decreasing `timem` rejected).
3. Negative cumulative-depth increments (`drain < 0`) remain hard-fail.
4. No silent fallback/default masking on malformed breakpoint forcing.

## Explicit Compatibility Controls
1. `allow_breakpoint_cardinality_override`
- surface: parser compatibility mode (`CompatibilityOptions`)
- default: `false`
- effect: allows parsing breakpoint rows beyond `1500` for controlled investigations.

2. `allow_legacy_zero_drain_non_positive_dtime`
- surface: parser compatibility mode (`CompatibilityOptions`)
- default: `false`
- effect: explicit opt-in to legacy zero-drain non-positive-time acceptance behavior during parsing.

## Runtime Guard Posture
- Runtime seam adapters remain strict by default and reject non-monotone breakpoint elapsed times.
- Compatibility controls do not weaken strict defaults.

## Rationale
- CLIM01 ratified strict policy closure for breakpoint `dtime` and `1500` cardinality alignment.
- Explicit compatibility controls preserve investigatory access to legacy edge behavior without changing default correctness policy.
