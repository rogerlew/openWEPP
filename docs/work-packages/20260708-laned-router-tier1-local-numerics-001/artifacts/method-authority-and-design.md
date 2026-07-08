# Method Authority and Design

Status: `EXECUTED`

Authority: `SC-OFEROUTE-001` rev 47.

Design summary:

- `c = dq/dh` is computed from
  `q = sqrt(8 g S_o / f_eq(q,h)) h^1.5` using the selected branch
  derivatives.
- Manning uses the closed form `q = sqrt(S_o)/n * h^(5/3)` and
  `c = (5/3) q/h`.
- Pure laminar skin uses the closed form `q proportional h^3` and
  `c = 3 q/h`.
- Pure Hirsch skin uses the closed form
  `q proportional h^(1.5 / 0.775)` and `c = (1.5/0.775) q/h`.
- Pure-skin discontinuity gaps choose the pre-step Reynolds branch with no
  smoothing.
- Additive menus compute `df/dq` and `df/dh` for active components and solve
  with bounded log-Newton.
- Active vegetation derivative failures in local numerics are invalid state,
  not absent-canopy zeroes.
- `h.powf(1.5)` in the hot KWE path is replaced with `h * h.sqrt()`.

The `Re^0.45` approximation candidate is not landed. No bounded minimax or
table/vector envelope was available inside this package, and rev 47 keeps the
canonical exact-library evaluation binding.
