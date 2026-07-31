# Implementation Evidence

Status: `complete / pass`

Evidence mode: `Static + Ran`

The production Stage 3 provider now:

- projects persistent depositional layers into an upper
  `min(total depth, 0.25 m)` thermal control volume and a distinct lower
  control volume;
- retains one shared temperature within each control volume while preserving
  the active/lower boundary across substeps;
- evaluates surface radiation, vapor/latent exchange, and active/lower
  conduction from the same pre-substep state;
- uses the exact libsnobal `KTS+efcon` effective snow conductivity, including
  elevation-derived atmospheric pressure;
- schedules `3,600`, `900`, or `60 s` substeps at the contracted
  `60/10/1 kg m^-2` active-mass thresholds;
- exports signed requested, applied, and rejected `G_0` with the exact
  same-substep reconstruction operands; and
- rejects inadmissible temperatures through typed guards without an
  absolute-zero clamp, air-temperature replacement, fitted limiter, or new
  user coefficient.

Ran: 20 focused contract/runtime/legacy compatibility tests passed. Ran:
22 `openwepp-meteorology` tests passed, including the exact numerical
libsnobal conductivity reference. Ran: the six-cell real direct-production
consumer completed with `PASS`.
