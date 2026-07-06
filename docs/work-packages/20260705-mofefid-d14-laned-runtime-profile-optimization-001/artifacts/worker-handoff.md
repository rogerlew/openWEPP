# Worker Handoff

Status: **COMPLETE — handoff to D15 (opt-in production activation).**

## D14 result

The Lane D runtime cost is now empirical and substantially reduced, with the
candidate path's behavior untouched:

- Release-grade H2637 (2-year fixture, 19 OFEs, 622 routed days):
  default/off **2.3 s**, shadow-on **29.8 s** (was 67.6 s), shadow overhead
  **+27.5 s user CPU** (was +65.3 s; −58 %). Shadow multiplier over the
  default path: **~13×** (was ~29×).
- The overhead is ~97 % solver math by construction of the path: 10.3 M
  TVD-MacCormack steps per H2637 run (875 avg per OFE-day at the current
  shadow resolution: 10 cells/OFE, `sample_dt=900 s`, `max_dt=300 s`,
  routing window clipped to the active source span + 6 h).
- Persistent opt-in slot diagnostics: `OPENWEPP_LANED_SHADOW_PROFILE=1`
  emits one stderr `laned_shadow_profile {…}` JSON line (runner slots +
  `ofe_routing::profile` solver slots/counters). Local-CI discoverable;
  no GitHub CI required.

## D15 runtime budget (explicit)

- **Budget anchor:** on H2637-class hillslopes, opt-in ACTIVE routing should
  be expected to cost on the order of the measured shadow: **~+27.5 s user
  CPU per 2 climate years** (~+13.8 s per climate year) on 2.7 GHz
  Ivy-Bridge-class hardware, single-threaded, at the current shadow
  resolution. Scaling to the full ~50-year H2637 record: roughly
  **+11–12 min CPU** per hillslope — acceptable only as **opt-in**; this
  reconfirms the strategy's decision that default promotion (D16) needs its
  own endpoint evidence.
- The remaining cost is dominated by the physics (fixed-point friction
  resolution + `h^1.5` discharge updates over CFL-bounded substeps).
  Bit-identity-safe headroom left on the table is small: stateful
  upstream-interpolation cursors and celerity caching (~1–3 % each,
  recorded in `optimization-disposition.md`).
- **Material further reduction requires D10-class decisions** (mesh
  resolution, sub-timestep policy, sampling density, solver method), which
  are `GAP-OFEROUTE-005` source-authority surfaces — not performance knobs.
  D14 changed none of them.
- **Refresh rule (binding, from strategy §6.1):** if D10 or any later
  correction materially changes the activation candidate's solver
  resolution, friction operands, source shape, or handoff policy, D14's
  endpoint timing must be refreshed before D15 activation claims.

## Remaining activation risks for D15 (unchanged by D14)

1. `GAP-OFEROUTE-005` / `INV-OFEROUTE-011` Case-4 shock-numerics
   source-authority hold (D10, `EXECUTED-HOLD-SOURCE-AUTHORITY`) — including
   its real-H2637 resolution-sensitivity reproduction (the shadow's
   aggregate conservation diagnostic is resolution-sensitive and
   dt-non-monotone; currently 8.24 % at the shadow operating point, inside
   the <15 % diagnostic bound).
2. `INV-OFEROUTE-012` activation wiring: routing owning the water path,
   runtime closure hard-fail, DC01 disable, `latqcc` bypass in closure,
   routed-path publication.
3. Production-consumer proof obligations: the active path must read the
   rev-21 friction operands, the D12 melt limb, and supply
   `routed_hydrograph_runoff_fraction` to the D13 erosion consumer, with
   default/off byte-flat.
4. The 6 `days_uniform_shape` no-authorized-source-shape residual days
   remain diagnostic-only and cannot carry activation evidence.

## Explicit non-claims

- No production/default Lane D activation; no D15 selector or DC01 disable.
- No D10 shock-numerics correction, Case-4 acceptance, or tolerance change.
- No D11/D12/D13 semantic change (rev-21 operand path, melt limb, and
  routed-hydrograph erosion shape untouched).
- No solver-method, resolution, CFL-target, or fixed-point-iteration change;
  no surrogate/proxy physics.
- No HBP/pass schema or manifest-value change (the manifest `laned_shadow`
  block is bit-identical; the only new output surface is the opt-in stderr
  profile line).
