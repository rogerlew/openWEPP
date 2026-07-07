# Bridge Authority Audit

Status: EXECUTED-HOLD-ROUTE-COEFFICIENT-BRIDGE-AUTHORITY. Evidence mode: Static + Ran.

## Authority Standard

Static:

- `LANUSE-AUTH-3` says new-physics operands must not be inferred from legacy
  cropland fields unless a separate ratified bridge contract defines the
  mapping.
- `SC-INFILE-MANAGEMENT-001` binds the current source-authorized static route
  coefficient surface to native `ow-lanuse-1` forest/cropland
  `routing_coefficients` records only.
- `SC-OFEROUTE-001` requires active/activation-candidate paths to consume
  source-authorized operands or fail closed, and rejects missing-source
  all-lane defaults.
- D11 evidence found no WEPP-runtime source/default mapping for `k_o`, form
  `C_d`, `D_r`, or `lambda`, and rejected residue/random-roughness/Chapter-10
  inference as surrogate physics.

## Candidate Bridge Surfaces

| Candidate source | Audit result | Reason |
|---|---|---|
| Native `ow-lanuse-1` `routing_coefficients` | Accepted authority class, absent in selected roots | This is the ratified static operand source, but the current source-authored input audit found zero native datvers and zero coefficient markers. |
| Operator/source-authored sidecars | Accepted authority class, absent in session | No sidecar or active `*.run.toml` route-coefficient file was found in the selected roots. |
| Legacy row width / ridge spacing / `rrinit` / random roughness | Rejected | `LANUSE-AUTH-3` explicitly forbids this inference without a ratified bridge; D11 found no direct Papanicolaou operand mapping. |
| Residue depth or residue-cover surfaces | Rejected | D11 recorded that residue/roughness candidates do not define `D_r` or `lambda`; using them would be surrogate physics. |
| Chapter-10 hydraulics or friction terms | Rejected | No contract maps those symbols to the five route coefficients; D11 rejected this as a bridge source. |
| Canopy cover, LAI, canopy height | Rejected for static coefficients | Rev-21 already sources dynamic `LAI` and `h_c`, but those do not authorize static `k_o`, form `C_d`, `D_r`, `lambda`, or vegetation `C_d`. |
| H2637 timing patch recipe `500.0 0.0 0.0 0.0 0.0` | Rejected for cohort bridge | Useful as H2637 test/timing scaffolding only; `SC-OFEROUTE-001` rejects missing-source all-lane defaults for activation-candidate/default claims. |
| D-val constants | Rejected | D11 identifies them as fixture values, not runtime defaults or broad production-cohort authority. |

## Current Runtime Guard

Ran:

```text
cargo test -q --test laned_shadow_h2637 h2637_active_fails_closed_without_routing_coefficients

running 1 test
.
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.25s
```

Interpretation: the active path still fails closed when the static
route-coefficient authority surface is missing.

## Result

No safe bridge can be authored from the current repo/session evidence. The
only accepted route-coefficient source class is native/operator-authored input,
and that input is absent. A bridge would require new canonical authority that
names source fields and maps all five operands; current package evidence does
not contain that authority.
