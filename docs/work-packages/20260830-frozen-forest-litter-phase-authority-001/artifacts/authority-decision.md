# Authority decision

Status: `AUTHORITY FROZEN — CONTRACT EDIT AUTHORIZED`

Evidence mode: `Static`

Retained authority:

- R-156 peer-reviewed ISBA-MEB litter model:
  `references/vendorable/gmd-10-1621-2017-isba-meb-litter.pdf`, SHA-256
  `2a8c14d912651457bf9205a4a963b78dd12f1aa7f243bccb025e4b81ce99716d`;
- official SURFEX v8 `isba_meb.F90` generated source, SHA-256
  `0a300739b5dc660b61d29db144dd92f886e8fdf9934eac8facc022585992087a`;
- official SURFEX v8 `isba_fluxes_meb.F90` generated source, SHA-256
  `e0378bc89ee0d52cffe14841aac56de1d8d379edf18ad29f24cfdb9ea0dfdbbc`;
- official SURFEX v8 `ini_csts.F90` generated source, SHA-256
  `f39840df4d851efc70044f9e3ad62822371ed743c3c3a8055a4c940e2f86d73a`;
- CeCILL-C v1 English license, SHA-256
  `7280115e43fa03917f2f23370519be8c9fb0b57f4c86f8da5f7ac10c070f6aa0`.

Scientific selection:

- bounded kinetic phase transfer, not instantaneous complementarity;
- `T_ref=273.15 K`, `rho_i=920 kg m^-3`, and `C_i=2106 J kg^-1 K^-1`
  from R-156; `tau_ice=3300 s`, `L_f=333700 J kg^-1`, executable ordering,
  and ice capacity `0.85 m3 m^-3` from the named SURFEX v8 instantiation.
  Because the source converts both `PWRL` and `PWRLI` to volumetric values
  using `rho_w`, the areal ice-capacity operand is exactly
  `0.85*rho_w*litter_depth`; it is not `0.85*rho_i*litter_depth`;
- R-156 printed A4 has a sign inconsistency against A1-A3 and its melt prose.
  Conservation plus official implementation selects
  `signed_phase = freeze - melt`: positive phase debits liquid, credits ice,
  and credits fusion energy; negative phase reverses those identities;
- use a conservative enthalpy-coordinate realization. From phase-free
  `U*`,`T*`, compute bounded phase mass; transfer equal liquid/ice mass; set
  `U_end=U*+L_f*m_freeze-L_f*m_melt`; then derive temperature from the ending
  dry/liquid/ice heat capacity. Literal SURFEX `T += Q/C_old` is a rejected
  wrong formula because changing phase heat capacity otherwise creates an
  unowned `delta-C*(T-T_ref)` term;
- retain phase-specific liquid evaporation/condensation with `L_v` and ice
  sublimation/deposition with `L_s`. R-156 A7-A9 and the retained flux source
  use liquid-water saturation humidity for both litter components; saturation
  over ice is explicitly not introduced. The exact empty-pool frozen fraction
  is zero; otherwise it is `W_i/(W_l+W_i)`. The SURFEX `xwgmin` denominator
  regularization is rejected as a hidden floor. Each outbound component is
  separately capped by its named beginning-phase availability; each inbound
  component credits only its named phase and has no availability cap;
- phase-specific vapor carries exact surface enthalpy: the liquid operand is
  `C_w*(T-T_ref)+L_v(T)` and the ice operand is
  `C_i*(T-T_ref)+L_s(T)`. Signed outbound/inbound mass and energy are recorded
  independently before any total-air-flux aggregation;
- preserve immutable beginning-phase availability for the phase-specific vapor
  authorization; install the finalized liquid evaporation/condensation and ice
  sublimation/deposition debits/credits first; compute bounded kinetic phase
  transfer from `T*` and those exact post-vapor/pre-ingress phase masses; apply
  the atomic equal liquid/ice transfer and fusion-energy update; only then admit
  current ingress and WB14. Vapor and phase therefore share one reconstructable
  beginning lineage without double debit, while same-interval ingress cannot
  donate to either process.

Identity, migration, and chronology selection:

- LSE V3 and surface-owner/restart V2 are immutable successors. V1/V2 LSE,
  surface-owner V1, legacy complete-owner projection V2, and their canonical
  bytes remain unchanged;
- a checked V1-to-V2 surface-owner migration initializes litter ice to exact
  zero. A new V2 seed may supply explicit finite nonnegative ice. Temperature
  never implies or synthesizes day-zero ice;
- production V2-to-V1 downgrade is prohibited. A test-only proof utility may
  establish representability when ice is exactly zero, but it is not a runtime
  fallback;
- phase-adjusted temperature is the ending state and next-support warm start.
  Phase transfer never triggers a same-support flux or fixed-point re-solve;
- the successor model definition binds terminal contract digests, parent
  identity, exact retained-source hashes, constants, formulas, ordering, and
  refusals. Restart/checkpoint bytes carry explicit successor tags and reject
  absent, stale, or mismatched identities fail-closed;
- `SC-EVAP-001` remains the daily WB17 ET owner. This distinct subdaily
  surface-vapor transaction is owned by SC-LSE v14, precedes current ingress
  and WB14, and neither mutates soil nor contributes litter ice to `frozwt`.

Explicitly refused: SURFEX `zertol` tiny-ice deletion, soil compensation,
instantaneous projection, lower-bound patch, snow/soil/bare frozen-state
admission, ice as WB14 liquid supply, current-ingress donation, freeze-only
logic, saturation-over-ice substitution, hidden empty-pool regularization,
same-support phase re-solve, implicit ice initialization, production downgrade,
tolerance/floor changes, and producer-residual closure.

Implementation requires new immutable LSE V3 and versioned surface-owner
state/restart identities; V1/V2 bytes and definitions remain frozen.
