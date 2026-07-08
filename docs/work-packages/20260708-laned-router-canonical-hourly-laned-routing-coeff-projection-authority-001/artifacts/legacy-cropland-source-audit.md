# Legacy Cropland Source Audit

Status: complete.
Evidence class: Static.

Baseline source pin:
`/workdir/wepp-forest_260430_baseline` at
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.

## Decision

No legacy cropland source audited here is direct or bounded projection authority
for the complete five-value Lane D static coefficient set. All implicit
legacy-field routes are rejected for production/default activation.

## Candidate Classification

| Candidate | Potential target | Classification | Evidence | Disposition |
|---|---|---|---|---|
| Native `routing_coefficients` block | all five static operands | direct coefficient authority | Parser reads exactly five values at `management.rs:2038-2064`; runtime validates the five `route_*` fields at `00_builders_and_authority.rs:876-1010`. | Accepted; remains authoritative. |
| Authorized explicit producer table | all five static operands | direct producer authority if it emits all five fields with provenance | Management-lanuse authority contract allows explicit producer publication and keeps `LANUSE-AUTH-3` no-inference binding. | Accepted only as explicit route fields, not hidden bridge math. |
| Random roughness `rrc`, `rrinit`, `rrough` | `D_r_m`, `lambda`, roughness context | rejected alias | `frcfac.for:164-192` uses `rrinit`/`rrc` to compute cropland interrill roughness friction; rangeland branch uses `rrough` at `frcfac.for:251-292`. `bigout.for:187` and `watbal_hourly.for:1112` publish `rrc` diagnostics. | Context/diagnostic only; not element height or concentration authority. |
| Row/rill geometry `width`, `rspace` | `lambda` or geometry-derived concentration | rejected alias | `frcfac.for:311-327` uses `width/rspace` only to weight rill vs interrill friction into `frcteq`. | Not a roughness-element concentration bridge. |
| Cropland interrill/rill friction factors `inrfto`, `frcsol`, `frctrl`, `frcteq` | `k_o` or aggregate friction comparator | diagnostic/context only | `frcfac.for:151-238` computes cropland aggregate interrill/rill friction from roughness, cover, live plant, and bare-soil constants; `frcfac.for:320-327` combines them. | Cannot be inverted into five independent Lane D operands without surrogate assumptions. |
| Cover/residue/canopy cover terms `inrcov`, `rilcov`, `rescov`, `cancov` | `lambda`, vegetation drag context | rejected alias | Cropland cover contributes to aggregate friction at `frcfac.for:194-201` and `227-236`; rangeland cover/residue/canopy terms appear in `frcfac.for:261-305`. | Cover fractions are not Lane D `lambda` or vegetation `C_d` authority. |
| Live plant `canhgt`, `hmax`, `flivmx` | dynamic vegetation context | context only | `frcfac.for:207-214` computes `frlive` from canopy height over maximum height times `flivmx`. | Dynamic `h_c`/`LAI` are already separately authorized by `SC-OFEROUTE-001`; `flivmx` is not vegetation `C_d`. |
| Erosion delivery ratio from `rrc` | sediment comparator context | rejected alias | `param.for:412-459` computes cropland interrill delivery ratio from `rrc` and particle classes, then erosion parameters continue at `param.for:461-550`. | Erosion delivery math is not routing-coefficient authority. |
| Hourly water-balance cropland growth context | dynamic plant state context | context only | `watbal_hourly.for:936-965` updates cropland/rangeland plant state; no static Lane D route coefficients are emitted. | Does not supply static route operands. |

## Rationale

The legacy surfaces are aggregate model terms or diagnostics. A production Lane
D coefficient bridge would need to produce all five static operands
independently, with units, domains, provenance, and a bounded fidelity argument.
The audited sources do not contain that mapping.
