# Canopy--Ground Turbulent Coupling

Open tiles use the admitted neutral reference-air resistance directly. Covered
tiles use one shared tile canopy-air temperature and humidity. Every occupancy
component exchanges with that shared node using the unchanged V7 component
conductances; the ground/litter exchanges through `r_g-c` from ISBA-MEB/CLM5.
The shared zero-storage nodes satisfy

```text
sum(H_vegetation) + H_ground - H_to_atmosphere = 0
sum(E_vegetation) + E_ground - E_to_atmosphere = 0.
```

This successor topology replaces V7's independent occupancy canopy-air nodes.
Ground heat/vapor enters once, at the shared tile node; it is never injected
into each occupancy. Positive vapor flux is evaporation to air and negative is
condensation to the surface owner.

Neutral reference and under-canopy resistances require positive finite wind,
valid heights/roughness/displacement, and positive conductance. Calm and
nonneutral conditions return typed unsupported failures. No minimum wind or
stability-neutralizing clamp is admitted.
