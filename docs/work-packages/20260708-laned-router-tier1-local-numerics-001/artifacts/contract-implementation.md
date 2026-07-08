# Contract Implementation

Status: `EXECUTED`

`SC-OFEROUTE-001` was amended from rev 46 to rev 47 before the production
implementation landed.

Rev 47 authorizes:

- analytic implicit celerity for Manning, pure laminar skin, pure Hirsch skin,
  and additive friction menus;
- closed-form `alpha/q/c` for Manning and pure skin limbs;
- bounded Newton for additive friction menus with finite derivative guards;
- a `1e-18 m^2 s^-1` dust residual floor for final additive Newton consistency;
- `h^1.5` evaluation as `h * sqrt(h)` for non-negative depth;
- no smoothing at `Re`/`Fr`/submergence branch discontinuities.
- active vegetation local-numerics non-finite math as typed hard failure, not a
  zero-resistance fallback.

Rev 47 explicitly does not authorize a Hirsch `Re^0.45` approximation. The
canonical exact-library `powf` path remains binding until a later bounded-error
envelope is ratified.
