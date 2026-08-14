# Ground Heat And Soil Boundary

The thermal column uses CLM5 equations (2.6.1)--(2.6.49) over the caller's
actual openWEPP soil-layer geometry rather than a synthesized 25-layer grid.
Conductivity and heat capacity use equations (2.6.75)--(2.6.91). The lower
boundary is zero heat flux. Frozen or phase-changing layers are typed
unsupported in V1.

For litter, ISBA-MEB equation (A10) is selected:

```text
G_l,1 = (T_l - T_1) / (dz_l/lambda_l + dz_1/lambda_1)
```

with equation (A13) conductivity and (A14) heat capacity. Define the
constitutive `G_down=G_l,1` as positive from litter into soil. Under the
package-wide positive-into-control-volume convention,
`G_surface=-G_down` and the soil-thermal receipt is `G_soil=+G_down`. The
energy owner emits this one joined surface/soil pair; the soil-thermal owner
never recomputes another `G`.

For the finite-capacity surface branch, the Crank--Nicolson beginning
surface-side temperature is derived from authoritative beginning enthalpy,
water mass, and dry capacity. For the equilibrium-zero branch there is no
physical beginning surface temperature: the current algebraic trial surface
temperature is used on the surface side of both top-interface endpoints,
while the soil side retains its distinct beginning and trial-ending
temperatures. A caller warm start remains numerical-only and cannot become a
beginning energy operand.
