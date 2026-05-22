# Continuous-Daily Kernel Port Contract (CLIM03)

Evidence mode: `Static`
Status: `implemented`

## Scope
Port continuous-daily climate runtime behavior (`itemp=1`, `ibrkpt=0`) from legacy
`stmget/disag/const/dblex/idat` lineage into typed runtime seams used by:
- `openwepp-hillslope-orchestrator`
- `openwepp-watershed-orchestrator`

## Version Policy
1. `datver=0.0` -> `iclig=0` (explicit override branch, no legacy pre-4 correction factors).
2. `datver>=4.0` -> `iclig=1`.
3. `0.0<datver<4.0` -> typed rejection (`CLIM-RUNTIME-E-001`).

## Continuous-Daily Normalization
For each non-breakpoint record:
1. Validate finite/non-negative forcing domains (`prcp`, `stmdur`, `timep`, `ip`) and finite weather fields.
2. Apply `stmdur_h = min(stmdur_h, 23.999)`.
3. Convert units:
- `prcp_m = prcp_mm * 0.001`
- `stmdur_s = stmdur_h * 3600`
4. Apply CLIGEN v4 policy scaling on `ip` when `iclig=1`:
- `ip = ip * 0.70`
5. Hard-fail when `prcp_m > 0` and `stmdur_s <= 0`.

## Event-Shape / Disaggregation Port
For wet events (`prcp_m>0 && stmdur_s>0`):
1. Normalize disaggregation controls:
- `ip = max(ip, 1.0)`
- `timep = 1.0` when `timep>1.0` or `ip==1.0`
- `timep = 0.01` when `timep<=0.0` and non-constant case
- clamp `ip<=60`, `timep<=0.99` for `dblex` branch
2. Start with `ninten=11`; regenerate with decreasing `ninten` until minimum interval spacing is `>=300 s` (legacy behavior), with 2-step constant fallback.
3. Branch selection:
- `const` when `timep>=1 && ip<=1`
- `dblex` otherwise (including `eqroot` solve path)
4. Dimensionalize:
- `timem(i) = timedl(i) * stmdur_s`
- `intsty(i) = intdl(i) * prcp_m / stmdur_s`
5. Enforce typed guards:
- strict monotone `timem`
- disaggregation root-solve domain/convergence checks
- precipitation closure residual tolerance (`1e-9 m`)

Dry events (`prcp_m<=0 || stmdur_s<=0`) publish zeroed event-shape (`ninten=0`, empty `timem/intsty`, `avrint=0`, `mxint=0`).

## Runtime Symbols Published
No-breakpoint runtime surfaces now include:
- existing: `prcp`, `stmdur`, `timep`, `ip`, weather fields
- added CLIM03: `ninten`, `avrint`, `mxint`, `timem_####`, `intsty_####`

Watershed assignments publish per-hillslope equivalents with `hs{ID}_` prefix.

## Typed Guard Surfaces
- Hillslope seam: `CLIM-RUNTIME-E-012..015` added for disaggregation runtime failures.
- Watershed seam: `CLIM-RUNTIME-E-013..016` added with `hillslope_id` context.

## Implementation Anchors
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
- `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`
- `tests/integration/parser_runtime_seam_integration.rs`
