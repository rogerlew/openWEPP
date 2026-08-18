# Hydraulic Feasibility Audit

Status: `HOLD / scalar envelope complete / coupled authority incomplete`

The current executable reconstructs only a scalar hydraulic feasibility
envelope at no fewer than 80 decimal digits. It does **not** reconstruct the
canonical coupled midnight potential or fixed-final branches. Rust output
supplies observed inputs and failure evidence only; it is not an expected
oracle.

The diagnostic executable is `reference_nighttime_hydraulics.py` and uses 100
decimal digits. For an illustrative upper-occupancy envelope it reconstructs:

- `rho = 1.032082853115747... kg m^-3`;
- `gs0 = 7.006714164866917...e-7 m s^-1`;
- `E0 = 5.989918357162202...e-9 kg m^-2 s^-1`;
- accepted fixed authorization `9.7595293578063313e-10 kg m^-2 s^-1`;
- `fhyd = 0.162932593999996...`;
- `psi_leaf = -194149.9449894088... mm`;
- `psi_stem = -194149.9449120328... mm`;
- `psi_root = -181646.9032740586... mm`;
- algebraically recovered `gs = 1.141596660688823...e-7 m s^-1`;
- `beta_hyd = 0.1629289612544807...`;
- `Ci = 27924.94467378438... Pa`, strictly inside `(0, Patm)`.

All three scalar continuity differences are below `1.2e-99` in their
dimensional decimal representations. This proves that the proposed scalar
relationships are mutually consistent at those substituted operands. It does
not prove a root of the canonical two-occupancy canopy-air, energy, soil, and
per-layer-cap system, and it does not classify the intake Newton failure.

The same diagnostic rejects zero authorization and the selected 25%/very-small
authorizations as typed `VEG-E-121` because the conductance required by the
hydraulic attenuation would place `Ci >= Patm`. It returns no approximate
candidate. These results are branch-feasibility evidence, not a claim that
cuticular loss or plant capacitance exists; neither mechanism is admitted.

The first independent review correctly noted that attenuation of `g0` is a
new prospective V10 stomatal-regulation equation rather than an algebraic
consequence of V9 Medlyn. Final authority therefore must say so explicitly,
must dispatch only under validated V10/LSE-V2 identity, and must not alter the
uncapped V9 potential pass or any V3--V9 byte.

## Missing authority evidence

Independent science review requires a replacement coupled oracle before any
production Rust implementation. It must bind full-precision immutable owner
operands and solve both occupancies, wet and stem energy, shared canopy heat
and vapor, ground/soil energy, q1/q2/all q3 laws, and fixed per-layer cap
selection from the same beginning state. Potential requests must be derived
from an uncapped solve; fixed-final branches must use `cap <= law` without
assuming aggregate authorization equals realized flux. Accepted dry,
partially-wet, fully-wet, ordinary/low-g0, full and feasible-partial cases need
two materially different convergent warm starts. Zero/insufficient supply,
inaccessible/frozen roots, impossible topology, condensation, and Ci-domain
failures need exact typed dispositions with no candidate.

Until that artifact passes independent review, V10/LSE-V2 implementation and
release remain blocked and no numerical tolerance or solver change is
authorized.
