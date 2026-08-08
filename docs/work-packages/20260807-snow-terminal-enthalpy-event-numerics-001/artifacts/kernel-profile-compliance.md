# Kernel Profile Compliance

Status: compliant and terminally verified

Evidence mode: Static + Ran

- Contract-first amendments and derived tests passed before production edits.
- Positive cold-content deficit and `H = -Q_cc + L_f m_l` are retained.
- Endpoint solid includes refreeze and deposition; melt availability excludes
  them.
- Step doubling uses scaled mass/energy error control; event localization uses
  safeguarded bisection and explicit rejection limits.
- Typed request/state mismatch and numerical nonconvergence fail closed.
- Snow-domain mass, liquid, energy, and time are independently reconstructed.
- No receiving-surface continuation or provisional process physics was added.
