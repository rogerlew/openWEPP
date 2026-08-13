# V3 Potential-Pass Authority Selection

Status: `approved authority complete`

Evidence mode: `Static`

## Resolution of the Prior HOLD

The prior implementation HOLD identified five constitutive/schema gaps and one
evidence gap. `SC-VEGETATION-001@7` now binds the selected resolution:

| Gap | V3 resolution | Canonical binding |
| --- | --- | --- |
| Leaf/stem optics and absorption | Transport over `L+S`, area-weighted optics, `K_eff=Omega*K`, leaf-only sun/shade area, absorptivity-weighted leaf/stem owners | `INV-VEGETATION-080` |
| Local surface wind | neutral `u_star` from reference wind; semantic `u_leaf=u_wet=u_stem=u_star`; dimension-specific conductance | `INV-VEGETATION-081` |
| Stem path and gravity | `z2=height_m`; `Delta_psi=1000*height_m` | `INV-VEGETATION-082` |
| Root warm-start schema | one `root_node_potential_mm`; bitwise-identical-only V2 migration | `INV-VEGETATION-082/083` |
| Potential beta semantics | internal class beta-one maximum only; accepted owner-uncapped six-unknown/six-residual solve with distinct sun/shade beta, both class loss equations, both class flux equalities, and two downstream continuity equations; persisted `beta_hyd` is only the Emax-weighted aggregate/warm start | `INV-VEGETATION-084` |
| Independent evidence | committed Python-generated V3 radiation, potential, migration, respiration, and failure fixtures with named poisons | `INV-VEGETATION-080`--`086` |

The package also resolves the discovered leaf-respiration ambiguity by making
Atkin leaf-N/T10 the only `Rd25` source and the identical class `Rd` both the one
net-assimilation subtraction and the one leaf-maintenance carbon debit.

## Provenance Classification

- `[DIRECT][Static]`: CLM5 exposed plant area uses `L+S`; leaf/stem optical
  properties are area-weighted; incident canopy wind is friction velocity;
  the hydraulic circuit has one common root node and height/gravity terms;
  net assimilation subtracts `Rd` with Rd-specific temperature constants.
- `[INFERENCE][Static] OPENWEPP_CANONICAL_SELECTION`: clumping is applied as
  `K_eff=Omega*K`; solved absorption is returned to physical leaf/stem owners
  by area-times-absorptivity; ambiguous V2 layer warm starts fail rather than
  average; accepted Stage-A potential is hydrology-uncapped but hydraulically
  coupled with distinct CLM class factors and one explicitly selected persisted
  aggregate; numerical diagnostics are canonical typed transaction evidence.

No production Rust or runtime identity is released by this artifact.
