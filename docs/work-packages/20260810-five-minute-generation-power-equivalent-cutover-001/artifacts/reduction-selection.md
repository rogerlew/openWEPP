# Reduction Selection

Status: `complete — structural reduction identified, no production selection`

Evidence mode: `Static + Ran`

Consumer semantics reject the fixed-hour power mean because the real solver
requires representative rate times `effdrn_s` to reconstruct hourly runoff
depth and uses that duration in load normalization/denormalization.

For any future authorized exponent `p>1`, the only structurally admissible
form is:

    g_eq = (E_p / V)^(1/(p-1))
    d_eq = V / g_eq

It reconstructs both volume and the selected power moment exactly and would
require a typed split between local erosion hydraulics and arithmetic-mean
water/inter-OFE discharge. The current study admits no exponent, so this form
is recorded as the rejected-candidate architecture only. It is not selected
for diagnostic shadow execution or production.

Raw five-minute maximum remains diagnostic-only and is not a candidate.
