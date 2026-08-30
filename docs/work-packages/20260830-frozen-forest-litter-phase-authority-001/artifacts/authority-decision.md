# Authority decision

Status: `AUTHORITY FROZEN — CONTRACT EDIT PENDING`

Evidence mode: `Static`

Retained authority:

- R-156 peer-reviewed ISBA-MEB litter model:
  `references/vendorable/gmd-10-1621-2017-isba-meb-litter.pdf`, SHA-256
  `2a8c14d912651457bf9205a4a963b78dd12f1aa7f243bccb025e4b81ce99716d`;
- official SURFEX v8 `isba_meb.F90` generated source, SHA-256
  `0a300739b5dc660b61d29db144dd92f886e8fdf9934eac8facc022585992087a`;
- official SURFEX v8 `ini_csts.F90` generated source, SHA-256
  `f39840df4d851efc70044f9e3ad62822371ed743c3c3a8055a4c940e2f86d73a`;
- CeCILL-C v1 English license, SHA-256
  `7280115e43fa03917f2f23370519be8c9fb0b57f4c86f8da5f7ac10c070f6aa0`.

Scientific selection:

- bounded kinetic phase transfer, not instantaneous complementarity;
- `T_ref=273.15 K`, `rho_i=920 kg m^-3`, and `C_i=2106 J kg^-1 K^-1`
  from R-156; `tau_ice=3300 s`, `L_f=333700 J kg^-1`, executable ordering,
  and ice capacity `0.85 m3 m^-3` from the named SURFEX v8 instantiation;
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
  sublimation/deposition with `L_s`, separately availability-bounded and
  owner-keyed;
- preserve immutable beginning-phase availability, finalized vapor debit,
  atomic phase adjustment, current-ingress, then WB14 ordering.

Explicitly refused: SURFEX `zertol` tiny-ice deletion, soil compensation,
instantaneous projection, lower-bound patch, snow/soil/bare frozen-state
admission, ice as WB14 liquid supply, current-ingress donation, freeze-only
logic, tolerance/floor changes, and producer-residual closure.

Implementation requires new immutable LSE V3 and versioned surface-owner
state/restart identities; V1/V2 bytes and definitions remain frozen.
