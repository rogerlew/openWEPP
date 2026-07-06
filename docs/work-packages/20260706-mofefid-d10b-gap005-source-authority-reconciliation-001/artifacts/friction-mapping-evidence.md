# Friction Mapping Evidence (D10B-S2)

Status: executed (binding recorded rev 24)
Evidence mode: Static

## The mapping

Iwagaki 1955 analyzes experiment (B) with a Manning-type resistance,
`n = 0.009` (m-s units) — extracted verbatim from the primary
(`source-acquisition-record.md`). The Lane D solver's kinematic form is
`q = alpha h^1.5` with `alpha = sqrt(8 g / f_eq) sqrt(S_o)`. The
definitional identity between the Manning and Darcy-Weisbach laws is

    V = (1/n) R^(2/3) S^(1/2)  and  V = sqrt(8 g R S / f)
    =>  f = 8 g n^2 / R^(1/3)   (wide channel: R = h)

so a Manning cell is expressed through the existing `alpha` machinery as
`f_eq(h) = 8 g n^2 / h^(1/3)`, giving `alpha = (sqrt(S_o)/n) h^(1/6)` and
`q = (sqrt(S_o)/n) h^(5/3)` — Manning exactly. This is a definitional
unit-consistent identity (any standard hydraulics reference states it;
in-library: HEC-RAS Hydraulic Reference Manual, `references/vendorable/`),
not a tuned or fitted mapping.

## Units

`n` in m-s units (s·m^(-1/3)) per the primary's explicit statement;
`g = 9.81 m/s^2`; `h` in m; resulting `f` dimensionless; `alpha` in
m^(1/2)/s only for the m = 1.5 form — for the Manning limb `alpha`
carries `h^(1/6)` and is evaluated per cell per step by the existing
implicit-alpha machinery (converges in one iteration; no `Re`
dependence).

## Boundaries (explicit)

- This mapping enters ONLY the Case-4 D-val configuration and the oracle
  (`INV-OFEROUTE-011` rev 24). It does NOT touch the D11 rev-20/21
  production operand path (`routing_coefficients`, live `I`/`LAI`/`h_c`),
  which remains the sole production friction authority.
- `k_o` is REMOVED from Case-4 acceptance (it was never specified by the
  primary); it remains a D11-owned operand for production lanes and the
  other D-val cases.
- No default, production, or activation semantics change with this
  mapping.
